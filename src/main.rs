extern crate libc;

use libc::kill;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::process;
use std::process::{Command, Stdio};
extern crate clap;
use clap::{App, Arg};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut part = args.split(|elem| elem == "--");

    let opts = part.next().unwrap();

    let app_matches = App::new("A solo player")
        .version("0.1.0")
        .author("Thinh T. <duythinht@gmail.com>")
        .about("Prevent application running twice")
        .arg(
            Arg::with_name("pid")
                .short("p")
                .long("pid")
                .help("path to pid file")
                .takes_value(true),
        )
        .get_matches_from(opts);

    let pid_path = app_matches.value_of("pid").unwrap_or("default.pid");

    println!("Checking PID file at '{}'", pid_path);

    if let Ok(mut f) = File::open(&pid_path) {
        let mut s = String::new();
        if let Ok(_) = f.read_to_string(&mut s) {
            let pid: i32 = match s.trim().parse() {
                Ok(pid) => pid,
                Err(err) => {
                    println!("Parse pid error: {:?}, please check pid file!", err);
                    0
                }
            };
            if is_running(pid) {
                println!("Process is running...");
                return;
            }
        };
    }

    match OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(&pid_path)
    {
        Ok(mut f) => {
            println!("Exec command...\n-------------------------------------------------");

            if let Ok(_) = write!(f, "{}", process::id()) {}

            let cmd = part
                .next()
                .unwrap()
                .into_iter()
                .map(|elm| {
                    if let Some(_) = elm.find(' ') {
                        let mut s = "'".to_string();
                        s.push_str(elm);
                        s.push_str("'");
                        s.to_string()
                    } else {
                        elm.to_string()
                    }
                })
                .collect::<Vec<_>>();
            println!("$ {}", cmd.join(" "));
            println!("-------------------------------------------------");
            exec(&cmd.join(" "))
        }
        Err(e) => panic!("{:?}", e),
    }
}

fn is_running(pid: i32) -> bool {
    unsafe {
        let status = kill(pid, 0);
        println!("Signal sent, status = {}", status);
        return status == 0;
    }
}

#[warn(dead_code)]
fn exec(command: &str) {
    Command::new("sh")
        .stdout(Stdio::inherit())
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
}
