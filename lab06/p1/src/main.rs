use anyhow::Result;
use std::fs;
use std::process::exit;
struct Terminal {
    v: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal {
        let t = Terminal { v: Vec::new() };
        t
    }
    fn register(&mut self, c: Box<dyn Command>) {
        self.v.push(c);
    }
    fn run(&mut self) -> Result<()> {
        let input = fs::read_to_string("src/input.txt")?;
        let mut check;

        for mut line in input.lines() {
            line = line.trim();

            if line.len() == 0 {
                println!("Empty Line!");
            } else {
                check = 0;
                if let Some(lsplit) = line.split_once(" ") {
                    if lsplit.0 == "stop" {
                        self.stop();
                    }

                    for el in &mut self.v {
                        if el.get_name() == lsplit.0 {
                            check = 1;
                            let _ = el.exec(&[lsplit.1]);
                        } else if el.get_name() == lsplit.0.to_ascii_uppercase() {
                            println!("Correct command name: {}", el.get_name())
                        }
                    }
                } else {
                    if line == "stop" {
                        self.stop();
                    }

                    for el in &mut self.v {
                        if el.get_name() == line {
                            check = 1;
                            let _ = el.exec(&[]);
                        } else if el.get_name() == line.to_ascii_uppercase() {
                            println!("Correct command name: {}", el.get_name())
                        }
                    }
                }
                if check == 0 {
                    println!("Error!");
                }
            }
        }

        Ok(())
    }
}

trait Command {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: &[&str]) -> Result<()>;
}

struct PingCommand {}

impl Command for PingCommand {
    fn get_name(&self) -> &'static str {
        "ping"
    }

    fn exec(&mut self, _args: &[&str]) -> Result<()> {
        println!("pong!");
        Ok(())
    }
}

struct CountCommand {}

impl Command for CountCommand {
    fn get_name(&self) -> &'static str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        let mut count = 0;
        for word in args {
            for letter in word.chars() {
                if letter != ' ' {
                    count += 1;
                }
            }
        }
        println!("counted {} args", count);

        Ok(())
    }
}

struct CountLowCommand {}

impl Command for CountLowCommand {
    fn get_name(&self) -> &'static str {
        "countlower"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        let mut count = 0;
        for word in args {
            for letter in word.chars() {
                if letter.is_lowercase() {
                    count += 1;
                }
            }
        }
        println!("counted {} lowercase args", count);

        Ok(())
    }
}
struct TimesCommand {
    count: usize,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &'static str {
        "times"
    }

    fn exec(&mut self, _args: &[&str]) -> Result<()> {
        self.count += 1;
        println!("called {} times", self.count);

        Ok(())
    }
}

// struct <urchoice>{

// }

impl Terminal {
    fn stop(&mut self) {
        exit(0);
    }
}

fn main() -> Result<()> {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    //terminal.register(Box::new(CountLowCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));

    terminal.run()?;

    Ok(())
}
