use core::time;
use gpio::{sysfs::SysFsGpioInput, GpioIn, GpioOut};
use multicaster::{self, Multicaster};
use std::{thread, time::Duration};
use tokio;

#[tokio::main]
async fn main() {
    //let multi = Multicaster::new("0.0.0.0", "239.0.0.20", 5007, true).await;
    let mut buzzer = gpio::sysfs::SysFsGpioOutput::open(27).unwrap();

    buzzer.set_high().unwrap();
    thread::sleep(Duration::from(time::Duration::from_millis(500)));
    buzzer.set_low().unwrap();
}
