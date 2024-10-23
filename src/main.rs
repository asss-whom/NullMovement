use rdev::{listen, simulate, Event, EventType, Key, SimulateError};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread, time,
};

static A_PRESS: AtomicBool = AtomicBool::new(false);
static D_PRESS: AtomicBool = AtomicBool::new(false);
static S_PRESS: AtomicBool = AtomicBool::new(false);
static W_PRESS: AtomicBool = AtomicBool::new(false);

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
    // Just a random magic number
    let delay = time::Duration::from_millis(27);
    let last = time::Duration::from_millis(100);

    if event.event_type == EventType::KeyRelease(rdev::Key::KeyA) {
        if A_PRESS.load(Ordering::Relaxed) {
            A_PRESS.store(false, Ordering::Relaxed);
            return;
        }
        thread::sleep(delay);
        send(&EventType::KeyPress(Key::KeyD));
        D_PRESS.store(true, Ordering::Relaxed);
        thread::sleep(last);
        send(&EventType::KeyRelease(Key::KeyD));
    }
    if event.event_type == EventType::KeyRelease(rdev::Key::KeyD) {
        if D_PRESS.load(Ordering::Relaxed) {
            D_PRESS.store(false, Ordering::Relaxed);
            return;
        }
        thread::sleep(delay);
        send(&EventType::KeyPress(Key::KeyA));
        A_PRESS.store(true, Ordering::Relaxed);
        thread::sleep(last);
        send(&EventType::KeyRelease(Key::KeyA));
    }
    if event.event_type == EventType::KeyRelease(rdev::Key::KeyW) {
        if W_PRESS.load(Ordering::Relaxed) {
            W_PRESS.store(false, Ordering::Relaxed);
            return;
        }
        thread::sleep(delay);
        send(&EventType::KeyPress(Key::KeyS));
        S_PRESS.store(true, Ordering::Relaxed);
        thread::sleep(last);
        send(&EventType::KeyRelease(Key::KeyS));
    }
    if event.event_type == EventType::KeyRelease(rdev::Key::KeyS) {
        if S_PRESS.load(Ordering::Relaxed) {
            S_PRESS.store(false, Ordering::Relaxed);
            return;
        }
        thread::sleep(delay);
        send(&EventType::KeyPress(Key::KeyW));
        W_PRESS.store(true, Ordering::Relaxed);
        thread::sleep(last);
        send(&EventType::KeyRelease(Key::KeyW));
    }
}
