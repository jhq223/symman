# Symman (Symlink Manager)

[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![License: LGPL v3](https://img.shields.io/badge/License-LGPL_v3-blue.svg)](https://www.gnu.org/licenses/lgpl-3.0)

A lightweight, cross-platform command-line tool to manage, track, and health-check your symbolic links. Powered by Rust and SQLite.

[English](README.md) | [中文说明](README_zh.md)

---

## Table of Contents
- [Symman (Symlink Manager)](#symman-symlink-manager)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation](#installation)
    - [1. Download Pre-compiled Binaries (Recommended)](#1-download-pre-compiled-binaries-recommended)
    - [2. Install via Cargo](#2-install-via-cargo)
    - [3. Build from Source](#3-build-from-source)
  - [Usage](#usage)
  - [Platform Notes (Windows)](#platform-notes-windows)
  - [License](#license)

---

## Features
- **Centralized Tracking**: All created symlinks are recorded in a local SQLite database.
- **Cross-Platform**: Works seamlessly on both Unix-like systems and Windows.
- **Health Checks**: Quickly scan your system for broken or missing symlinks.
- **Smart Restore**: Easily remove a symlink and automatically copy the target file/directory back to the original location.

## Installation

### 1. Download Pre-compiled Binaries (Recommended)
You can download the latest pre-compiled binaries for your operating system from the [Releases](../../releases) page.
Just extract the executable and add it to your system's `PATH`.

### 2. Install via Cargo
If you have the Rust toolchain installed, you can easily install `symman` directly from crates.io:
```bash
cargo install symman
```

### 3. Build from Source
To compile manually, ensure you have [Rust installed](https://rustup.rs/), then run:
```bash
git clone https://github.com/yourusername/symman.git
cd symman
cargo build --release
```

---

## Usage
*Note: The first argument is where you want to CREATE the link. The second argument is the REAL target file/folder that already exists.*

```bash
# Create a new symlink and name it 'my_config'
symman new /path/to/where/link/goes /path/to/real/target --name my_config

# List all managed symlinks
symman list

# Check health of all symlinks
symman check

# Remove a link (add --restore to copy original files back)
symman remove my_config --restore
```

## Platform Notes (Windows)
Creating symbolic links on Windows requires specific permissions. If you encounter an **"Insufficient Privilege"** error, please do one of the following:
1. **Enable Developer Mode** (Recommended): 
   - *Windows 11*: Settings > System > Advanced > Developer Mode > turn on "Developer Mode".
   - *Windows 10*: Settings > Privacy & security > For developers.
2. Run your terminal (PowerShell/CMD) as **Administrator**.

## License
This project is licensed under the **GNU Lesser General Public License v3.0** (LGPL-3.0). See the [LICENSE](LICENSE) file for details.
