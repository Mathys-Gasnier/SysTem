use std::{sync::mpsc, thread};
use tray_item::{IconSource, TrayItem};

mod app;
mod hotkey;
mod modules;

use crate::hotkey::register_hotkeys;
use crate::app::start_app;

// Main loop messages enum
pub enum Message {
    Open,
    Quit
}

fn main() {

    // Register the tray item with it's name and icon (loaded from sys-tem.rc)
    let mut tray = TrayItem::new(
        "SysTem",
        IconSource::Resource("icon"),
    ).unwrap();

    // Main loop message sender and receiver
    let (tx, rx) = mpsc::sync_channel(1);

    // System tray button to open the app
    let open_tx = tx.clone();
    tray.add_menu_item("Open", move || {
        open_tx.send(Message::Open).unwrap();
    }).unwrap();

    // System tray button to close the app
    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    }).unwrap();

    // Creating the thread responsible for hotkeys
    let hotkey_tx = tx.clone();
    let hotkeys_thread = register_hotkeys(hotkey_tx);

    // Variable used to store the running app thread
    let mut raylib_thread: Option<thread::JoinHandle<()>> = None;

    println!("Launched");
    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                break;
            },
            Ok(Message::Open) => {
                // If the thread exists and it isn't finish then we don't answer to open messages
                if let Some(thread) = &raylib_thread {
                    if !thread.is_finished() {
                        continue;
                    }
                }
                // Open the app
                raylib_thread = Some(start_app());
            },
            _ => {}
        }
    }

    hotkeys_thread.join().unwrap();
}
