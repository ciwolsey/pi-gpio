use core::time;
use gpio::{sysfs::SysFsGpioInput, GpioIn, GpioOut};
use multicaster;
use std::thread;
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

#[tokio::main]
async fn main() {
    let mut pins = Pins::new();

    let rain = pins.check(Pin::Rain);
    let alarm = pins.check(Pin::Alarm);

    match rain {
        Some(change_to) => println!("Rain value changed to: {}", change_to),
        None => (),
    }

    match alarm {
        Some(change_to) => println!("Alarm value changed to: {}", change_to),
        None => (),
    }
}
