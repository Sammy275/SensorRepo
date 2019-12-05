#[derive(Debug)]
pub struct LightSensor {
    pub status: bool,
    pub intensity: u32,
}


#[derive(Debug)]
pub struct FireAlarm {
    pub status: bool,
    pub intensity: u32,
}


#[derive(Debug)]
pub struct GateAlarm {
    pub status: bool,
    pub intensity: u32,
}

pub trait Func {
    fn double(&mut self);
    fn dec(&mut self);
    fn switch_on(&mut self);
    fn switch_off(&mut self);
    fn check(&mut self);
}

// For LightSensor

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

//For Fire Alarm
impl Func for FireAlarm {
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

//For Gate Alarm
impl Func for GateAlarm {
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