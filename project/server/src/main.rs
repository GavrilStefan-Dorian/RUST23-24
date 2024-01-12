use clap::Parser;
use rsa::pkcs8::DecodePublicKey;
use rsa::{pkcs8::EncodePublicKey, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rusqlite::Connection;
use std::collections::HashMap;
use std::error;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Username {
    name: String,
}

#[derive(Debug)]
struct Message {
    sender: String,
    message: String,
}
#[derive(Debug)]
struct MessageWithReply {
    sender: String,
    message: String,
    reply_to: String,
}

#[derive(Parser)]
#[command(
    version,
    about = "cargo run [-- --IP -- PORT] \n\n List of commands:\n\t1.login <username> <password>\n\t2.start_chat <recipient>\n\t3.end_chat\n\t4.send_message <message>\n\t5.history\n\t6.reply_to <message_index> <message>\n\t7.logout\n\t8.help\n"
)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    ip: String,

    #[arg(long, default_value = "7878")]
    port: String,
}

#[derive(Debug)]
struct User {
    stream: TcpStream,
    chat_partner: String,
    public_key: RsaPublicKey,
}

#[derive(Debug)]
struct UsersOnline {
    map: HashMap<String, User>,
}

impl UsersOnline {
    fn new() -> Self {
        UsersOnline {
            map: HashMap::<String, User>::new(),
        }
    }

    fn insert(&mut self, username: String, user: User) {
        self.map.insert(username, user);
    }

    fn get(&self, username: &str) -> Option<&User> {
        self.map.get(username)
    }

    fn get_mut(&mut self, username: &str) -> Option<&mut User> {
        self.map.get_mut(username)
    }

    fn remove(&mut self, username: &str) {
        self.map.remove(username);
    }
}

fn db_create_if_not_exists() -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open("server_db.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users_registry (
                name TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL
                )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS message_history (
                name TEXT NOT NULL,
                sender TEXT NOT NULL,
                message TEXT NOT NULL,
                reply_to TEXT NOT NULL,
                read INTEGER NOT NULL
                )",
        [],
    )?;

    //      conn.execute(
    //     "INSERT INTO users_registry (name, password) VALUES (?1, ?2)",
    //     ["Stefan", "pass1"],
    // )?;
    // conn.execute(
    //     "INSERT INTO users_registry (name, password) VALUES (?1, ?2)",
    //     ["Tudor", "pass2"],
    // )?;
    // conn.execute(
    //     "INSERT INTO users_registry (name, password) VALUES (?1, ?2)",
    //     ["Dorian", "pass3"],
    // )?;
    // conn.execute(
    //     "INSERT INTO users_registry (name, password) VALUES (?1, ?2)",
    //     ["User D","pass4"],
    // )?;
    // conn.execute(
    //     "INSERT INTO users_registry (name, password) VALUES (?1, ?2)",
    //     ["User E", "pass5"],
    // )?;

    Ok(())
}

fn write_encrypted(
    public_key: &RsaPublicKey,
    mut stream: &TcpStream,
    message: String,
) -> Result<(), Box<dyn error::Error>> {
    let mut rng = rand::thread_rng();
    let enc_message = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, message.as_bytes())
        .expect("Failed to encrypt");

    stream.write_all(&enc_message.len().to_be_bytes())?;
    stream.write_all(enc_message.as_slice())?;

    Ok(())
}

