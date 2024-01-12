use clap::Parser;
use rsa::{
    pkcs8::DecodePublicKey, pkcs8::EncodePublicKey, pkcs8::LineEnding, Pkcs1v15Encrypt,
    RsaPrivateKey, RsaPublicKey,
};
use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::thread;

#[derive(Parser)]
#[command(version, about = "cargo run [-- --IP -- PORT] \n\n List of commands:\n\t1.login <username> <password>\n\t2.start_chat <recipient>\n\t3.end_chat\n\t4.send_message <message>\n\t5.history\n\t6.reply_to <message_index> <message>\n\t7.logout\n\t8.help\n")]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    ip: String,

    #[arg(long, default_value = "7878")]
    port: String,
}

fn handle_stdin(mut stream: TcpStream, server_pub_key: RsaPublicKey) -> Result<(), io::Error> {
    println!("List of commands:\n\t1.login <username> <password>\n\t2.start_chat <recipient>\n\t3.end_chat\n\t4.send_message <message>\n\t5.history\n\t6.reply_to <message_index> <message>\n\t7.logout\n\t8.help\n\n");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut rng = rand::thread_rng(); //encrypt and send
        let enc_message = server_pub_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, input.as_bytes())
            .expect("Failed to encrypt");
        stream.write_all(&enc_message.len().to_be_bytes())?;
        stream.write_all(&enc_message)?;
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let addr = format!("{}:{}", args.ip, args.port);

    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            let mut rng = rand::thread_rng();
            let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate key");
            let public_key = RsaPublicKey::from(&private_key);
            let pem = public_key.to_public_key_pem(LineEnding::CRLF).unwrap();

            let pem_size = pem.len().to_be_bytes();
            stream.write_all(&pem_size)?;
            stream.write_all(pem.as_bytes())?;

            let stream_copy = stream.try_clone().expect("clone failed ...");

            let mut reader = BufReader::new(&stream);

            let mut size_bytes = [0; 8];
            reader.read_exact(&mut size_bytes)?;
            let server_pem_size = usize::from_be_bytes(size_bytes);
            let mut server_pem_bytes = vec![0; server_pem_size];
            reader.read_exact(&mut server_pem_bytes)?;

            let server_pub_key =
                RsaPublicKey::from_public_key_pem(&String::from_utf8_lossy(&server_pem_bytes))
                    .unwrap();

            thread::spawn(move || {
                handle_stdin(stream_copy, server_pub_key.clone())
                    .unwrap_or_else(|e| println!("{}", e))
            });

            loop {

                let mut size_bytes = [0; 8];
                match reader.read_exact(&mut size_bytes) {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                };

                let message_size = usize::from_be_bytes(size_bytes);

                let mut message = vec![0; message_size];
                reader.read_exact(&mut message)?;

                let response = private_key
                    .decrypt(Pkcs1v15Encrypt, message.as_slice())
                    .expect("failed to decrypt");
                println!(
                    "read from server:{}\n",
                    String::from_utf8(response).unwrap()
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
    Ok(())
}
