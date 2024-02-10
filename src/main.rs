#![windows_subsystem = "windows"]

use rdev::{listen, simulate, Button, Event, EventType, Key};
use std::{thread, time};
use notify_rust::Notification;
use tray_icon::{Icon, TrayIconBuilder};

static mut AFK_TIME: i32 = 0;

fn main() {

    let icono: Icon = Icon::from_path(std::path::Path::new("C:/Users/juan/Desktop/Informática/Programacion/rust_gta_afk/target/debug/icon.ico"), None).ok().unwrap();

    let _tray_icon = TrayIconBuilder::new()
        .with_tooltip("Anti-AFK")
        .with_icon(icono)
        .with_menu_on_left_click(true)
        .build()
        .unwrap();

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
        terminar();
    }
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    let _ = simulate(event_type);
    thread::sleep(delay);
}

fn terminar(){
    let _ = Notification::new()
    .body("Proceso terminado")
    .show();
    std::process::exit(0);
}