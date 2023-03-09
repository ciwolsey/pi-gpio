use core::time;
use gpio::{sysfs::SysFsGpioInput, GpioIn};
use multicaster::{self, Multicaster};
use std::{thread, time::Duration};
use tokio;

struct Pins {
    alarm: SysFsGpioInput,
    alarm_last: bool,
    rain: SysFsGpioInput,
    rain_last: bool,
}

impl Pins {
    fn new() -> Self {
        let mut rain = gpio::sysfs::SysFsGpioInput::open(17).unwrap();
        let mut alarm = gpio::sysfs::SysFsGpioInput::open(23).unwrap();
        let rain_val = rain.read_value().unwrap().into();
        let alarm_val = alarm.read_value().unwrap().into();

        Self {
            alarm,
            rain,
            alarm_last: alarm_val,
            rain_last: rain_val,
        }
    }

    /// Alarm value: true when door is shut
    fn check(&mut self, pin: Pin) -> Option<bool> {
        match pin {
            Pin::Rain => {
                let current: bool = self.rain.read_value().unwrap().into();
                let changed = current != self.rain_last;
                self.rain_last = current;
                if changed {
                    Some(current)
                } else {
                    None
                }
            }
            Pin::Alarm => {
                let current: bool = self.alarm.read_value().unwrap().into();
                let changed = current != self.alarm_last;
                self.alarm_last = current;
                if changed {
                    Some(current)
                } else {
                    None
                }
            }
        }
    }
}

enum Pin {
    Rain,
    Alarm,
}

fn wait_for_gpio() {
    loop {
        if let Ok(_) = gpio::sysfs::SysFsGpioOutput::open(2) {
            println!("GPIO ready.");
            return;
        }
        println!("Failed to open GPIO, retrying in 5 seconds.");
        thread::sleep(Duration::from(time::Duration::from_millis(5000)));
    };
}

#[tokio::main]
async fn main() {
    wait_for_gpio();
    let mut pins = Pins::new();
    let multi = Multicaster::new("0.0.0.0", "239.0.0.20", 5007, true).await;

    loop {
        thread::sleep(Duration::from(time::Duration::from_millis(20)));

        match pins.check(Pin::Rain) {
            Some(changed_to) => {
                println!("Rain value changed to: {}", changed_to);
                match changed_to {
                    true => multi.send(String::from("rain: true").as_bytes()).await,
                    false => multi.send(String::from("rain: false").as_bytes()).await,
                }
            }
            _ => (),
        }

        match pins.check(Pin::Alarm) {
            Some(changed_to) => {
                println!("Alarm value changed to: {}", changed_to);
                match changed_to {
                    true => multi.send(String::from("door: closed").as_bytes()).await,
                    false => multi.send(String::from("door: opened").as_bytes()).await,
                }
            }
            _ => (),
        }
    }
}
