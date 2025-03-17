import logging
import random
import time

import win32api
import win32con
from rich.logging import RichHandler

FPS = 60  # frames
REACTION_TIME = 0.1  # seconds


class State:
    def __init__(self) -> None:
        self.w = bool(win32api.GetAsyncKeyState(0x57) & 0x8000)
        self.a = bool(win32api.GetAsyncKeyState(0x41) & 0x8000)
        self.s = bool(win32api.GetAsyncKeyState(0x53) & 0x8000)
        self.d = bool(win32api.GetAsyncKeyState(0x44) & 0x8000)

    def update(self, w: bool, a: bool, s: bool, d: bool) -> None:
        self.w = w
        self.a = a
        self.s = s
        self.d = d


def main() -> None:
    log.info("✅ Program initialized successfully.")
    log.info('🛑 Press the "End" key to exit')

    state = State()
    while True:
        counter_strafe(state)
        if finish():
            break
        time.sleep(1 / FPS)

    log.info("🎉 Program finished.")


def finish() -> bool:
    return bool(win32api.GetAsyncKeyState(0x23) & 0x8000)  # End


def counter_strafe(last_state: State) -> None:
    w = bool(win32api.GetAsyncKeyState(0x57) & 0x8000)
    a = bool(win32api.GetAsyncKeyState(0x41) & 0x8000)
    s = bool(win32api.GetAsyncKeyState(0x53) & 0x8000)
    d = bool(win32api.GetAsyncKeyState(0x44) & 0x8000)

    duration = REACTION_TIME + random.uniform(-0.05, 0.05)

    if not w and last_state.w:
        log.debug('🔼 The "W" key has been released.')
        time.sleep(duration)
        win32api.keybd_event(0x53, 0, 0, 0)
        time.sleep(0.05)
        win32api.keybd_event(0x53, 0, win32con.KEYEVENTF_KEYUP, 0)
        log.debug(f'🔽 The "S" key has been pressed ({duration * 1000:.2f}ms).')
    if not a and last_state.a:
        log.debug('🔼 The "A" key has been released.')
        time.sleep(duration)
        win32api.keybd_event(0x44, 0, 0, 0)
        time.sleep(0.05)
        win32api.keybd_event(0x44, 0, win32con.KEYEVENTF_KEYUP, 0)
        log.debug(f'🔽 The "D" key has been pressed ({duration * 1000:.2f}ms).')
    if not s and last_state.s:
        log.debug('🔼 The "S" key has been released.')
        time.sleep(duration)
        win32api.keybd_event(0x57, 0, 0, 0)
        time.sleep(0.05)
        win32api.keybd_event(0x57, 0, win32con.KEYEVENTF_KEYUP, 0)
        log.debug(f'🔽 The "W" key has been pressed ({duration * 1000:.2f}ms).')
    if not d and last_state.d:
        log.debug('🔼 The "D" key has been released.')
        time.sleep(duration)
        win32api.keybd_event(0x41, 0, 0, 0)
        time.sleep(0.05)
        win32api.keybd_event(0x41, 0, win32con.KEYEVENTF_KEYUP, 0)
        log.debug(f'🔽 The "A" key has been pressed ({duration * 1000:.2f}ms).')
    last_state.update(w, a, s, d)


if __name__ == "__main__":
    FORMAT = "%(message)s"
    logging.basicConfig(
        level="NOTSET",
        format="%(message)s",
        datefmt="[%X]",
        handlers=[RichHandler(rich_tracebacks=True, tracebacks_show_locals=True)],
    )
    log = logging.getLogger("rich")

    try:
        main()
    except Exception as e:
        log.exception(e)
