fn main() {
   let mut dev_1 = LightSensor {status: true, intensity: 2};
   println!("{:?}", dev_1);
   dev_1.double();
   println!("{:?}", dev_1);

}