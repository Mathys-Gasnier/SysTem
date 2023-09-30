use std::sync::mpsc::SyncSender;
use std::thread;

use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

use crate::Message;

// Takes in the sender to send message to the main loop and returns hotkey handler thread
pub fn register_hotkeys(tx: SyncSender<Message>) -> thread::JoinHandle<()> {
    thread::spawn(|| {
        let mut hkm = HotkeyManager::new();
    
        // Register the hotkey ALT + W to send message to the main loop to open the app
        hkm.register(VKey::W, &[ModKey::Alt], move || {
            tx.send(Message::Open).unwrap();
        })
        .unwrap();
    
        // Start the event loop
        hkm.event_loop();
    })
}