mod hotkey;
use native_windows_gui as nwg;
use nwg::NativeUi;
use windows::Win32::Foundation::{HWND, WPARAM, LPARAM};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
pub struct SearchBarApp {
    window: nwg::Window,
    input: nwg::TextInput,
    listbox: nwg::ListBox<String>,
    runner: nwg::Window,
    poll_timer: nwg::Timer,
    runner_bg: nwg::RichLabel,
}

impl nwg::NativeUi<Rc<RefCell<SearchBarApp>>> for SearchBarApp {
    fn build_ui(mut data: SearchBarApp) -> Result<Rc<RefCell<SearchBarApp>>, nwg::NwgError> {
        // 1) Create the main window
        nwg::Window::builder()
            .size((400, 200))
            .position((300, 300))
            .title("TotalControl - NWG")
            .build(&mut data.window)?;

        // 1.5) Create the runner box (hidden by default)
        nwg::Window::builder()
            .size((300, 60))
            .position((350, 220)) // Position above the main window
            .title("")
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE | nwg::WindowFlags::POPUP)
            .build(&mut data.runner)?;
        data.runner.set_visible(false);

        // Add a RichLabel as a grey background
        nwg::RichLabel::builder()
            .text("")
            .parent(&data.runner)
            .size((300, 60))
            .position((0, 0))
            .background_color(Some([200, 200, 200])) // light grey
            .build(&mut data.runner_bg)?;

        // Add a label to the runner box for visibility
        let mut runner_label = nwg::Label::default();
        nwg::Label::builder()
            .text("Runner Box")
            .parent(&data.runner)
            .size((280, 40))
            .position((10, 10))
            .build(&mut runner_label)?;

        // Hide the main window by default
        data.window.set_visible(false);

        // 2) Create a text input for the search query
        nwg::TextInput::builder()
            .parent(&data.window)
            .size((380, 30))
            .position((10, 10))
            .placeholder_text(Some("Search...")) 
            .build(&mut data.input)?;

        // 3) Create a list box to show suggestions
        nwg::ListBox::builder()
            .parent(&data.window)
            .size((380, 120))
            .position((10, 50))
            .build(&mut data.listbox)?;

        // Add some dummy suggestions
        data.listbox.insert(0, "First Suggestion".to_string());
        data.listbox.insert(1, "Second Suggestion".to_string());

        nwg::Timer::builder()
            .interval(100)
            .parent(&data.runner)
            .build(&mut data.poll_timer)?;
        Ok(Rc::new(RefCell::new(data)))
    }
}

fn main() {
    println!("[DEBUG] App starting...");
    // Initialize NWG (optional, can be removed if not using NWG)
    // nwg::init().expect("Failed to init Native Windows GUI");

    // Register a global Ctrl+Space hotkey and print a message when pressed
    unsafe {
        use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, HOT_KEY_MODIFIERS, MOD_CONTROL, VK_SPACE};
        use windows::Win32::UI::WindowsAndMessaging::{GetMessageA, MSG};
        use windows::Win32::Foundation::HWND;
        let ok = RegisterHotKey(
            HWND(0),
            1,
            HOT_KEY_MODIFIERS(MOD_CONTROL.0),
            VK_SPACE.0 as u32
        );
        if !ok.as_bool() {
            eprintln!("[DEBUG] Failed to register hotkey for Ctrl+Space.");
        } else {
            println!("[DEBUG] Hotkey registered (Ctrl+Space)");
            let mut msg = MSG::default();
            while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
                if msg.message == 786 {
                    println!("Ctrl+Space was pressed!");
                }
            }
        }
    }
}
