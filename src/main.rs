use rdev::{listen, simulate, Event, EventType, Key, SimulateError};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread, time,
};

static PRESSED: AtomicBool = AtomicBool::new(false);

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

fn callback(event: Event) {
    // 300ms is close to the minimum interval allowed for two keyboard inputs in CS2.
    let delay = time::Duration::from_millis(300);
    let key_interval = time::Duration::from_millis(20);

    if PRESSED.load(Ordering::Relaxed) {
        PRESSED.store(false, Ordering::Relaxed);
        return;
    }

    match event.event_type {
        EventType::KeyRelease(key @ (Key::KeyA | Key::KeyD | Key::KeyS | Key::KeyW)) => {
            let opposite = match key {
                Key::KeyA => Key::KeyD,
                Key::KeyD => Key::KeyA,
                Key::KeyS => Key::KeyW,
                Key::KeyW => Key::KeyS,
                _ => unreachable!(),
            };
            thread::sleep(delay);
            send(&EventType::KeyPress(opposite));
            thread::sleep(key_interval);
            PRESSED.store(true, Ordering::Relaxed);
            send(&EventType::KeyRelease(opposite));
        }
        _ => (),
    }
}
