
# Profiling Heap Allocations in Rust on Windows

This project demonstrates the process of profiling heap allocations in a Rust application on Windows using the Windows Performance Toolkit. It performs heap allocations of different sizes and uses `wpr` to capture these allocations.

Heap profiling requires running `wpr` with administrative privileges. Because the application runs `wpr` directly, it must be run with administrative privileges too.

## Prerequisites

- The latest available version of *An external link was removed to protect your privacy.*.

## Example

```powershell
git clone --depth 1 https://github.com/Vaiz/rust-wpr.git
cd rust-wpr
# For demonstrative purposes, it's better to build a debug version of the application
cargo run heapsnapshot.etl
wpa heapsnapshot.etl 
```
