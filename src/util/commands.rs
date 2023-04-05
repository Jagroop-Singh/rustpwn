use std::io::{self, prelude::*};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

pub async fn menu() {
    let mut process: Child = Command::new("ls").stdout(Stdio::piped()).spawn().unwrap();
    let mut stdin: Option<ChildStdin> = None;
    let mut stdout: Option<ChildStdout> = None;
    let mut reader: Option<BufReader<ChildStdout>> = None;

    loop {
        print!("[Command]: ");
        io::stdout().flush();
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        match command.as_str().trim() {
            "start" | "r" => {
                process = start_process().await;
                stdin = process.stdin.take();
                stdout = process.stdout.take();
                reader = Some(BufReader::new(stdout.unwrap()));
                // Ensure the child process is spawned in the runtime so it can
                // make progress on its own while we await for any output.
                tokio::spawn(async move {
                    let status = process
                        .wait()
                        .await
                        .expect("child process encountered an error");

                    println!("child status was: {}", status);
                });
            }
            "readl" => {
                read_l(&mut reader.as_mut().unwrap()).await;
            }
            "readu" => {
                read_until(&mut reader.as_mut().unwrap()).await;
            }
            "sendl" => {
                send_l(&mut stdin.as_mut().unwrap()).await;
            }
            "cyclic" =>{
                cyclic(100usize);
            }
            "help" | "h" => help(),
            "exit" | "quit" | "end" => break,
            _ => println!("Input not recognized"),
        }
    }
}

fn help() {
    println!("start/r to run process\nend to end program\nh to display help");
}

async fn start_process() -> Child {
    print!("[Enter Process Name]: ");
    io::stdout().flush();
    let mut process: String = String::new();
    std::io::stdin().read_line(&mut process).unwrap();
    print!("[Args]: ");
    io::stdout().flush();
    let mut arg: String = String::new();
    std::io::stdin().read_line(&mut arg).unwrap();
    println!("process: {} arg: {}", &process, &arg);
    let p = match Command::new(&process.trim())
        .arg(&arg.trim())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(err) => panic!("Couldn't spawn {}: {}", process, err),
        Ok(process) => process,
    };
    p
}

async fn read_until(reader: &mut BufReader<ChildStdout>) -> std::io::Result<()> {
    let mut input = String::new();
    let mut delim: u8 = 0;
    println!("Stop at Byte: ");
    std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    delim = input.bytes().nth(0).expect("no byte read"); 
    let mut s = vec![];
    reader.read_until(delim, &mut s).await?;
    for i in s {
        print!("{}", i as char);
    }
    println!();
    Ok(())
}

async fn read_l(reader: &mut BufReader<ChildStdout>) -> std::io::Result<()> {
    println!("Line: {}", reader.lines().next_line().await?.unwrap());
    Ok(())
}

async fn send_l(p: &mut ChildStdin) {
    println!("What do you want to write");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    p.write(input.as_bytes())
        .await
        .expect("could not write to stdin");
}

async fn send_char() {}

const UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";

pub fn cyclic(length: usize){
    let mut payload: Vec<String> = Vec::new();
    for c in UPPER_CASE.chars() {
        for s in LOWER_CASE.chars() {
            for n in NUMBERS.chars() {
                if payload.len() < length {
                    payload.push(c.to_string());
                }
                if payload.len() < length {
                    payload.push(s.to_string());
                }
                if payload.len() < length {
                    payload.push(n.to_string());    
                }
            }
        }
    }
    println!("{}", payload.join(""));
}

// pub fn offset(value: &str){
//     let payload = cyclic(20280);
//     if let Some(i) = payload.find(value) {
//         println!("{}", i);
//         return Some(i);
//     } else {
//         println!("Not Found");
//     }
// }
