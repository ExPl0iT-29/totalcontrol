mod hotkey;
use native_windows_gui as nwg;
use nwg::NativeUi;
use std::sync::{Arc, Mutex};
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
    poll_timer: nwg::Timer,
}

impl SearchBarApp {
    fn show_launcher(&self) {
        println!("[DEBUG] Showing launcher window");
        
        // Clear previous input
        self.input.set_text("");
        
        // Position window in center of screen
        let screen_width = unsafe { windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CXSCREEN) };
        let screen_height = unsafe { windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CYSCREEN) };
        
        let window_width = 500;
        let window_height = 250;
        let x = (screen_width - window_width) / 2;
        let y = (screen_height - window_height) / 3; // Position in upper third
        
        self.window.set_position(x, y);
        self.window.set_size(window_width, window_height);
        
        // Show and bring to front
        self.window.set_visible(true);
        
        // Bring window to foreground and focus the input
        unsafe {
            use windows::Win32::UI::WindowsAndMessaging::{SetForegroundWindow, SetActiveWindow, SetFocus};
            use windows::Win32::Foundation::HWND;
            
            let hwnd = HWND(self.window.handle().hwnd().unwrap() as isize);
            SetForegroundWindow(hwnd);
            SetActiveWindow(hwnd);
            
            // Focus the input field
            let input_hwnd = HWND(self.input.handle().hwnd().unwrap() as isize);
            SetFocus(input_hwnd);
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
            .font(Some(&nwg::Font {
                family: "Segoe UI".to_string(),
                size: 14,
                weight: 400,
                decoration: nwg::FontDecoration::default(),
            }))
            .build(&mut data.input)?;
        
        // Create suggestions listbox
        nwg::ListBox::builder()
            .parent(&data.window)
            .size((460, 150))
            .position((20, 65))
            .build(&mut data.listbox)?;
        
        // Add default suggestion
        data.listbox.insert(0, "Press Ctrl+Space to activate".to_string());
        
        // Create close button
        nwg::Button::builder()
            .parent(&data.window)
            .size((80, 25))
            .position((400, 220))
            .text("Close")
            .build(&mut data.close_button)?;
        
        // Create timer for polling hotkey events
        nwg::Timer::builder()
            .parent(&data.window)
            .interval(50) // Check every 50ms
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
        
        // Set up event handlers
        let app_clone = app.clone();
        app.borrow().poll_timer.set_tick(move |_| {
            let mut app_ref = app_clone.borrow_mut();
            if let Some(ref receiver) = app_ref.hotkey_receiver {
                if receiver.try_recv().is_ok() {
                    println!("[DEBUG] Received hotkey signal");
                    app_ref.show_launcher();
                }
            }
        });
        
        let app_clone = app.clone();
        app.borrow().input.set_text_changed(move |_| {
            app_clone.borrow().handle_input_change();
        });
        
        let app_clone = app.clone();
        app.borrow().close_button.set_click(move |_| {
            app_clone.borrow().hide_launcher();
        });
        
        let app_clone = app.clone();
        app.borrow().listbox.set_double_click(move |_| {
            app_clone.borrow().execute_command();
        });
        
        // Handle Enter key in input
        let app_clone = app.clone();
        app.borrow().input.set_key_press(move |key| {
            if key.code == nwg::keys::RETURN {
                app_clone.borrow().execute_command();
            } else if key.code == nwg::keys::ESCAPE {
                app_clone.borrow().hide_launcher();
            }
        });
        
        // Handle window close
        let app_clone = app.clone();
        app.borrow().window.set_on_close(move |_| {
            app_clone.borrow().hide_launcher();
            nwg::stop_thread_dispatch();
        });
        
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