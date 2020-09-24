use serde::{Deserialize, Serialize};
use serde_json;

use std::env;
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct ResultStruct {
    status: bool,
    result: String,
}

fn main() {
    // parse command line
    let target_host = parse_args(env::args().collect());
    let command_result = exec_ssh_command(target_host);

    // "naosuke@10.0.1.18: Permission denied (publickey,password)." みたいなのが入ってくる
    let stderr: String = String::from_utf8_lossy(&command_result.stderr).to_string();
    let result = if stderr.contains("password") {
        ResultStruct {
            status: false,
            result: "[WARN] password authentication is enable".to_string(),
        }
    } else if stderr.contains("Connection refused") {
        ResultStruct {
            status: true,
            result: "[INFO] Connection refused".to_string(),
        }
    } else {
        ResultStruct {
            status: true,
            result: "[INFO] no problem!".to_string(),
        }
    };

    println!("{}", serde_json::to_string(&result).unwrap());
}

fn parse_args(exec_args: Vec<String>) -> Vec<String> {
    let mut args = exec_args.clone();
    let mut target_host: Vec<String> = Vec::new();

    match args.len() {
        2 => {
            target_host.push(args.remove(1));
        },
        3 => {
            target_host.push(String::from("-p"));
            target_host.push(args.remove(2));
            target_host.push(args.remove(1));
        },
        _ => {
            panic!();
        },
    };

    return target_host
}

fn exec_ssh_command(target_host: Vec<String>) -> std::process::Output {
    return Command::new("ssh")
                    .arg("-tt")
                    .arg("-o StrictHostKeyChecking=no")
                    .arg("-o PubkeyAuthentication=no")
                    .arg("-o PasswordAuthentication=no")
                    .arg("-o ChallengeResponseAuthentication=no")
                    .arg("-o ConnectTimeout=2")
                    .args(target_host).output().expect("failed");
}
