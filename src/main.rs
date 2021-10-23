use std::io::{self, BufRead};
use tokio::signal;
use tokio::sync::watch;
use tokio::time::{sleep, Duration};
use regex::Regex;
use tokio::sync::broadcast;
use tokio::task;

mod actors;

use actors::stdin::InteractiveParserHandle;

async fn say_world() {
    println!("world");
}

async fn ctrlc_handler(tx: broadcast::Sender<()>) {
    signal::ctrl_c().await;
    println!();
    println!("Closing...");
    tx.send((())).expect("TODO");
}
//clap rust -> cmd
fn read_input() {
    let mut line = String::new();
    let stdin = io::stdin();
    let re_send = Regex::new("^send").unwrap();
    //let handle_send = tokio::spawn(async move {send_can()}.await );

    loop {
        stdin.lock().read_line(&mut line).expect("Could not read line");
        let op = line.trim_right();
        
        if op == "EXIT" {
            break;
        } else if re_send.is_match(op) {
            //tokio::spawn(async move {send_can(op.to_string())}.await).await;
        }
        line.clear();
    }
}

async fn send_can(cmd: String) {
    let re_canid = Regex::new(r".* -id (\d+)").unwrap();
    let re_candata = Regex::new(r".* -m [[:punct:]](.*)[[:punct:]]").unwrap();
    let mut canid = String::new();
    let mut msg = String::new();
    let mut stdin = io::stdin();
    // println!("match, {}", &cmd);
    if re_canid.is_match(&cmd) {
        let cap = re_canid.captures(&cmd).unwrap();
        canid = cap[1].to_string();
        println!("{} -> CAN id", canid);
    }
    if re_candata.is_match(&cmd) {
        let cap = re_candata.captures(&cmd).unwrap();
        msg = cap[1].to_string();
        println!("{} -> CAN data", msg);
    }
    // println!("CAN ID:");
    // stdin.lock().read_line(&mut canid).expect("Could not read line");
    // println!("Message:");
    // stdin.lock().read_line(&mut msg).expect("Could not read line");
    // send(canid, msg);
}


fn send(canid : String, msg : String) {
    println!("Message Sent \nid: {}message: {}", canid, msg);
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(1);

    InteractiveParserHandle::new();


    //let res = task::spawn_blocking(move || {
    //    read_input()
    //});

    ctrlc_handler(tx).await;
    std::process::exit(0);
}

