use core::time;
use gpio::{sysfs::SysFsGpioOutput, GpioOut};
use multicaster::{self, Multicaster};
use std::{thread, time::Duration};
use tokio;

struct Buzzer {
    gpio: SysFsGpioOutput,
}

impl Buzzer {
    fn new() -> Self {
        Self {
            gpio: gpio::sysfs::SysFsGpioOutput::open(27).unwrap(),
        }
    }
    fn beep(&mut self, duration: u64) {
        self.gpio.set_high().unwrap();
        thread::sleep(Duration::from(time::Duration::from_millis(duration)));
        self.gpio.set_low().unwrap();
    }
}

fn wait_for_gpio() {
    loop {
        if let Ok(_) = gpio::sysfs::SysFsGpioOutput::open(2) {
            println!("Failed to open GPIO, retrying in 5 seconds.");
            return;
        }
        println!("Failed to open GPIO, retrying in 5 seconds.");
        thread::sleep(Duration::from(time::Duration::from_millis(5000)));
    };
}

#[tokio::main]
async fn main() {
    // Need to wait for gpio to become avilable
    wait_for_gpio();

    let mut buzzer = Buzzer::new();
    buzzer.beep(500);

    let multi = Multicaster::new("0.0.0.0", "239.0.0.20", 5007, true).await;
    let mut buf: [u8; 256] = [0x00; 256];

    loop {
        buf.fill(0x00);
        multi.rec(&mut buf).await;

        let message = String::from_utf8(buf.to_vec()).unwrap();
        println!("Received: {}", message);

        if message.contains("door: opened") {
            buzzer.beep(3000);
        }

        if message.contains("rain: true") {
            buzzer.beep(300);
            thread::sleep(Duration::from(time::Duration::from_millis(200)));
            buzzer.beep(300);
            thread::sleep(Duration::from(time::Duration::from_millis(200)));
            buzzer.beep(300);
            thread::sleep(Duration::from(time::Duration::from_millis(200)));
            buzzer.beep(300);
        }
    }
}
