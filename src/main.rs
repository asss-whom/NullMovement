use std::{thread, time::Duration};

use log::{debug, info};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, KEYEVENTF_KEYUP, VIRTUAL_KEY, VK_A, VK_D, VK_END, VK_S, VK_W, keybd_event,
};

const FPS: u64 = 60;

struct State {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
}

impl State {
    fn new(w: bool, a: bool, s: bool, d: bool) -> Self {
        Self { w, a, s, d }
    }

    fn from_now() -> Self {
        Self::new(
            get_async_key_state(VK_W),
            get_async_key_state(VK_A),
            get_async_key_state(VK_S),
            get_async_key_state(VK_D),
        )
    }
}

fn get_async_key_state(vkey: VIRTUAL_KEY) -> bool {
    unsafe {
        if GetAsyncKeyState(vkey.0 as i32) & i16::MIN == 0 {
            false
        } else {
            true
        }
    }
}

fn press(vkey: VIRTUAL_KEY) {
    unsafe {
        let duration = Duration::from_millis(5);
        keybd_event(vkey.0 as u8, 0, std::mem::zeroed(), 0);
        thread::sleep(duration);
        keybd_event(vkey.0 as u8, 0, KEYEVENTF_KEYUP, 0);
    }
}

fn finish(vkey: VIRTUAL_KEY) -> bool {
    unsafe {
        if GetAsyncKeyState(vkey.0 as i32) & i16::MIN == 0 {
            false
        } else {
            true
        }
    }
}

fn main() {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();
    info!("✅ Program initialized successfully.");
    info!("🛑 Press the \"End\" key to exit");
    let mut state = State::from_now();
    loop {
        counter_strafe(&mut state);
        if finish(VK_END) {
            break;
        }
        thread::sleep(Duration::from_millis(1000 / FPS));
    }
    info!("🎉 Program finished.");
}

fn counter_strafe(last_state: &mut State) {
    let now_state = State::from_now();
    let duration = Duration::from_millis(rand::random_range(50..150));

    if !now_state.w && last_state.w {
        debug!("🔼 The \"W\" key has been released.");
        thread::sleep(duration);
        press(VK_S);
        debug!("🔽 The \"S\" key has been released. ({:.?})", duration);
    }
    if !now_state.a && last_state.a {
        debug!("🔼 The \"A\" key has been released.");
        thread::sleep(duration);
        press(VK_D);
        debug!("🔽 The \"D\" key has been released. ({:.?})", duration);
    }
    if !now_state.s && last_state.s {
        debug!("🔼 The \"S\" key has been released.");
        thread::sleep(duration);
        press(VK_W);
        debug!("🔽 The \"W\" key has been released. ({:.?})", duration);
    }
    if !now_state.d && last_state.d {
        debug!("🔼 The \"D\" key has been released.");
        thread::sleep(duration);
        press(VK_A);
        debug!("🔽 The \"A\" key has been released. ({:.?})", duration);
    }

    *last_state = now_state;
}
