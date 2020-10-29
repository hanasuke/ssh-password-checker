use serde::{Deserialize, Serialize};
use serde_json;

use std::env;
use std::process::Command;

#[allow(dead_code)]
const RESULT_PASS: u16 = 100;
#[allow(dead_code)]
const RESULT_SUCCESS: u16 = 200;
#[allow(dead_code)]
const RESULT_WARNING: u16 = 300;
#[allow(dead_code)]
const RESULT_CRITICAL: u16 = 400;

#[derive(Serialize, Deserialize)]
struct ResultStruct {
    message: String,
    result_code: u16,
}

fn main() {
    // parse command line
    let target_host = parse_args(env::args().collect());
    let command_result = exec_ssh_command(target_host);

    // "naosuke@10.0.1.18: Permission denied (publickey,password)." みたいなのが入ってくる
    let stderr: String = String::from_utf8_lossy(&command_result.stderr).to_string();
    let (code, message) = parse_stderr(stderr);

    let output = ResultStruct {
        message: message,
        result_code: code,
    };

    println!("{}", serde_json::to_string(&output).unwrap());
}

fn parse_stderr(stderr: String) -> (u16, String) {
    return if stderr.contains("password") {
        // password auth有効のとき
        (RESULT_WARNING, "password authentication is enable".to_string())
    } else if stderr.contains("Connection refused") || stderr.contains("Operation timed out") {
        // sshがそもそも接続失敗
        (RESULT_PASS, "ssh couldn't connect to this host".to_string())
    } else {
        // それ以外
        (RESULT_PASS, "no problem!".to_string())
    }
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
