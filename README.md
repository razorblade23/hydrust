# 🌊 Hydrust 

**The Modular, High-Performance Media Archiving Engine.** *Built in Rust. Powered by WebAssembly. Driven by the Community.*

---

> [!WARNING]
> **COMPILATION STATUS: STAGE 0** > This project is currently in the architectural drafting phase. The codebase is under active construction and is not yet suitable for production workloads.

---

## 🚀 Engine Overview

Hydrust is a **native media orchestration platform** engineered to solve the "maintenance trap" of modern streaming scrapers. 

By decoupling the **Core Engine** (Download/Muxing/DRM) from the **Site Logic** (Scraping/API interaction) via a WebAssembly (WASM) plugin system, Hydrust provides a secure, sandboxed environment for extensions. Developers can iterate on site-specific logic without recompiling or compromising the core binary.

### 🏗️ Architectural Pillars

| Component | Technology | Role |
| :--- | :--- | :--- |
| **Memory Safety** | `Rust` | High-concurrency segment fetching and memory management. |
| **Sandboxing** | `Wasmtime` | Isolated execution of site-specific Guest modules. |
| **Native Pipeline** | `GStreamer` | True native remuxing (HLS/DASH to MP4) without sub-processing. |
| **Discovery** | `Playwright/Chromium` | Headless manifest sniffing for JS-heavy environments. |

---

## 🛠 System Architecture



The Hydrust ecosystem operates as a distributed runtime:

1.  **Host (The Engine):** Orchestrates the GStreamer pipeline, manages asynchronous `tokio` tasks, and handles the DRM (Widevine/AES) stack.
2.  **Guest (The Plugins):** Lightweight `.wasm` modules compiled via the **Hydrust SDK**. These modules contain the logic to parse manifests and extract media URLs.
3.  **WIT (The Contract):** A strictly typed WebAssembly Interface Type definition that ensures safe, deterministic communication between Host and Guest.

---

## 🎯 Development Roadmap ()

### 📦 Phase 1: The Foundation (Current)
- [ ] **SDK Stabilization:** Finalize the WIT interface and plugin lifecycle.
- [ ] **Parallel Downloader:** Implement a high-throughput `tokio`-based segment fetcher.
- [ ] **Core Pipeline:** Basic GStreamer integration for HLS/DASH remuxing.

### 🏗️ Phase 2: The Extension Ecosystem
- [ ] **CLI Tooling:** Launch `hydrust-cli` for automated plugin scaffolding.
- [ ] **Host-Call API:** Implement networking shims for Guest modules.
- [ ] **Manifest Sniffing:** Integrated headless Chrome discovery.

### 🔐 Phase 3: The DRM Layer
- [ ] **Widevine L3:** Integration with `rswidevine` for content decryption.
- [ ] **CDM Management:** Support for custom Content Decryption Module mounting.

### 💻 Phase 4: The User Interface
- [ ] **Desktop GUI:** A modern, cross-platform interface built with **Tauri**.
- [ ] **Remote Dashboard:** Web-based orchestration and monitoring.

---

## 🦀 For Contributors

Hydrust is built for developers who demand efficiency and safety. We utilize a level-based contributor system (see our Discord for details).

* **GStreamer Gurus:** Help optimize our Rust-based media pipelines.
* **WASM Wizards:** Refine WIT interfaces and the plugin runtime.
* **Rust Enthusiasts:** Tackle `good-first-issues` in CLI development and documentation.

> [!INFO]
> **Rust SDK v0.1.2 is available** 
> Current version of SDK for Rust is available on crates.io.
> SDK for other languages will be available soon.
> Rust: [Hydrust SDK](https://crates.io/crates/hydrust-sdk)
---

## ⚖️ Legal Disclaimer

Hydrust is a research tool designed for **personal archival and educational purposes**. The developers do not condone or encourage the violation of any platform's Terms of Service or Copyright laws. Use this software responsibly.

---

<p align="center">
  <b>Join the Pipeline</b><br>
  <a href="https://discord.gg/uX4EFqD6ew">
    <img src="https://img.shields.io/discord/1469101324036739278?label=Hydrust&logo=discord&style=for-the-badge&color=7289da" alt="Discord">
  </a>
</p>