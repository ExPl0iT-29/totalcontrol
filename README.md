# ⚙️ TotalControl

> A blazing-fast Windows productivity tool that brings Mac Spotlight-like power to your keyboard.

<p align="center">
  <img src="https://img.shields.io/github/license/ExPl0iT-29/totalcontrol?style=for-the-badge&color=brightgreen" />
  <img src="https://img.shields.io/github/stars/ExPl0iT-29/totalcontrol?style=for-the-badge&color=blue" />
  <img src="https://img.shields.io/github/issues/ExPl0iT-29/totalcontrol?style=for-the-badge&color=yellow" />
  <img src="https://img.shields.io/badge/Made%20for-HSSoC-ff69b4?style=for-the-badge" />
</p>

---

## 🚀 What is TotalControl?

**TotalControl** is a modern, Rust-based desktop app that enables global hotkey-triggered search and command execution on **Windows**.

Press `Ctrl + Space` to launch a slick, semi-transparent search bar with **autocomplete**, where you can run:

* 🖥️ Desktop applications
* 🐚 Scripts or terminal commands
* 🌐 Websites
* 🔧 Your own custom workflows

> Built to boost productivity by letting you control your system without touching the mouse.

---

## ✨ Features

* ⌨️ **Global Hotkey Activation** (Ctrl + Space)
* 🔍 **Search Bar with Autocomplete** (Slint-powered UI)
* 💻 **Run Apps, Scripts, URLs** via commands
* 💾 **JSON-based Config File** for user-defined commands
* 🌈 **UI Customization** (themes, hotkey editing) *(upcoming)*
* 🪟 **Windows-Acrylic Blur** *(upcoming)*

---

## 📦 Project Structure

```
totalcontrol/
├── src/
│   ├── main.rs         # App entrypoint
│   ├── hotkey.rs       # Global hotkey listener
│   ├── ui.slint        # Slint-based UI layout
│   ├── commands.rs     # (Planned) Run apps/scripts/URLs
│   ├── autocomplete.rs # (Planned) Search & suggestions
│   ├── config.rs       # (Planned) Command storage via JSON
├── build.rs            # Slint UI build script
├── Cargo.toml          # Rust dependencies
```

---

## 📈 Current Progress

✅ **Hotkey Listener**
✅ **Basic Search UI**
🔄 **Command Execution** (in progress)
🔄 **Autocomplete Engine**
🔄 **Config System (commands.json)**
🔄 **Acrylic Blur & UI Polish**

---

## 🔧 Getting Started

### 🛠 Prerequisites

* [Rust (stable)](https://www.rust-lang.org/tools/install)
* Windows 10 or 11

### 📥 Clone & Run

```bash
git clone https://github.com/ExPl0iT-29/totalcontrol.git
cd totalcontrol
cargo run
```

Press `Ctrl + Space` to trigger the launcher and type your command!

---

## 🛣️ Roadmap

* [ ] Add Acrylic Blur & dark/light mode UI toggle
* [ ] Autocomplete via Trie/Tantivy
* [ ] Command Execution (open apps, shell scripts, websites)
* [ ] `commands.json` for configuration
* [ ] Global hotkey customization
* [ ] Performance & background mode optimization
* [ ] Cross-platform support (long-term goal)

---

## 🧠 Why TotalControl?

> Most OSes don’t make automation and keyboard-first actions easy. TotalControl solves that.

* Run anything from your keyboard
* No more Start menu or shortcuts
* Make your own command launcher
* Fully customizable, no vendor lock-in

---

## 🤝 Contributing

This project is part of **[HashSlap Summer of Code (HSSoC)](https://hashslap.github.io/hssoc)** — an open source initiative to help developers build real-world tools.

We welcome contributions in:

* Rust development (hotkey, autocomplete, async handling)
* UX/UI design improvements
* Command runners and integrations

---

## 📜 License

Licensed under the [MIT License](LICENSE)

---

### 🙌 Built with ❤️ for the Developer Community

**TotalControl** is proudly maintained under the **HashSlap Summer of Code (HSSoC)** banner.

Let’s make Windows automation seamless — one shortcut at a time.
🌐 [GitHub](https://github.com/ExPl0iT-29/totalcontrol) • 🔗 [HSSoC Website](https://hashslap.github.io/hssoc)

---
