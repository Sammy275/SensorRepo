use std::io;

#[derive(Debug)]
struct LightSensor {
    status: bool,
    intensity: u32,
}

impl LightSensor {
    fn double(&mut self) {
        self.intensity = self.intensity + 2;
    }

    fn dec(&mut self) {
        self.intensity = self.intensity - 2;
        if self.intensity == 0 {
            self.status = false;
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
}

fn main() {
    let mut dev_1 = LightSensor {status: false, intensity: 0};
    let mut status = String::new();
    println!("Do you want to turn on the light? type 'On'");
    io::stdin().read_line(&mut status);
    let status: String = status.trim().parse().unwrap();
    match status {
        On => dev_1.switch_off(),
        _ => panic!("Please enter correct command"),
    }
    loop {
        println!("");
        println!("-------------------------------------");
        println!("Type 'Increase' to increase intensity");
        println!("Type 'Decrease' to decrease intensity");
        println!("Type 'Off' to turn of the light");
        println!("Type 'Check' to check the status of device");
        let mut status = String::new();
        io::stdin().read_line(&mut status);
        let status: String = status.trim().parse().unwrap();
        match status {
            Increase => dev_1.double(),
            Decrease => dev_1.dec(),
        }    
    }
}