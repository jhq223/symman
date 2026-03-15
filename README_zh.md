# Symman (软链接管理器)

[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![License: LGPL v3](https://img.shields.io/badge/License-LGPL_v3-blue.svg)](https://www.gnu.org/licenses/lgpl-3.0)

一个轻量级的跨平台命令行工具，用于管理、追踪和检查系统中的软链接（Symbolic Links）。基于 Rust 和 SQLite 构建。

[English](README.md) | [中文说明](README_zh.md)

---

## 目录
- [Symman (软链接管理器)](#symman-软链接管理器)
  - [目录](#目录)
  - [主要特性](#主要特性)
  - [安装指南](#安装指南)
    - [1. 下载预编译程序 (推荐)](#1-下载预编译程序-推荐)
    - [2. 通过 Cargo 安装](#2-通过-cargo-安装)
    - [3. 手动编译源码](#3-手动编译源码)
  - [使用说明](#使用说明)
  - [Windows 平台注意事项](#windows-平台注意事项)
  - [开源协议](#开源协议)

---

## 主要特性
- **集中追踪**：所有通过工具创建的软链接都会被记录在本地的 SQLite 数据库中。
- **跨平台支持**：完美支持 Windows、macOS 和 Linux，并自动处理不同系统的底层差异。
- **健康度检查**：一键扫描所有软链接，快速找出变成“死链”或丢失的链接。
- **一键还原**：删除软链接时，支持将真实目标文件/文件夹原样拷贝回原处。

## 安装指南

### 1. 下载预编译程序 (推荐)
前往项目的 [Releases](../../releases) 页面，下载对应操作系统的预编译程序。解压后将其所在目录添加到系统的 `PATH` 环境变量中即可。

### 2. 通过 Cargo 安装
如果你已经安装了 Rust 工具链，可以直接通过 `cargo` 安装：
```bash
cargo install symman
```

### 3. 手动编译源码
如果你想从源码构建，请先确保安装了 [Rust](https://rustup.rs/)，然后执行：
```bash
git clone https://github.com/yourusername/symman.git
cd symman
cargo build --release
```

---

## 使用说明
*提示：第一个参数是你想要【创建出链接的位置】，第二个参数是【已经真实存在的源文件/目录】。*

```bash
# 创建软链接并命名为 my_app_data
symman new C:\Users\foo\AppData D:\Symlinks\AppData --name my_app_data

# 列出所有记录在案的软链接
symman list

# 检查所有软链接是否断裂/丢失
symman check

# 删除软链接 (附带 --restore 会在删除前把真实文件拷回原处)
symman remove my_app_data --restore
```

## Windows 平台注意事项
在 Windows 系统中创建软链接需要特定的权限。如果你遇到 **"Insufficient Privilege" (权限不足)** 错误，请尝试以下两种解决方法之一：
1. **开启开发者模式**（推荐）：
   - *Windows 11*：设置 > 系统 > 高级 > 开发者选项 > 开启“开发人员模式”。
   - *Windows 10*：设置 > 隐私和安全性 > 开发者选项 > 开启“开发人员模式”。
2. 以**管理员身份**运行终端（PowerShell 或 CMD）。

## 开源协议
本项目采用 **GNU Lesser General Public License v3.0** (LGPL-3.0) 协议开源。详细信息请参阅 [LICENSE](LICENSE) 文件。