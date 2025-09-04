use std::io;
enum PowerState {
    Off,
    On,
    Shutdown,
}

impl PowerState {
    fn new(s: &str) -> Option<PowerState> {
        let state = s.trim().to_lowercase();
        match state.as_str() {
            "off" => Some(PowerState::Off),
            "on" => Some(PowerState::On),
            "shutdown" => Some(PowerState::Shutdown),
            _ => None,
        }
    }
}

fn print_power_state(state: &PowerState) {
    match state {
        PowerState::Off => {
            println!("The computer is off");
        }
        PowerState::On => {
            println!("The computer is on");
        }
        PowerState::Shutdown => {
            println!("The computer is shutting down");
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let read_result = io::stdin().read_line(&mut buffer);
    match read_result {
        Ok(data) => {
            println!("data={}, buffer={}", data, buffer);
            let power_state = PowerState::new(buffer.as_str());
            match power_state {
                Some(ps) => {
                    print_power_state(&ps);
                }
                None => {
                    println!("Invalid input");
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
