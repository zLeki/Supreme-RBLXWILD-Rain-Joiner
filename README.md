# Rust + devkitPro

> Installation of Rust on a devKitPro environment, made simple.

## Step by step

### 1 - Installing Rust (skip if already installed)

Depending on your OS, installing Rust is different. However, every type of installation should ask you which toolchain you want to install, and whether it should modify PATH.

- For the toolchain to install, select `nightly-2019-01-19`, and don't change the host triple unless you know what you're doing.

- Make sure that the option to modify PATH is enabled!

### 2 - Specific toolchain (`nightly-2019-01-19-*`)

In order to get our [custom libstd](https://github.com/rusty-horizon/switch-rust-std) working, an slightly old toolchain must be used (this specific version, with the one Rust defaults to your machine)

Example:

- Windows 10, 64-bit -> `nightly-2019-01-19-x86_64-pc-windows-gnu`

- WSL (above but with WSL) -> `nightly-2019-01-19-x86_64-unknown-linux-gnu`

In order to install and default the toolchain:

- Run `rustup install <your-nightly-toolchain>`

- Run `rustup default <your-nightly-toolchain>`

### 3 - Final components: rust-src and xargo

We use `xargo` to be able to cross-compile with our custom `libstd`, and rust-src is a dependency of xargo:

> (make sure you have already installed and defaulted the specific toolchain stated above)

- Run `cargo install xargo`

- Run `rustup component add rust-src`

### 4 - Profit!

You should be done now! Try cloning [Rust examples repository](https://github.com/rusty-horizon/rust-examples) and compiling the examples to test if everything is fine.
