use clap::Parser;
use std::io;
use anyhow::Result;
use base64::encode;
use std::fs;

#[cfg(target_os = "windows")]
fn get_os_name()->&'static str{
    "windows"
}

#[cfg(target_os = "linux")]
fn get_os_name()->&'static str{
    "linux"
}

#[cfg(target_os = "macos")]
fn get_os_name()->&'static str{
    "macos"
}

#[derive(Parser)]
#[command(version, about = "input file to encode, output file with encoding")]
struct Args{
    #[arg(long,default_value = "stdin")]
    input: String,

    #[arg(long,default_value = "stdout")]
    output:String,
}

fn main() -> Result<()>{
    let args = Args::parse();
    println!("encoder, version {}, built for {}",env!("CARGO_PKG_VERSION"),get_os_name());

    if args.input == "stdin" && args.output == "stdout"{
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.remove(input.len() - 1);
        input.remove(input.len() - 1);
        println!("{}",encode(&Vec::from(input)));
        return Ok(());
    }

    let input_string = fs::read_to_string(args.input)?;
    fs::write(args.output,encode(&Vec::from(input_string)))?;

    Ok(())
}
