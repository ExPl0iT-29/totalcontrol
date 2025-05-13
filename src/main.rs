use native_windows_gui as nwg;
use nwg::NativeUi;
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, MOD_CONTROL, VIRTUAL_KEY};
use windows::Win32::Foundation::HWND;

#[derive(Default)]
pub struct SearchBarApp {
    window: nwg::Window,
    input: nwg::TextInput,
    listbox: nwg::ListBox<String>,
    hotkey: Option<nwg::GlobalHotKey>, // Store the registered hotkey
}

impl nwg::NativeUi<SearchBarApp> for SearchBarApp {
    fn build_ui(mut data: SearchBarApp) -> Result<SearchBarApp, nwg::NwgError> {
        // 1) Create the main window
        nwg::Window::builder()
            .size((400, 200))
            .position((300, 300))
            .title("TotalControl - NWG")
            .build(&mut data.window)?;

        // Hide the window by default
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

        // 4) Bind an event handler to detect text changes
        let input_handle = data.input.handle;
        nwg::bind_event_handler(
            &data.input.handle,
            &data.window.handle,
            move |evt, _evt_data, handle| {
                if evt == nwg::Event::OnTextInput && handle == input_handle {
                    let text = nwg::ControlHandle::from(handle).text().unwrap_or_default();
                    println!("User typed: {}", text);
                }
            },
        );

        Ok(data)
    }
}

impl SearchBarApp {
    /// Store the hotkey in our app state and bind an event handler to show/hide the window.
    pub fn set_hotkey(&mut self, hotkey: nwg::GlobalHotKey) {
        self.hotkey = Some(hotkey);

        // Toggle the window’s visibility when the hotkey is pressed.
        let window_handle = self.window.handle;
        let hotkey_handle = self.hotkey.as_ref().unwrap().handle;
        nwg::bind_event_handler(
            &hotkey_handle,
            &window_handle,
            move |evt, _evt_data, handle| {
                if evt == nwg::Event::OnGlobalHotkey && handle == hotkey_handle {
                    let window_ctrl = nwg::ControlHandle::from(window_handle);
                    if let Some(win) = nwg::Window::from_handle(&window_ctrl) {
                        let visible = win.visible();
                        win.set_visible(!visible);
                        if !visible {
                            win.set_focus();
                        }
                    }
                }
            },
        );
    }
}

fn main() {
    // Initialize NWG
    nwg::init().expect("Failed to init Native Windows GUI");

    // // Set a global default font (optional, but recommended)
    // let default_font = nwg::Font::builder()
    //     .family("Segoe UI")
    //     .size(17)
    //     .build()
    //     .expect("Failed to build default font");
    // nwg::Font::set_global_default(Some(default_font));

    // Build the main UI
    let mut app = SearchBarApp::build_ui(Default::default())
        .expect("Failed to build UI");

    // Register a global Ctrl+Space hotkey
    let hk = nwg::GlobalHotKey::new(
        Some("Ctrl+Space"),
        &nwg::HotKeyModifiers::CTRL,
        nwg::keys::VK_SPACE as u32
    )
    .expect("Failed to create global hotkey");
    app.set_hotkey(hk);

    // Keep the app alive
    std::mem::forget(app);

    // Enter the NWG event loop
    nwg::dispatch_thread_events();
}
