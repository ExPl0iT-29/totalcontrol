Here's a clean and beginner-friendly `CONTRIBUTING.md` file for **TotalControl**, tailored to your project under **HSSoC**, with clear instructions for both code and non-code contributors:

---

````markdown
# 🤝 Contributing to TotalControl

Welcome, contributor! 🚀

Thank you for considering contributing to **TotalControl** — a Rust-based global command launcher developed as part of [HashSlap Summer of Code (HSSoC)](https://hashslap.github.io/hssoc).

We’re building a powerful, keyboard-first productivity tool for Windows — and you can help shape it!

---

## 📌 Before You Start

- Make sure you've ⭐ starred the repo
- Read the [README](./README.md) to understand the project goal
- Join our **Discord** (mandatory): Link on [HSSoC site](https://hashslap.github.io/hssoc)
- Read this guide fully before submitting a PR

---

## 🛠 Project Setup

1. **Fork** this repository
2. **Clone your fork:**
   ```bash
   git clone https://github.com/<your-username>/totalcontrol.git
   cd totalcontrol
````

3. **Build & Run:**

   ```bash
   cargo run
   ```

4. Press `Ctrl + Space` to test global hotkey functionality

---

## 🧠 Areas You Can Contribute

### 🔧 Code Contributions (Rust)

* Implement command autocomplete logic
* Improve hotkey handling
* UI enhancements with [Slint](https://slint.dev/)
* Add support for `commands.json` configuration
* Improve performance or error handling

### 🖌 UI/UX Enhancements

* Suggest & design blur effects (Windows Acrylic)
* Help polish search bar layout and animations
* Add themes (light/dark mode toggle)

### 📝 Non-Code Contributions

* Improve documentation
* Create issues with feature ideas
* Help write `commands.json` examples
* Create demo GIFs/videos

---

## 🧾 Pull Request Guidelines

* Link your PR to an existing issue (or create one)
* Write **clear commit messages** (`feat:`, `fix:`, `docs:` etc.)
* Test your changes locally before opening PR
* Only touch files relevant to your changes

---

## 🧪 Testing Tips

* Run the app using `cargo run`
* Log debug messages using `println!()` or `tracing`
* Check Windows terminal output when using hotkey (Ctrl + Space)

---

## 💬 Get Support

* Ask in the **Discord community**
* Open an issue with the `question` label
* Mention maintainers if urgent

---

Made with 💙 by the **HSSoC Community**

---