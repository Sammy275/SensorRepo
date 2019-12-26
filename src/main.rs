use std::io;
use std::fs;
use std::thread;
use std::time::Duration;

use std::process;
extern crate loging;
use loging::logs;
extern crate logic;
use logic::{LightSensor, FireAlarm, GateAlarm};
use logic::Func;
use std::fs::OpenOptions;
use std::io::prelude::*;


fn main() {
    loop {
    println!("\nType 'login' or 'signup' or 'quit' to exit");
    let mut log = String::new();
    io::stdin().read_line(&mut log)
        .expect("Wrong command");
    let log: String = log.trim().parse().unwrap();
    let filename = match log.as_ref() {
        "login" => logs::login(),
        "signup" => logs::signup(),
        "quit" => {println!("\nGoodBye!!!"); thread::sleep(Duration::from_secs(2)); process::exit(0);},
        _ => {println!("\nPlease enter the right command"); continue;},
    };
    loop {
    process(&filename);
    }
    }
}


fn process(filename: &String) { 
    let content = fs::read_to_string(&filename)
    .expect("Something went wrong");
    let vec: Vec<&str> = content.trim().split('\n').collect();
    println!("--------------------Logs--------------------");
    for i in 1..vec.len() {
        println!("{:?}", vec[i]);
    }   
    let mut opt = String::new();
    println!("Please select anyone");
    println!("1: Light sensor\n2: Fire Alarm\n3: Gate Alarm\n4: Cancel");
    io::stdin().read_line(&mut opt)
        .expect("Please enter something");
    let opt: String = opt.trim().parse().unwrap();
    match opt.as_ref() {
        "1" => light(&filename),
        "2" => fire(&filename),
        "3" => gate(&filename),
        "4" => {println!("GoodBye Have a Nice Day"); thread::sleep(Duration::from_secs(2)); main()},
        _ => {println!("Enter right number"); thread::sleep(Duration::from_secs(1));},
    };
}




fn light(filename: &String) {
    let mut dev_1 = LightSensor {status: false, intensity: 0};
    loop {
    println!("Do you want to turn on the light? type 'On'");
    let mut status = String::new();
    io::stdin().read_line(&mut status)
        .expect("Please Enter Command");
    let status: String = status.trim().parse().unwrap();
    match status.as_ref() {
        "On" => {dev_1.switch_on(); break;},
        _ => {println!("Please enter correct command"); thread::sleep(Duration::from_secs(1)); continue;},
    }
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
    let mut file = OpenOptions::new().append(true).create(true).open(&filename).unwrap();
    match write!(&mut file, "\n{:?}", dev_1) {
        Ok(r) => r,
        Err(_) => panic!("Error occured "),
    };
}



fn fire(filename: &String) {
    let mut dev_1 = FireAlarm {status: false, intensity: 0};
    loop {
        println!("Do you want to turn on the light? type 'On'");
        let mut status = String::new();
        io::stdin().read_line(&mut status)
            .expect("Please Enter Command");
        let status: String = status.trim().parse().unwrap();
        match status.as_ref() {
            "On" => {dev_1.switch_on(); break;},
            _ => {println!("Please enter correct command"); thread::sleep(Duration::from_secs(1)); continue;},
        }
    }
    loop {
        println!("");
        println!("-------------------------------------");
        println!("Type 'Increase' to increase intensity\nType 'Decrease' to decrease intensity\nType 'Off' to turn of the alarm\nType 'Check' to check the status of device\nType 'Back to return'");
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
    let mut file = OpenOptions::new().append(true).create(true).open(&filename).unwrap();
    match write!(&mut file, "\n{:?}", dev_1) {
        Ok(r) => r,
        Err(_) => panic!("Error occured "),
    };
}

fn gate(filename: &String) {
    let mut dev_1 = GateAlarm {status: false, intensity: 0};
    loop {
        println!("Do you want to turn on the light? type 'On'");
        let mut status = String::new();
        io::stdin().read_line(&mut status)
            .expect("Please Enter Command");
        let status: String = status.trim().parse().unwrap();
        match status.as_ref() {
            "On" => {dev_1.switch_on(); break;},
            _ => {println!("Please enter correct command"); thread::sleep(Duration::from_secs(1)); continue;},
        }
    }
    loop {
        println!("");
        println!("-------------------------------------");
        println!("Type 'Increase' to increase intensity\nType 'Decrease' to decrease intensity\nType 'Off' to turn of the alarm\nType 'Check' to check the status of device\nType 'Back to return'");
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
    let mut file = OpenOptions::new().append(true).create(true).open(&filename).unwrap();
    match write!(&mut file, "\n{:?}", dev_1) {
        Ok(r) => r,
        Err(_) => panic!("Error occured "),
    };
}