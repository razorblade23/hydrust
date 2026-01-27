# 🤝 Contributing to Hydrust

First off, thank you for considering contributing to **Hydrust**! It’s people like you that make open-source tools great.

## 🛠 Our Development Strategy: Dev Containers

Hydrust is a high-performance engine that relies on **GStreamer** and several native C-libraries. To save you from "Dependency Hell" (manually installing headers, configuring `pkg-config`, and fixing linker errors), we use **Dev Containers**.

### 🏗 Prerequisites
Before you start, you only need two things installed on your host machine:
1. **Docker** (or Podman)
2. **Visual Studio Code** with the **[Dev Containers Extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)**

### 🚀 Getting Started
1. **Clone the repo:** `git clone https://github.com/razorblade23/hydrust.git`
2. **Open in VS Code:** `code hydrust`
3. **Trigger the Container:** A popup should appear in the bottom-right corner asking to **"Reopen in Container"**. Click it!
4. **Wait for the magic:** VS Code will now pull a specialized Rust image and install all the GStreamer headers for you automatically.

---

## 🚦 Troubleshooting the Setup

> It takes a couple of minutes to install and run everything
> The first time you set up the container, it may take 5–10 minutes depending on your internet speed. Grab a coffee! ☕ Once it's built, subsequent launches are nearly instant.

### "Connection Reset by Peer"
If you have a slow or unstable network, the initial image download might fail with a `read: connection reset by peer` error.

**The Fix:** Manually pull the base image in your terminal to "pre-heat" Docker before opening VS Code:
```bash
docker pull [mcr.microsoft.com/devcontainers/rust:1-1-bookworm](https://mcr.microsoft.com/devcontainers/rust:1-1-bookworm)
```

### SDK fails to find "*.wit" file
If SDK fails to build because of the missing `*.wit` files, here is the fix

**The Fix:** Manually symling the files
```bash
# Move into SDK directory
cd crates/hydrust-sdk

# Create a symlink to actual file
ln -s ../../wit wit
```


## 🗺 How Can I Help?
We are looking for help in several areas:
- Core Development: Improving the GStreamer pipeline and segment fetching logic.
- SDK Design: Refining the WIT interfaces for easier plugin development.
- Testing: Writing unit tests for the WASM loader.
- Documentation: Improving this guide or writing tutorials for new plugin devs!

## Pull Request Process
- Create a branch for your feature/fix: `git checkout -b feature/cool-new-thing`.
- Ensure your code is formatted: `cargo fmt`.
- Open a Pull Request and describe your changes. We'll review it as soon as possible!

** By contributing to Hydrust, you agree that your contributions will be licensed under the project's MIT/Apache-2.0 license. **