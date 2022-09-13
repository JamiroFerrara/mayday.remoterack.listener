use gpio::GpioOut;

pub struct Ks0212 {
    pub outpins: Vec<u16>
}

impl Ks0212 {
    pub fn new() -> Self {
        let outpins = vec![7,3,22,25];
        Ks0212 { outpins }
    }

    pub fn set_relay_value(&mut self, index: usize, value: bool) {
        // Let's open GPIO23 and -24, e.g. on a Raspberry Pi 2.
        let mut gpio = gpio::sysfs::SysFsGpioOutput::open(self.outpins[index]).unwrap();
        match gpio.set_value(value) {
            Ok(_) => println!("Set value of pin {} to {}", self.outpins[index], value),
            Err(e) => println!("Error setting value of pin {} to {}: {}", self.outpins[index], value, e),
        }
    }
}
