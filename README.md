# Ark Security Core — Universal Unlocker 🛡️

High-performance, low-level hardware interaction subsystem and 10-mirrors data pipeline architecture engineered in **Rust** and orchestrated via **Tauri** for secure Windows 10/11 environments.

## 🏗️ System Architecture

The core architecture operates on a zero-copy, memory-hardened pipeline that safe-routes low-level protocol commands through a sequential matrix of 10 cryptographic mirrors.

*   **Core Engine (`src-tauri/src/core/`):** Manages atomic data beams, state transitions, and immediate memory sanitization via the Brown Stone protocol.
*   **Protocol Suite (`src-tauri/src/protocols/`):** Low-level hardware communication handlers targeting MediaTek (BROM), Qualcomm (EDL 9008), Samsung (Odin), and Apple (DFU/PongoOS).
*   **Security Gates:** Embedded asynchronous infrastructure linked directly to hardware detection states to prevent race conditions or memory leakage during exploit drops.

## 🛠️ Prerequisites

To compile and run the Ark Security Core execution layer, your local workstation requires:

1.  **Rust Toolchain:** Stable channel (v1.75+ recommended). [Install Rust](https://www.rust-lang.org/tools/install)
2.  **Node.js Environment:** LTS Version (v18 or v20) along with `npm`. [Install Node.js](https://nodejs.org/)
3.  **C++ Build Tools:** Windows 10/11 SDK and C++ x64 build tools installed via Visual Studio Installer (Required for compiling low-level Win32 API bindings and Rust compilation).

## 🚀 Quick Start & Deployment

### 1. Install Dependencies
```bash
# Install frontend package allocations
npm install
