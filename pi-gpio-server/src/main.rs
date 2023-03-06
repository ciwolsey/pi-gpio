use core::time;
use std::thread;
use tokio;
use multicaster;
use gpio::{GpioIn, GpioOut};

#[tokio::main]
async fn main() {
    let mut gpio23 = gpio::sysfs::SysFsGpioInput::open(23).unwrap();
	let mut gpio17 = gpio::sysfs::SysFsGpioInput::open(17).unwrap();

	loop {
		println!("GPIO17: {:?}", gpio17.read_value().unwrap());
		println!("GPIO23: {:?}", gpio23.read_value().unwrap());
		thread::sleep(time::Duration::from_millis(100));
	} 
}
