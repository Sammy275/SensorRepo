use std::io;
use std::fs;
use std::fs::File;
use std::process;
extern crate logic;
use logic::{LightSensor, FireAlarm, GateAlarm};
use logic::Func;

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
    println!("{}", filename);
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
    File::create(&filename);
    println!("Enter your password");
    let mut pass = String::new();
    io::stdin().read_line(&mut pass)
        .expect("Enter password");
    let pass: String = pass.trim().parse().unwrap();
    fs::write(&filename, pass)
        .expect("unable to write to file");
    login();
    return filename;
}


// log in functionality
fn login() -> String {
    println!("Please enter your username");
    let mut username = String::new();
    io::stdin().read_line(&mut username)
        .expect("Enter command");
    let username: String = username.trim().parse().unwrap();
    let filename = format!("{}.txt",username);
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
    filename
}












    // let mut dev_1 = LightSensor {status: false, intensity: 0};
    // let mut status = String::new();
    // println!("Do you want to turn on the light? type 'On'");
    // io::stdin().read_line(&mut status)
    //     .expect("Please Enter Command");
    // let status: String = status.trim().parse().unwrap();
    // match status.as_ref() {
    //     "On" => dev_1.switch_on(),
    //     _ => panic!("Please enter correct command"),
    // }
    // loop {
    //     println!("");
    //     println!("-------------------------------------");
    //     println!("Type 'Increase' to increase intensity");
    //     println!("Type 'Decrease' to decrease intensity");
    //     println!("Type 'Off' to turn of the light");
    //     println!("Type 'Check' to check the status of device");
    //     let mut status = String::new();
    //     io::stdin().read_line(&mut status)
    //         .expect("Enter the command");
    //     let status: String = status.trim().parse().unwrap();
    //     match status.as_ref() {
    //         "Increase" => dev_1.double(),
    //         "Decrease" => dev_1.dec(),
    //         "Off" => dev_1.switch_off(),
    //         "Check" => dev_1.check(),
    //         _ => println!("Enter the right command"),
    //     }    
    // }
// }