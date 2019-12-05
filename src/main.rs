use std::io;
use std::fs;
use std::fs::File;
use std::process;
extern crate loging;
use loging::logs;
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
        "login" => logs::login(),
        "signup" => logs::signup(),
        _ => {println!("Please enter the right command"); process::exit(1);},
    };
    loop {
    // let mut content = fs::read_to_string(&filename)
    //     .expect("Something went wrong");
    // println!("Your previous logs = {}", content);
    process(/*&filename*/);
    }
}


fn process(/*filename: &String*/) {    
    let mut opt = String::new();
    println!("Please select anyone");
    println!("1: Light sensor\n2: Fire Alarm\n3: Gate Alarm\n4: Cancel");
    io::stdin().read_line(&mut opt)
        .expect("Please enter something");
    let opt: String = opt.trim().parse().unwrap();
    match opt.as_ref() {
        "1" => light(/*&filename*/),
        "2" => fire(/*&filename*/),
        "3" => gate(/*&filename*/),
        "4" => {println!("GoodBye Have a Nice Day"); process::exit(5)},
        _ => {println!("Enter right number"); process::exit(1)},
    };
}




fn light(/*filename: &String*/) {
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
    // let mut file = OpenOptions::new().append(true).create(true).open(&filename).unwrap();
    // write!(&mut file, "\n{:?}", dev_1);
}



fn fire(/*filename: &String*/) {
    let mut dev_1 = FireAlarm {status: false, intensity: 0};
    let mut status = String::new();
    println!("Do you want to turn on the alarm? type 'On'");
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
//     let mut file = OpenOptions::new().append(true).create(true).open(&filename).unwrap();
//     write!(&mut file, "\n{:?}", dev_1);
}

fn gate(/*filename: &String*/) {
    let mut dev_1 = GateAlarm {status: false, intensity: 0};
    let mut status = String::new();
    println!("Do you want to turn on the alarm? type 'On'");
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
    // let mut file = OpenOptions::new().append(true).create(true).open(&filename).unwrap();
    // write!(&mut file, "\n{:?}", dev_1);
}