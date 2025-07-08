mod hotkey;
use native_windows_gui as nwg;
use nwg::NativeUi;
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::sync::mpsc;

#[derive(Default)]
pub struct SearchBarApp {
    window: nwg::Window,
    input: nwg::TextInput,
    listbox: nwg::ListBox<String>,
    close_button: nwg::Button,
    hotkey_receiver: Option<mpsc::Receiver<()>>,
    poll_timer: nwg::AnimationTimer,
}

impl SearchBarApp {
    fn show_launcher(&self) {
        println!("[DEBUG] Showing launcher window");
        
        // Clear previous input
        self.input.set_text("");
        
        // Position window in center of screen
        let screen_width = unsafe { 
            windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics(
                windows::Win32::UI::WindowsAndMessaging::SM_CXSCREEN
            ) 
        };
        let screen_height = unsafe { 
            windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics(
                windows::Win32::UI::WindowsAndMessaging::SM_CYSCREEN
            ) 
        };
        
        let window_width = 500u32;
        let window_height = 250u32;
        let x = ((screen_width - window_width as i32) / 2) as i32;
        let y = ((screen_height - window_height as i32) / 3) as i32; // Position in upper third
        
        self.window.set_position(x, y);
        self.window.set_size(window_width, window_height);
        
        // Show and bring to front
        self.window.set_visible(true);
        
        // Bring window to foreground
        unsafe {
            use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;
            use windows::Win32::Foundation::HWND;
            
            let hwnd = HWND(self.window.handle.hwnd().unwrap() as isize);
            SetForegroundWindow(hwnd);
        }
        
        println!("[DEBUG] Launcher window should now be visible and focused");
    }
    
    fn hide_launcher(&self) {
        println!("[DEBUG] Hiding launcher window");
        self.window.set_visible(false);
    }
    
    fn handle_input_change(&self) {
        let query = self.input.text();
        println!("[DEBUG] Input changed: '{}'", query);
        
        // Clear existing suggestions
        self.listbox.clear();
        
        if query.is_empty() {
            // Show default suggestions
            self.listbox.insert(0, "Type to search...".to_string());
            return;
        }
        
        // Simple autocomplete logic
        let suggestions = get_suggestions(&query);
        for (i, suggestion) in suggestions.iter().enumerate() {
            self.listbox.insert(i, suggestion.clone());
        }
        
        if suggestions.is_empty() {
            self.listbox.insert(0, format!("No results for '{}'", query));
        }
    }
    
    fn execute_command(&self) {
        let selected_index = self.listbox.selection();
        if let Some(index) = selected_index {
            if let Some(command) = self.listbox.collection().get(index) {
                println!("[DEBUG] Executing command: {}", command);
                
                // Simple command execution
                if command.starts_with("http") {
                    // Open URL
                    std::process::Command::new("cmd")
                        .args(&["/C", "start", command])
                        .spawn()
                        .ok();
                } else if command.ends_with(".exe") {
                    // Run executable
                    std::process::Command::new(command)
                        .spawn()
                        .ok();
                } else {
                    // Try to run as command
                    std::process::Command::new("cmd")
                        .args(&["/C", command])
                        .spawn()
                        .ok();
                }
                
                self.hide_launcher();
            }
        }
    }
}

// Simple suggestion system
fn get_suggestions(query: &str) -> Vec<String> {
    let mut suggestions = Vec::new();
    let query_lower = query.to_lowercase();
    
    // Common applications
    let apps = vec![
        ("notepad", "notepad.exe"),
        ("calc", "calc.exe"),
        ("paint", "mspaint.exe"),
        ("cmd", "cmd.exe"),
        ("powershell", "powershell.exe"),
        ("explorer", "explorer.exe"),
        ("chrome", "chrome.exe"),
        ("firefox", "firefox.exe"),
    ];
    
    // Common websites
    let websites = vec![
        ("google", "https://www.google.com"),
        ("youtube", "https://www.youtube.com"),
        ("github", "https://www.github.com"),
        ("stackoverflow", "https://stackoverflow.com"),
    ];
    
    // Match applications
    for (name, command) in apps {
        if name.contains(&query_lower) {
            suggestions.push(format!("{} → {}", name, command));
        }
    }
    
    // Match websites
    for (name, url) in websites {
        if name.contains(&query_lower) {
            suggestions.push(format!("{} → {}", name, url));
        }
    }
    
    // If no matches, suggest running as command
    if suggestions.is_empty() && !query.is_empty() {
        suggestions.push(format!("Run: {}", query));
    }
    
    suggestions
}

