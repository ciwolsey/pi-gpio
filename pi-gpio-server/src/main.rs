use core::time;
use std::thread;
use tokio;
use multicaster;
use gpio::{GpioIn, GpioOut};

#[tokio::main]
async fn main() {
    let mut alarm = gpio::sysfs::SysFsGpioInput::open(23).unwrap();
	let mut rain = gpio::sysfs::SysFsGpioInput::open(17).unwrap();

	loop {
		println!("Rain: {:?}", rain.read_value().unwrap());
		println!("Alarm: {:?}", alarm.read_value().unwrap());

		thread::sleep(time::Duration::from_millis(100));
	} 
}