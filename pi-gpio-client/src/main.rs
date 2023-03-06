use core::time;
use gpio::{sysfs::{SysFsGpioInput, SysFsGpioOutput}, GpioIn, GpioOut};
use multicaster::{self, Multicaster};
use std::{thread, time::Duration};
use tokio;



struct Buzzer {
    gpio: SysFsGpioOutput
}

impl Buzzer {
    fn new() -> Self {
        Self {
            gpio: gpio::sysfs::SysFsGpioOutput::open(27).unwrap()
        }
    }
    fn beep(&mut self, duration: u64) {
        self.gpio.set_high().unwrap();
        thread::sleep(Duration::from(time::Duration::from_millis(500)));
        self.gpio.set_low().unwrap();
    }
}

#[tokio::main]
async fn main() {
    let multi = Multicaster::new("0.0.0.0", "239.0.0.20", 5007, true).await;
    
    let mut buzzer = Buzzer::new();
    buzzer.beep(500);
}
