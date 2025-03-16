import logging
import random
import time

import win32api
import win32con
from rich.logging import RichHandler

FPS = 60  # frames
REACTION_TIME = 0.2  # seconds


def main():
    log.info("✅ Program initialized successfully.")
    log.info('🛑 Press the "Alt" key to exit')

    w = bool(win32api.GetAsyncKeyState(0x57) & 0x8000)
    a = bool(win32api.GetAsyncKeyState(0x41) & 0x8000)
    s = bool(win32api.GetAsyncKeyState(0x53) & 0x8000)
    d = bool(win32api.GetAsyncKeyState(0x44) & 0x8000)
    while True:
        w, a, s, d = event_loop(w, a, s, d)
        if finish():
            break

    log.info("🎉 Program finished.")


def finish() -> bool:
    return bool(win32api.GetAsyncKeyState(0x12) & 0x8000)  # Alt


def event_loop(
    last_w: bool,
    last_a: bool,
    last_s: bool,
    last_d: bool,
) -> tuple[bool, bool, bool, bool]:
    w = bool(win32api.GetAsyncKeyState(0x57) & 0x8000)
    a = bool(win32api.GetAsyncKeyState(0x41) & 0x8000)
    s = bool(win32api.GetAsyncKeyState(0x53) & 0x8000)
    d = bool(win32api.GetAsyncKeyState(0x44) & 0x8000)

    duration = random.normalvariate(REACTION_TIME, 0.05)

    if not w and last_w:
        log.debug('🔼 The "W" key has been released.')
        time.sleep(duration)
        win32api.keybd_event(0x53, 0, 0, 0)
        time.sleep(0.03)
        win32api.keybd_event(0x53, 0, win32con.KEYEVENTF_KEYUP, 0)
        log.debug(f'🔽 The "S" key has been pressed ({duration * 1000:.2f}ms).')
    if not a and last_a:
        log.debug('🔼 The "A" key has been released.')
        time.sleep(duration)
        win32api.keybd_event(0x44, 0, 0, 0)
        time.sleep(0.03)
        win32api.keybd_event(0x44, 0, win32con.KEYEVENTF_KEYUP, 0)
        log.debug(f'🔽 The "D" key has been pressed ({duration * 1000:.2f}ms).')
    if not s and last_s:
        log.debug('🔼 The "S" key has been released.')
        time.sleep(duration)
        win32api.keybd_event(0x57, 0, 0, 0)
        time.sleep(0.03)
        win32api.keybd_event(0x57, 0, win32con.KEYEVENTF_KEYUP, 0)
        log.debug(f'🔽 The "W" key has been pressed ({duration * 1000:.2f}ms).')
    if not d and last_d:
        log.debug('🔼 The "D" key has been released.')
        time.sleep(duration)
        win32api.keybd_event(0x41, 0, 0, 0)
        time.sleep(0.03)
        win32api.keybd_event(0x41, 0, win32con.KEYEVENTF_KEYUP, 0)
        log.debug(f'🔽 The "A" key has been pressed ({duration * 1000:.2f}ms).')
    time.sleep(1 / FPS)
    return w, a, s, d


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
