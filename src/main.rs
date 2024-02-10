#![windows_subsystem = "windows"]

use rdev::{listen, simulate, Button, Event, EventType, Key};
use std::{thread, time};
use notify_rust::Notification;

static mut AFK_TIME: i32 = 0;

fn main() {

    let _ = Notification::new()
    .summary("Modo de uso")
    .body("Actuará automáticamente cuando se detecte inactividad\nF12: Terminar")
    .show();

    thread::spawn(|| {
        loop {
            unsafe { AFK_TIME = AFK_TIME + 1 };
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    thread::spawn(|| {
        loop {
            if unsafe { AFK_TIME >= 70 } {
                send(&EventType::ButtonPress(Button::Right));
                thread::sleep(time::Duration::from_millis(200));
                send(&EventType::ButtonRelease(Button::Right));
            }
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    let _ = listen(listener);
}

fn listener(event: Event) {
    let ev: EventType = event.event_type;
    unsafe { AFK_TIME = 0 };
    if ev != EventType::KeyPress(Key::F12) {
        return;
    }
    else {        
        let _ = Notification::new()
        .body("Proceso terminado")
        .show();
        std::process::exit(0);

    }
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    let _ = simulate(event_type);
    thread::sleep(delay);
}
