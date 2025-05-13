use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, HOT_KEY_MODIFIERS, MOD_CONTROL, VK_SPACE
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetMessageA, MSG
};

pub unsafe fn register_hotkey() {
    // Register Ctrl + Space
    let ok = RegisterHotKey(
        HWND(0),
        1,
        HOT_KEY_MODIFIERS(MOD_CONTROL.0),
        VK_SPACE.0 as u32
    );

    if !ok.as_bool() {
        eprintln!("Failed to register hotkey for Ctrl+Space.");
        return;
    }

    let mut msg = MSG::default();
    while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
        // WM_HOTKEY = 0x0312 = 786 decimal
        if msg.message == 786 {
            println!("Ctrl + Space was pressed!");
            // In a real app, you could trigger the UI to focus here
        }
    }
}
