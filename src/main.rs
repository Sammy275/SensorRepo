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

}