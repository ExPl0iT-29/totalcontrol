use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS, MOD_CONTROL, VK_SPACE
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetMessageA, MSG, WM_HOTKEY
};

pub unsafe fn register_hotkey() {
    println!("[DEBUG] Registering Ctrl+Space hotkey...");
    
    // Register Ctrl + Space
    let result = RegisterHotKey(
        HWND(0),
        1,
        HOT_KEY_MODIFIERS(MOD_CONTROL.0),
        VK_SPACE.0 as u32
    );

    if !result.as_bool() {
        eprintln!("[ERROR] Failed to register hotkey for Ctrl+Space.");
        return;
    }

    println!("[DEBUG] Hotkey registered successfully. Listening for Ctrl+Space...");

    let mut msg = MSG::default();
    while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
        if msg.message == WM_HOTKEY {
            println!("[DEBUG] Ctrl + Space was pressed!");
            // In the new implementation, this will be handled by the callback
        }
    }
}

pub unsafe fn register_hotkey_with_callback<F>(callback: F) 
where 
    F: Fn() + Send + 'static 
{
    println!("[DEBUG] Registering Ctrl+Space hotkey with callback...");
    
    // Unregister any existing hotkey first
    UnregisterHotKey(HWND(0), 1);
    
    // Register Ctrl + Space
    let result = RegisterHotKey(
        HWND(0),
        1,
        HOT_KEY_MODIFIERS(MOD_CONTROL.0),
        VK_SPACE.0 as u32
    );

    if !result.as_bool() {
        eprintln!("[ERROR] Failed to register hotkey for Ctrl+Space.");
        return;
    }

    println!("[DEBUG] Hotkey registered successfully. Listening for Ctrl+Space...");

    let mut msg = MSG::default();
    while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
        if msg.message == WM_HOTKEY && msg.wParam.0 == 1 {
            println!("[DEBUG] Hotkey detected, calling callback");
            callback();
        }
    }
}