impl nwg::NativeUi<Rc<RefCell<SearchBarApp>>> for SearchBarApp {
    fn build_ui(mut data: SearchBarApp) -> Result<Rc<RefCell<SearchBarApp>>, nwg::NwgError> {
        // Create font for better appearance
        let mut font = nwg::Font::default();
        nwg::Font::builder()
            .family("Segoe UI")
            .size(16)
            .build(&mut font)?;
        
        // Create the main window (initially hidden)
        nwg::Window::builder()
            .size((500, 250))
            .position((300, 300))
            .title("TotalControl")
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
            .build(&mut data.window)?;
        
        // Hide initially
        data.window.set_visible(false);
        
        // Create search input
        nwg::TextInput::builder()
            .parent(&data.window)
            .size((460, 35))
            .position((20, 20))
            .placeholder_text(Some("Type to search..."))
            .font(Some(&font))
            .build(&mut data.input)?;
        
        // Create suggestions listbox
        nwg::ListBox::builder()
            .parent(&data.window)
            .size((460, 150))
            .position((20, 65))
            .font(Some(&font))
            .build(&mut data.listbox)?;
        
        // Add default suggestion
        data.listbox.insert(0, "Press Ctrl+Space to activate".to_string());
        
        // Create close button
        nwg::Button::builder()
            .parent(&data.window)
            .size((80, 25))
            .position((400, 220))
            .text("Close")
            .font(Some(&font))
            .build(&mut data.close_button)?;
        
        // Create animation timer for polling hotkey events
        nwg::AnimationTimer::builder()
            .parent(&data.window)
            .interval(std::time::Duration::from_millis(50))
            .build(&mut data.poll_timer)?;
        
        // Set up hotkey monitoring in separate thread
        let (tx, rx) = mpsc::channel();
        data.hotkey_receiver = Some(rx);
        
        thread::spawn(move || {
            unsafe {
                hotkey::register_hotkey_with_callback(move || {
                    println!("[DEBUG] Hotkey pressed, sending signal");
                    tx.send(()).ok();
                });
            }
        });
        
        let app = Rc::new(RefCell::new(data));
        
        // Set up event handlers using the correct NWG event system
        let app_clone = app.clone();
        let timer_handler = nwg::bind_event_handler(&app.borrow().poll_timer.handle, nwg::Event::OnTimerTick, move |_evt, _evt_data, _handle| {
            let mut app_ref = app_clone.borrow_mut();
            if let Some(ref receiver) = app_ref.hotkey_receiver {
                if receiver.try_recv().is_ok() {
                    println!("[DEBUG] Received hotkey signal");
                    app_ref.show_launcher();
                }
            }
        });
        
        let app_clone = app.clone();
        let input_handler = nwg::bind_event_handler(&app.borrow().input.handle, nwg::Event::OnTextInput, move |_evt, _evt_data, _handle| {
            app_clone.borrow().handle_input_change();
        });
        
        let app_clone = app.clone();
        let button_handler = nwg::bind_event_handler(&app.borrow().close_button.handle, nwg::Event::OnButtonClick, move |_evt, _evt_data, _handle| {
            app_clone.borrow().hide_launcher();
        });
        
        let app_clone = app.clone();
        let listbox_handler = nwg::bind_event_handler(&app.borrow().listbox.handle, nwg::Event::OnListBoxDoubleClick, move |_evt, _evt_data, _handle| {
            app_clone.borrow().execute_command();
        });
        
        // Handle key events on the input
        let app_clone = app.clone();
        let key_handler = nwg::bind_event_handler(&app.borrow().input.handle, nwg::Event::OnKeyPress, move |evt, evt_data, _handle| {
            if let nwg::EventData::OnKey(key_data) = evt_data {
                match key_data {
                    nwg::keys::RETURN => {
                        app_clone.borrow().execute_command();
                    },
                    nwg::keys::ESCAPE => {
                        app_clone.borrow().hide_launcher();
                    },
                    _ => {}
                }
            }
        });
        
        // Handle window close
        let app_clone = app.clone();
        let close_handler = nwg::bind_event_handler(&app.borrow().window.handle, nwg::Event::OnWindowClose, move |_evt, _evt_data, _handle| {
            app_clone.borrow().hide_launcher();
            nwg::stop_thread_dispatch();
        });
        
        // Start the timer
        app.borrow().poll_timer.start();
        
        Ok(app)
    }
}

fn main() {
    println!("[DEBUG] TotalControl starting...");
    
    nwg::init().expect("Failed to init Native Windows GUI");
    
    let app = SearchBarApp::default();
    let _ui = SearchBarApp::build_ui(app).expect("Failed to build UI");
    
    println!("[DEBUG] UI built, starting message loop. Press Ctrl+Space to activate!");
    nwg::dispatch_thread_events();
}