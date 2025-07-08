mod hotkey;
use native_windows_gui as nwg;
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
        
        // Focus the input field
        self.input.set_focus();
        
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
                
                // Extract actual command from display text
                let actual_command = if command.contains(" → ") {
                    command.split(" → ").nth(1).unwrap_or(command)
                } else if command.starts_with("Run: ") {
                    &command[5..]
                } else {
                    command
                };
                
                // Simple command execution
                if actual_command.starts_with("http") {
                    // Open URL
                    std::process::Command::new("cmd")
                        .args(&["/C", "start", actual_command])
                        .spawn()
                        .ok();
                } else if actual_command.ends_with(".exe") {
                    // Run executable
                    std::process::Command::new(actual_command)
                        .spawn()
                        .ok();
                } else {
                    // Try to run as command
                    std::process::Command::new("cmd")
                        .args(&["/C", actual_command])
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
        ("calculator", "calc.exe"),
        ("paint", "mspaint.exe"),
        ("cmd", "cmd.exe"),
        ("command", "cmd.exe"),
        ("powershell", "powershell.exe"),
        ("explorer", "explorer.exe"),
        ("chrome", "chrome.exe"),
        ("firefox", "firefox.exe"),
        ("edge", "msedge.exe"),
        ("task", "taskmgr.exe"),
        ("taskmanager", "taskmgr.exe"),
    ];
    
    // Common websites
    let websites = vec![
        ("google", "https://www.google.com"),
        ("youtube", "https://www.youtube.com"),
        ("github", "https://www.github.com"),
        ("stackoverflow", "https://stackoverflow.com"),
        ("reddit", "https://www.reddit.com"),
        ("twitter", "https://www.twitter.com"),
        ("facebook", "https://www.facebook.com"),
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

// Event handling structure
#[derive(Default)]
struct AppEvents {
    app: Option<Rc<RefCell<SearchBarApp>>>,
    last_input_text: String,
}

impl AppEvents {
    fn new(app: Rc<RefCell<SearchBarApp>>) -> Self {
        Self {
            app: Some(app),
            last_input_text: String::new(),
        }
    }
    
    fn handle_timer(&mut self) {
        if let Some(ref app) = self.app {
            let mut app_ref = app.borrow_mut();
            if let Some(ref receiver) = app_ref.hotkey_receiver {
                if receiver.try_recv().is_ok() {
                    println!("[DEBUG] Received hotkey signal");
                    drop(app_ref); // Release the mutable borrow
                    app.borrow().show_launcher();
                }
            }
        }
    }
    
    fn handle_input_change(&mut self) {
        if let Some(ref app) = self.app {
            let current_text = app.borrow().input.text();
            if current_text != self.last_input_text {
                self.last_input_text = current_text;
                app.borrow().handle_input_change();
            }
        }
    }
    
    fn handle_button_click(&self) {
        if let Some(ref app) = self.app {
            app.borrow().hide_launcher();
        }
    }
    
    fn handle_listbox_double_click(&self) {
        if let Some(ref app) = self.app {
            app.borrow().execute_command();
        }
    }
    
    fn handle_window_close(&self) {
        if let Some(ref app) = self.app {
            app.borrow().hide_launcher();
        }
        nwg::stop_thread_dispatch();
    }
    
    fn handle_key_press(&self, key_code: u32) {
        if key_code == 13 { // Enter key
            if let Some(ref app) = self.app {
                app.borrow().execute_command();
            }
        } else if key_code == 27 { // Escape key
            if let Some(ref app) = self.app {
                app.borrow().hide_launcher();
            }
        }
    }
}

fn main() {
    println!("[DEBUG] TotalControl starting...");
    
    nwg::init().expect("Failed to init Native Windows GUI");
    
    // Create font for better appearance
    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .family("Segoe UI")
        .size(16)
        .build(&mut font)
        .expect("Failed to create font");
    
    let mut app = SearchBarApp::default();
    
    // Create the main window (initially hidden)
    nwg::Window::builder()
        .size((500, 250))
        .position((300, 300))
        .title("TotalControl")
        .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
        .build(&mut app.window)
        .expect("Failed to create window");
    
    // Hide initially
    app.window.set_visible(false);
    
    // Create search input
    nwg::TextInput::builder()
        .parent(&app.window)
        .size((460, 35))
        .position((20, 20))
        .placeholder_text(Some("Type to search..."))
        .font(Some(&font))
        .build(&mut app.input)
        .expect("Failed to create input");
    
    // Create suggestions listbox
    nwg::ListBox::builder()
        .parent(&app.window)
        .size((460, 150))
        .position((20, 65))
        .font(Some(&font))
        .build(&mut app.listbox)
        .expect("Failed to create listbox");
    
    // Add default suggestion
    app.listbox.insert(0, "Press Ctrl+Space to activate".to_string());
    
    // Create close button
    nwg::Button::builder()
        .parent(&app.window)
        .size((80, 25))
        .position((400, 220))
        .text("Close")
        .font(Some(&font))
        .build(&mut app.close_button)
        .expect("Failed to create button");
    
    // Create animation timer for polling hotkey events
    nwg::AnimationTimer::builder()
        .parent(&app.window)
        .interval(std::time::Duration::from_millis(50))
        .build(&mut app.poll_timer)
        .expect("Failed to create timer");
    
    // Set up hotkey monitoring in separate thread
    let (tx, rx) = mpsc::channel();
    app.hotkey_receiver = Some(rx);
    
    thread::spawn(move || {
        unsafe {
            hotkey::register_hotkey_with_callback(move || {
                println!("[DEBUG] Hotkey pressed, sending signal");
                tx.send(()).ok();
            });
        }
    });
    
    let app_rc = Rc::new(RefCell::new(app));
    let mut events = AppEvents::new(app_rc.clone());
    
    // Start the timer
    app_rc.borrow().poll_timer.start();
    
    // Manual event loop using NWG's message dispatch
    let ui = nwg::dispatch_thread_events_with_callback(move || {
        // Check for timer events
        events.handle_timer();
        
        // Check for input changes
        events.handle_input_change();
        
        // Handle window messages manually
        unsafe {
            use windows::Win32::UI::WindowsAndMessaging::{PeekMessageA, MSG, PM_REMOVE, WM_KEYDOWN, WM_LBUTTONDBLCLK, WM_COMMAND};
            use windows::Win32::Foundation::HWND;
            
            let mut msg = MSG::default();
            while PeekMessageA(&mut msg, HWND(0), 0, 0, PM_REMOVE).as_bool() {
                match msg.message {
                    WM_KEYDOWN => {
                        events.handle_key_press(msg.wParam.0 as u32);
                    }
                    WM_LBUTTONDBLCLK => {
                        // Check if it's from the listbox
                        events.handle_listbox_double_click();
                    }
                    WM_COMMAND => {
                        // Check if it's from the button
                        events.handle_button_click();
                    }
                    _ => {}
                }
                
                windows::Win32::UI::WindowsAndMessaging::TranslateMessage(&msg);
                windows::Win32::UI::WindowsAndMessaging::DispatchMessageA(&msg);
            }
        }
    });
    
    println!("[DEBUG] UI built, starting message loop. Press Ctrl+Space to activate!");
}