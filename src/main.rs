use std::io;
use std::fs;
use std::fs::File;

#[derive(Debug)]
struct LightSensor {
    status: bool,
    intensity: u32,
}
struct FireAlarm {
    status: bool,
    intensity: u32,
}
struct GateAlarm {
    status: bool,
    intensity: u32,
}

pub trait Func {
    fn double(&mut self);
    fn dec(&mut self);
    fn switch_on(&mut self);
    fn switch_off(&mut self);
    fn check(&mut self);
}

impl Func for LightSensor {
    fn double(&mut self) {
        self.intensity = self.intensity + 2;
    }

    fn dec(&mut self) {
        self.intensity = self.intensity - 2;
        if self.intensity == 0 {
            self.status = false;
            println!("Light turned off");
        }
    }

    fn switch_on(&mut self) {
        self.status = true;
        self.intensity = 2;
    }

    fn switch_off(&mut self) {
        self.status = false;
        self.intensity = 0;
    }

    fn check(&mut self) {
        println!("");
        println!("Status = {}\nIntensity = {}", self.status, self.intensity);
    }
}

fn main() {
    println!("Type 'login' or 'signup'");
    let mut log = String::new();
    io::stdin().read_line(&mut log)
        .expect("Wrong command");
    let log: String = log.trim().parse().unwrap();
    match log.as_ref() {
        "login" => login(),
        "signup" => signup(),
        _ => panic!("Please enter the right command"),
    }
//     let mut dev_1 = LightSensor {status: false, intensity: 0};
//     dev_1.switch_on();
//     println!("{:?}", dev_1);
}

fn signup() {
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
    fs::write(filename, pass)
        .expect("unable to write to file");
    login();
}

fn login() {
    println!("Please enter your username");
    let mut username = String::new();
    io::stdin().read_line(&mut username)
        .expect("Enter command");
    let username: String = username.trim().parse().unwrap();
    let filename = format!("{}.txt",username);
    let f = match File::open(&filename) {
        Ok(file) => file,
        Err(e) => panic!("Cannot find user"),
    };
    println!("Please enter your  password");
    let mut pass = String::new();
    io::stdin().read_line(&mut pass)
        .expect("Enter password");
    let pass: String = pass.trim().parse().unwrap();
    let content = fs::read_to_string(filename)
        .expect("Something went wrong");
    if content != pass {
        panic!("The password does not match");
    }
    else {
        println!("Access granted");
    }    
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