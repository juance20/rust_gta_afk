#![windows_subsystem = "windows"]

use rdev::{listen, simulate, Button, Event, EventType, Key};
use std::{thread, time};
use notify_rust::Notification;
use tray_icon::{menu::{Menu, PredefinedMenuItem}, Icon, TrayIcon, TrayIconBuilder};

static mut AFK_TIME: i32 = 0;

fn main() {
    println!("dwajkv dkwajvd");
    let icono: Icon = Icon::from_path(std::path::Path::new("C:/Users/juan/Desktop/Informática/Programacion/rust_gta_afk/target/debug/icon.ico"), None).ok().unwrap();

    let my_menu: Menu = Menu::new();

    let _ = my_menu.append(&PredefinedMenuItem::quit(Some("Terminar")));

    let _tray_icon: TrayIcon = TrayIconBuilder::new()
        .with_tooltip("Anti-AFK")
        .with_icon(icono)
        .with_menu(Box::new(my_menu))
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

    if let Err(error) = listen(listener) {
        println!("Error: {:?}", error);
    }
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