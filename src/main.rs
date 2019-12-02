#[derive(Debug)]
struct LightSensor {
    status: bool,
    intensity: u32,
}

impl LightSensor {
    fn double(&mut self) {
        self.intensity = self.intensity + 2;
    }
}

fn main() {
   let mut dev_1 = LightSensor {status: true, intensity: 2};
   println!("{:?}", dev_1);
   dev_1.double();
   println!("{:?}", dev_1);

}