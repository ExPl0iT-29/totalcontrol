mod hotkey;

slint::include_modules!();

fn main() {
    let ui = SearchBar::new().unwrap();

    // Optional: Provide a Rust implementation for the autoComplete callback.
    ui.on_auto_complete(|query| {
        vec![
            format!("Result A for {}", query),
            format!("Result B for {}", query),
        ]
    });

    unsafe {
        hotkey::register_hotkey();
    }

    ui.run().unwrap();
}





// Main file for the project
mod hotkey;
mod ui;
mod commands;
mod autocomplete;
mod config;
slint::include!("../src/ui.slint");

fn main() {
    println!("TotalControl Starting...");
    let ui = SearchBar::new().unwrap();
    unsafe {
        hotkey::register_hotkey();
    }
    ui.run().unwrap();
}



// hotkey.rs
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, HOT_KEY_MODIFIERS, MOD_CONTROL, VK_SPACE};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetMessageA, MSG};


pub unsafe fn register_hotkey() {
    // Fix type casting issues
    if RegisterHotKey(
        HWND(0), 
        1, 
        HOT_KEY_MODIFIERS(MOD_CONTROL.0), 
        VK_SPACE.0 as u32
    ).as_bool() == false {
        println!("Failed to register hotkey");
    }

    let mut msg = MSG::default();

    while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
        if msg.message == 786 { // WM_HOTKEY message
            println!("Ctrl + Space pressed!");
        }
    }
}