fn handle_sender(
    stream: TcpStream,
    shared_map: Arc<Mutex<UsersOnline>>,
    server_private_key: RsaPrivateKey,
) -> Result<(), Box<dyn error::Error>> {
    let mut logged = false;

    let mut username = String::new();
    let stream_clone = stream.try_clone().expect("failed to clone");
    let mut reader = BufReader::new(stream_clone);

    let mut size_bytes = [0; 8];
    reader.read_exact(&mut size_bytes)?;
    let client_pem_size = usize::from_be_bytes(size_bytes);
    let mut client_pem_bytes = vec![0; client_pem_size];
    reader.read_exact(&mut client_pem_bytes)?;

    let mut pem = String::new();
    for byte in client_pem_bytes {
        pem.push(byte as char);
    }

    let public_key_client = RsaPublicKey::from_public_key_pem(&pem)?;

    loop {
        let mut size_bytes = [0; 8];
        reader.read_exact(&mut size_bytes)?;

        let message_size = usize::from_be_bytes(size_bytes);

        let mut message = vec![0; message_size];
        reader.read_exact(&mut message)?;

        let dec_message = server_private_key.decrypt(Pkcs1v15Encrypt, &message)?;
        let message = String::from_utf8(dec_message)?;
        println!("from the sender: {}", message);

        if message.starts_with("login") {
            if let Some(lsplit) = message.split_once(' ') {
                if logged {
                    write_encrypted(
                        &public_key_client,
                        &stream,
                        String::from("Already logged in!"),
                    )?;
                }

                if !logged {
                    let lsplit_2 = lsplit.1.split_once(' ').unwrap_or(("none", "none"));
                    if lsplit_2 == ("none", "none") {
                        write_encrypted(
                            &public_key_client,
                            &stream,
                            "Invalid format.".to_string(),
                        )?;
                    }

                    let conn = Connection::open("server_db.db")?;
                    let mut stmt = conn.prepare(
                        "SELECT name FROM users_registry where name = ?1 AND password = ?2",
                    )?;
                    let user_iter = stmt
                        .query_map([lsplit_2.0.trim(), lsplit_2.1.trim()], |row| {
                            Ok(Username { name: row.get(0)? })
                        })?;

                    let mut stream_map = shared_map.lock().unwrap();

                    for user in user_iter {
                        username = user.unwrap().name;
                        println!("Found username {:?}", username);
                        write_encrypted(
                            &public_key_client,
                            &stream,
                            format!("Logged as {}!", username),
                        )?;
                        let mut stmt =
                        conn.prepare("SELECT sender,message,reply_to FROM message_history WHERE read = 0 AND sender != ?1")?;
                        let msgs = stmt.query_map([username.clone()], |row| {
                            Ok(MessageWithReply {
                                sender: row.get(0)?,
                                message: row.get(1)?,
                                reply_to: row.get(2)?,
                            })
                        })?;

                        let mut message_list = String::new();
                        let mut unread = false;
                        let mut recipient = String::new();
                        message_list.push_str("You've got unread messages!\n");

                        for msg in msgs {
                            let pair = msg.unwrap();
                            if pair.reply_to.is_empty() {
                                message_list.push_str(&format!(
                                    "sent by {}: {}",
                                    pair.sender, pair.message
                                ));
                            } else {
                                message_list.push_str(&format!(
                                    "reply from {}\n>\"{}\"\n{}\n",
                                    pair.sender, pair.reply_to, pair.message
                                ));
                            }
                            if !unread {
                                recipient = pair.sender;
                                unread = true;
                            }
                        }
                        if unread {
                            write_encrypted(
                                &public_key_client,
                                &stream,
                                format!("{}!", message_list),
                            )?;
                            conn.execute(
                                "UPDATE message_history SET read = 1 WHERE sender != ?1 AND name = ?2",
                             [&username, &username],
                                )?;

                            if let Some(pair) = stream_map.get(&recipient) {
                                if pair.chat_partner != "none" {
                                    stream_map.insert(
                                        username.clone(),
                                        User {
                                            stream: stream.try_clone().expect("clone failed..."),
                                            chat_partner: recipient,
                                            public_key: public_key_client.clone(),
                                        },
                                    );
                                } else {
                                    stream_map.insert(
                                        username.clone(),
                                        User {
                                            stream: stream.try_clone().expect("clone failed..."),
                                            chat_partner: "none".to_string(),
                                            public_key: public_key_client.clone(),
                                        },
                                    );
                                }
                            } else {
                                stream_map.insert(
                                    username.clone(),
                                    User {
                                        stream: stream.try_clone().expect("clone failed..."),
                                        chat_partner: "none".to_string(),
                                        public_key: public_key_client.clone(),
                                    },
                                );
                            }
                        } else {
                            write_encrypted(
                                &public_key_client,
                                &stream,
                                String::from("No unread messages!"),
                            )?;
                            stream_map.insert(
                                username.clone(),
                                User {
                                    stream: stream.try_clone().expect("clone failed..."),
                                    chat_partner: "none".to_string(),
                                    public_key: public_key_client.clone(),
                                },
                            );
                        }

                        logged = true;
                    }

                    if !logged {
                        write_encrypted(
                            &public_key_client,
                            &stream,
                            String::from("Username not found!"),
                        )?;
                    }
                }
            }
        } else if message.starts_with("start_chat") {
            if let Some(lsplit) = message.split_once(' ') {
                if !logged {
                    write_encrypted(
                        &public_key_client,
                        &stream,
                        String::from("Log in to use this command!"),
                    )?;
                }

                if logged {
                    if lsplit.1 == username {
                        write_encrypted(
                            &public_key_client,
                            &stream,
                            String::from("You can't open a chat with yourself!"),
                        )?;
                    } else {
                        let mut found = false;
                        let conn = Connection::open("server_db.db")?;
                        let mut stmt =
                            conn.prepare("SELECT name FROM users_registry where name = ?1")?;
                        let name_iter = stmt.query_map([lsplit.1.trim()], |row| {
                            Ok(Username { name: row.get(0)? })
                        })?;

                        for name in name_iter {
                            let recipient = name.unwrap().name.clone();
                            println!("Found username {}", recipient);
                            found = true;
                        }

                        if found {
                            let mut stream_map: std::sync::MutexGuard<'_, UsersOnline> =
                                shared_map.lock().unwrap();

                            stream_map.get_mut(&username).unwrap().chat_partner =
                                lsplit.1.trim().to_string();

                            if let Some(pair) = stream_map.get_mut(lsplit.1.trim()) {
                                if pair.chat_partner != "none" {
                                    write_encrypted(
                                        &public_key_client,
                                        &stream,
                                        format!(
                                            "{} is currently in a different conversation!",
                                            lsplit.1.trim()
                                        ),
                                    )?;
                                } else {
                                    write_encrypted(
                                        &public_key_client,
                                        &stream,
                                        "Chat started".to_string(),
                                    )?;

                                    write_encrypted(
                                        &pair.public_key,
                                        &pair.stream,
                                        format!("You're now chatting with {}", username.clone()),
                                    )?;

                                    pair.chat_partner = username.clone();
                                }
                            } else {
                                write_encrypted(&public_key_client, &stream, "Recipient is offline. They will receive your messages when they go online".to_string())?;
                            }
                        } else {
                            write_encrypted(
                                &public_key_client,
                                &stream,
                                "Recipient not found!".to_string(),
                            )?;
                        }
                    }
                }
            }
        } else if message.starts_with("send_message") {
            if let Some(lsplit) = message.split_once(' ') {
                if !logged {
                    write_encrypted(
                        &public_key_client,
                        &stream,
                        "Log in to use this command!".to_string(),
                    )?;
                }

                if logged {
                    let stream_map = shared_map.lock().unwrap();
                    if let Some(pair) = stream_map.get(&username) {
                        if pair.chat_partner == "none" {
                            write_encrypted(
                                &public_key_client,
                                &stream,
                                "You're not in a conversation!".to_string(),
                            )?;
                        } else {
                            let conn = Connection::open("server_db.db")?;

                            if let Some(pair_2) = stream_map.get(&pair.chat_partner) {
                                write_encrypted(
                                    &pair_2.public_key,
                                    &pair_2.stream,
                                    format!("Received a message:\n{}\n", lsplit.1.trim()),
                                )?;

                                write_encrypted(
                                    &public_key_client,
                                    &stream,
                                    "Message sent!\n".to_string(),
                                )?;

                                conn.execute(
                                     "INSERT INTO message_history (name,sender,message,reply_to,read) VALUES (?1,?2,?3,?4,?5)",
                                     [pair.chat_partner.clone(), username.clone(),lsplit.1.trim().to_string(),"".to_string(),1.to_string()],
                                 )?;
                            } else {
                                conn.execute(
                                    "INSERT INTO message_history (name,sender,message,reply_to,read) VALUES (?1,?2,?3,?4,?5)",
                                    [pair.chat_partner.clone(), username.clone(),lsplit.1.trim().to_string(),"".to_string(),0.to_string()],
                                )?;
                                write_encrypted(
                                    &public_key_client,
                                    &stream,
                                    "Message sent! User will see them when back online.\n"
                                        .to_string(),
                                )?;
                            }
                        }
                    }
                }
            }
        } else if message.starts_with("reply_to") {
            if let Some(lsplit) = message.split_once(' ') {
                if !logged {
                    write_encrypted(
                        &public_key_client,
                        &stream,
                        "Log in to use this command!\n".to_string(),
                    )?;
                }

                if logged {
                    if let Some(lsplit_2) = lsplit.1.split_once(' ') {
                        let stream_map = shared_map.lock().unwrap();
                        if let Some(pair) = stream_map.get(&username) {
                            if pair.chat_partner == "none" {
                                write_encrypted(
                                    &public_key_client,
                                    &stream,
                                    String::from("You're not in a conversation!"),
                                )?;
                            } else {
                                let conn = Connection::open("server_db.db")?;

                                let mut stmt = conn.prepare("SELECT * FROM (SELECT sender,message,row_number() OVER() AS row_id FROM message_history WHERE sender = ?1 AND name = ?2) WHERE row_id = ?3 + 0")?;
                                let msgs = stmt.query_map(
                                    [
                                        pair.chat_partner.clone(),
                                        username.clone(),
                                        lsplit_2.0.to_string(),
                                    ],
                                    |row| {
                                        Ok(Message {
                                            sender: row.get(0)?,
                                            message: row.get(1)?,
                                        })
                                    },
                                )?;

                                let mut reply = String::new();
                                let mut replied_message = String::new();

                                for msg in msgs {
                                    let contents: Message = msg.unwrap();
                                    replied_message.push_str(&contents.message);
                                    reply.push_str(&format!(
                                        "reply by {} to message \n>\"{}\"\n{}",
                                        contents.sender,
                                        contents.message,
                                        lsplit_2.1.trim()
                                    ));
                                }

                                if let Some(pair_2) = stream_map.get(&pair.chat_partner) {
                                    write_encrypted(
                                        &pair_2.public_key,
                                        &pair_2.stream,
                                        format!("{}\n", reply),
                                    )?;
                                    write_encrypted(
                                        &public_key_client,
                                        &stream,
                                        "Reply sent!\n".to_string(),
                                    )?;

                                    conn.execute(
                                     "INSERT INTO message_history (name,sender,message,reply_to,read) VALUES (?1,?2,?3,?4,?5)",
                                     [pair.chat_partner.clone(), username.clone(),lsplit_2.1.trim().to_string(),replied_message.clone(),1.to_string()],
                                 )?;
                                } else {
                                    conn.execute(
                                    "INSERT INTO message_history (name,sender,message,reply_to,read) VALUES (?1,?2,?3,?4,?5)",
                                    [pair.chat_partner.clone(), username.clone(),lsplit_2.1.trim().to_string(),replied_message.clone(),0.to_string()],
                                )?;
                                    write_encrypted(
                                        &public_key_client,
                                        &stream,
                                        "Reply sent! User will see them when back online.\n"
                                            .to_string(),
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
        } else if message.starts_with("end_chat") {
            if !logged {
                write_encrypted(
                    &public_key_client,
                    &stream,
                    "Log in to use this command!\n".to_string(),
                )?;
            }

            if logged {
                let mut stream_map = shared_map.lock().unwrap();
                let recipient = stream_map.get(&username).unwrap().chat_partner.clone();

                if recipient == "none" {
                    write_encrypted(
                        &public_key_client,
                        &stream,
                        "You're not in a conversation!\n".to_string(),
                    )?;
                } else {
                    write_encrypted(&public_key_client, &stream, "Chat finished!\n".to_string())?;
                    stream_map.get_mut(&username.clone()).unwrap().chat_partner =
                        "none".to_string();

                    let pair = stream_map.get_mut(&recipient).unwrap();
                    write_encrypted(
                        &pair.public_key,
                        &pair.stream,
                        String::from("Chat finished!\n"),
                    )?;

                    pair.chat_partner = "none".to_string();
                }
            }
        } else if message.starts_with("history") {
            if !logged {
                write_encrypted(
                    &public_key_client,
                    &stream,
                    "Log in to use this command!\n".to_string(),
                )?;
            }

            if logged {
                let conn = Connection::open("server_db.db")?;
                let mut stmt = conn.prepare(
                    "SELECT sender,message FROM message_history WHERE read = 1 AND sender != ?1",
                )?;
                let msgs = stmt.query_map([username.clone()], |row| {
                    Ok(Message {
                        sender: row.get(0)?,
                        message: row.get(1)?,
                    })
                })?;

                let mut message_list = String::new();
                for msg in msgs {
                    let pair: Message = msg.unwrap();
                    message_list.push_str(&format!("sent by {}: {}\n", pair.sender, pair.message));
                }
                write_encrypted(&public_key_client, &stream, format!("{}\n", message_list))?;
            }
        } else if message.starts_with("logout") {
            if !logged {
                write_encrypted(
                    &public_key_client,
                    &stream,
                    "Log in to use this command!\n".to_string(),
                )?;
            }

            if logged {
                let mut stream_map: std::sync::MutexGuard<'_, UsersOnline> =
                    shared_map.lock().unwrap();

                if stream_map.get(&username.clone()).unwrap().chat_partner == "none" {
                    write_encrypted(&public_key_client, &stream, "Logged out!\n".to_string())?;

                    stream_map.remove(&username.clone());
                    return Ok(());
                } else {
                    write_encrypted(
                        &public_key_client,
                        &stream,
                        "Finish chatting before logging out!\n".to_string(),
                    )?;
                }
            }
        } else if message.starts_with("help") {
            write_encrypted(&public_key_client, &stream, "List of commands:\n\t1.login <username> <password>\n\t2.start_chat <recipient>\n\t3.end_chat\n\t4.send_message <message>\n\t5.history\n\t6.reply_to <message_index> <message>\n\t7.logout\n\t8.help\n".to_string())?;
        } else {
            write_encrypted(
                &public_key_client,
                &stream,
                "Invalid command. Consider checking the list with 'help'\n".to_string(),
            )?;
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    db_create_if_not_exists()?;

    let args = Args::parse();
    let addr = format!("{}:{}", args.ip, args.port);
    let receiver_listener = TcpListener::bind(addr).expect("Failed to bind");

    let shared_map = Arc::new(Mutex::new(UsersOnline::new()));

    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate key");
    let public_key = RsaPublicKey::from(&private_key);
    let pem =
        EncodePublicKey::to_public_key_pem(&public_key, rsa::pkcs8::LineEnding::CRLF).unwrap();
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in receiver_listener.incoming() {
        let mut stream = stream.expect("failed");

        let shared_map_clone = Arc::clone(&shared_map);
        let private_key_clone = private_key.clone();

        stream.write_all(&pem.len().to_be_bytes())?;
        stream.write_all(pem.as_bytes())?;

        let handle = thread::spawn(move || {
            handle_sender(stream, shared_map_clone, private_key_clone)
                .unwrap_or_else(|error| eprintln!("{:?}", error))
        });

        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
