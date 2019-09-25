use std::io::{self, Write};
use std::env;
use std::path::Path;

mod builtin;

fn builtin(cmds: &Vec<&str>) -> i32 {
    match cmds[0] {
        "cd" => {
            builtin::cd(cmds);
            1
        },
        "exit" => {
            builtin::exit();
            1
        },
        _ => 0,
    }
}

fn main() {
    // start shell in /
    let root = Path::new("/");
    env::set_current_dir(&root).unwrap();

    let ps1 = env::var("PS1").unwrap_or(String::from("Z :: "));

    loop {
        print!("{}", ps1);
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("Failed to read line");
        cmd.pop();

        let cmdsplt: Vec<_> = cmd.split(" ").collect();

        // check for builtin commands
        let res = builtin(&cmdsplt);
        if res == 1 {
            continue;
        }

        let mut cmd = std::process::Command::new(cmdsplt[0].to_string());
        
        if cmdsplt.len() > 1 {
            for c in cmdsplt[1..].iter() {
                cmd.arg(c);
            }
        }

        match cmd.status() {
            Ok(r) => r,
            Err(_) => {
                println!("zee: error: could not execute {}", cmdsplt[0]);
                continue;
            },
        };

    }
}
