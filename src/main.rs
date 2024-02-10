#![windows_subsystem = "windows"]

use rdev::{listen, simulate, Button, Event, EventType, Key};
use std::{thread, time};
use notify_rust::Notification;

static mut RUNNING: bool = false;

fn main() {

    let _ = Notification::new()
    .summary("Modo de uso")
    .body("F9: Pausar\nF10: Iniciar\nF12: Terminar")
    .show();

    thread::spawn(|| {
        loop {
            if unsafe { RUNNING } == true{
                send(&EventType::ButtonPress(Button::Right));
                thread::sleep(time::Duration::from_millis(200));
                send(&EventType::ButtonRelease(Button::Right));
            }
            thread::sleep(time::Duration::from_secs(2));
        }
    });

    let _ = listen(listener);
}

fn listener(event: Event) {
    let ev: EventType = event.event_type;
    if ev == EventType::KeyPress(Key::F9) {
        unsafe { RUNNING = false };
    }
    if ev == EventType::KeyPress(Key::F10) {
        unsafe { RUNNING = true };
    }
    if ev == EventType::KeyPress(Key::F12) {
        let _ =Notification::new()
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
