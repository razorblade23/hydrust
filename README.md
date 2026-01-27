# 🌊 Hydrust 

**The Modular, High-Performance Media Archiving Engine.** *Built in Rust. Powered by WebAssembly. Driven by the Community.*

---

> [!WARNING]
> This is a work in progress and for now only a placeholder on name for now.

## 🚀 What is Hydrust?

Hydrust is not just another video downloader. It is a **native media orchestration platform** designed to solve the "maintenance trap" of modern streaming scrapers. 

By separating the **Core Engine** (Download/Muxing/DRM) from the **Site Logic** (Scraping/API interaction) via a WebAssembly (WASM) plugin system, Hydrust allows developers to build and maintain site-specific extensions in a secure, sandboxed environment without ever touching the core source code.

### Key Architectural Pillars:
* **Memory Safety:** Built 100% in Rust for high-concurrency segment fetching.
* **WASM Extensions:** Site-specific logic runs in an isolated sandbox via `Wasmtime`.
* **Native Pipeline:** Uses `GStreamer` for true native remuxing (no shell-outs to `ffmpeg.exe`).
* **Headless Discovery:** Integrated Chromium-based sniffing for modern, JS-heavy manifest discovery.

---

## 🏗 Project Architecture



1. **Host (The Engine):** Manages the GStreamer pipeline, parallel downloads, and the DRM (Widevine/AES) stack.
2. **Guest (The Plugins):** Small `.wasm` files compiled from the Hydrust SDK that tell the Engine where to find the media.
3. **WIT (The Contract):** A strictly typed interface that defines how Host and Guest talk.

---

## 🎯 Future Goals & Roadmap

### Phase 1: The Foundation (Current)
- [ ] Stabilize the `Hydrust-SDK` and WIT interface.
- [ ] Implement the `tokio`-based parallel segment downloader.
- [ ] Basic GStreamer remuxing (HLS/DASH to MP4).

### Phase 2: The Extension Ecosystem
- [ ] Launch the `hydrust-cli` plugin generator (`hydrust new-plugin`).
- [ ] Implement Host-provided Networking (allowing WASM plugins to "ask" the Host to fetch HTML).
- [ ] Support for Headless Discovery (Chrome-driven manifest sniffing).

### Phase 3: The DRM Layer
- [ ] Integration with `rswidevine` for L3 content decryption.
- [ ] Support for custom CDM (Content Decryption Module) mounting.

### Phase 4: The Interface
- [ ] A modern, cross-platform GUI built with **Tauri**.
- [ ] Remote management via a Web Dashboard.

---

## 🛠 For Contributors

Hydrust is built for developers who are tired of brittle scripts. We are currently looking for help in the following areas:
* **GStreamer Gurus:** Help us optimize our Rust-based pipelines.
* **WASM Wizards:** Help refine our WIT interfaces and plugin lifecycle.
* **Rust Newbies:** We have plenty of `good-first-issues` in CLI improvement and documentation!

---

## ⚖️ Legal Disclaimer

Hydrust is a research tool designed for personal archival and educational purposes. The developers of Hydrust do not condone or encourage the violation of any platform's Terms of Service or Copyright laws. Use this tool responsibly and at your own risk.