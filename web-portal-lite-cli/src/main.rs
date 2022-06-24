use clap::Parser;
use std::io;
use std::io::Write;
use web_portal_lite_core::create_hashed_password;

mod args;

fn handle_password_hasher() {
    let mut password = String::new();
    print!("enter password: ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut password) {
        Ok(_) => match create_hashed_password(&password) {
            Ok(hashed_pw) => println!("hashed password: {hashed_pw}"),
            Err(_) => eprintln!("error hashing password"),
        },
        Err(error) => eprintln!("error: {error}"),
    }
}

fn main() {
    let args = args::Args::parse();
    match args.cmd {
        args::Command::PwHasher => handle_password_hasher(),
    };
}
