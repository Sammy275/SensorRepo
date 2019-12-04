use std::io;
use std::fs;
use std::fs::File;
use std::process;
extern crate logic;
use logic::{LightSensor, FireAlarm, GateAlarm};
use logic::Func;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    println!("Type 'login' or 'signup'");
    let mut log = String::new();
    io::stdin().read_line(&mut log)
        .expect("Wrong command");
    let log: String = log.trim().parse().unwrap();
    let mut filename = match log.as_ref() {
        "login" => login(),
        "signup" => signup(),
        _ => {println!("Please enter the right command"); process::exit(1);},
    };
    let mut content = fs::read_to_string(&filename)
        .expect("Something went wrong");
    println!("Your previous logs = {}", content);
    let new_data = process();
    let mut file = OpenOptions::new().append(true).create(true).open(&filename).unwrap();
    write!(&mut file, "\n{:?}", new_data);
    println!("GoodBye Have A Nice Day");
    // let mut dev_1 = LightSensor {status: false, intensity: 0};
    // dev_1.switch_on();
    // println!("{:?}", dev_1);
}


// sign up functionality
fn signup() -> String {
    println!("Enter your username");
    let mut username = String::new();
    io::stdin().read_line(&mut username)
        .expect("Enter command");
    let username: String = username.trim().parse().unwrap();
    let filename = format!("{}.txt",username);
    let filename2 = format!("{}dev.txt",username);
    println!("Enter your password");
    let mut pass = String::new();
    io::stdin().read_line(&mut pass);
    let pass: String = pass.trim().parse().unwrap();
    if pass.len() == 0 {
        println!("Enter password");
        process::exit(1);
    }
    File::create(&filename);
    File::create(&filename2);
    fs::write(&filename, pass)
        .expect("unable to write to file");
    login();
    return filename2;
}


// log in functionality
fn login() -> String {
    println!("Please enter your username");
    let mut username = String::new();
    io::stdin().read_line(&mut username)
        .expect("Enter command");
    let username: String = username.trim().parse().unwrap();
    let filename = format!("{}.txt",username);
    let filename2 = format!("{}dev.txt",username);
    let f = match File::open(&filename) {
        Ok(file) => file,
        Err(e) => {
            println!("The system couldnt find the user");
            process::exit(1);
        },
    };
    println!("Please enter your password");
    let mut pass = String::new();
    io::stdin().read_line(&mut pass)
        .expect("Enter password");
    let pass: String = pass.trim().parse().unwrap();
    let content = fs::read_to_string(&filename)
        .expect("Something went wrong");
    if content != pass {
        println!("The password does not match");
        process::exit(1);
    }
    else {
        println!("Access granted");
    }    
    filename2
}

fn process() -> LightSensor {    
    let mut opt = String::new();
    println!("1: Light sensor\n2: Fire Alarm\n3: Gate Alarm");
    io::stdin().read_line(&mut opt)
        .expect("Please enter something");
    let opt: String = opt.trim().parse().unwrap();
    let data = match opt.as_ref() {
        "1" => light(),
        // "2" => fire(),
        // "3" => gate(),
        _ => {println!("Enter right number"); process::exit(1)},
    };
    data
}




fn light() -> LightSensor {
    let mut dev_1 = LightSensor {status: false, intensity: 0};
    let mut status = String::new();
    println!("Do you want to turn on the light? type 'On'");
    io::stdin().read_line(&mut status)
        .expect("Please Enter Command");
    let status: String = status.trim().parse().unwrap();
    match status.as_ref() {
        "On" => dev_1.switch_on(),
        _ => {println!("Please enter correct command"); process::exit(1)},
    }
    loop {
        println!("");
        println!("-------------------------------------");
        println!("Type 'Increase' to increase intensity\nType 'Decrease' to decrease intensity\nType 'Off' to turn of the light\nType 'Check' to check the status of device\nType 'Back to return'");
        let mut status = String::new();
        io::stdin().read_line(&mut status)
            .expect("Enter the command");
        let status: String = status.trim().parse().unwrap();
        match status.as_ref() {
            "Increase" => dev_1.double(),
            "Decrease" => dev_1.dec(),
            "Off" => dev_1.switch_off(),
            "Check" => dev_1.check(),
            "Back" => break,
            _ => println!("Enter the right command"),
        }    
    }
    dev_1
}