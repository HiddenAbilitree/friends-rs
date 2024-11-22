<h1 align="center">

[![friends-rs](assets/friends-rs.svg)](https://github.com/HiddenAbilitree/friends-rs)

</h1>

<p align="center">
  <a href="https://github.com/HiddenAbilitree/friends-rs/releases"><img src="https://img.shields.io/github/release/HiddenAbilitree/friends-rs.svg" alt="Latest Release"></a>
  <img alt="flake.nix" src="https://img.shields.io/badge/flake-nix-blue?logo=nixos">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written_in-rust-orange?logo=rust">
  <img alt="Blazingly Fast" src="https://img.shields.io/badge/%F0%9F%9A%80_blazingly-fast-orange">
  <a href="https://gitmoji.dev/"><img alt="gitmoji" src="https://img.shields.io/badge/%F0%9F%98%82_git-moji-yellow"></a>
</p>

> [!NOTE]
> This project is based on the work of `JacobEvelyn/friends`.
>
> <a href="https://github.com/JacobEvelyn/friends"><img alt="JacobEvelyn/friends" src="https://img.shields.io/badge/JacobEvelyn-friends-blue?logo=github"></a>

## Tracing 🚀

<a href="https://ui.perfetto.dev/"><img alt="Perfetto" src="https://img.shields.io/badge/perfetto-ui-green?logo=googlechrome"></a>
<a href="https://crates.io/crates/tracing"><img alt="Tracing" src="https://img.shields.io/badge/tokio--rs-tracing-blueviolet?logo=rust"></a>
<a href="https://crates.io/crates/tracing-subscriber"><img alt="Tracing Subscriber" src="https://img.shields.io/badge/tokio--rs-tracing--subscriber-blueviolet?logo=rust"></a>
<a href="https://crates.io/crates/tracing-chrome"><img alt="Tracing Chrome" src="https://img.shields.io/badge/thoren--d-tracing--chrome-blueviolet?logo=rust"></a>

`friends-rs` has a crate feature which enables tracing via [tracing-chrome](https://crates.io/crates/tracing-chrome)
through the feature flag `tracing`. You can run `friends-rs` with tracing using:

```sh
RUST_LOG=friends=trace cargo run --features=tracing
```

...and then load the json trace (which will be named something like `trace-....json`) in [perfetto](https://ui.perfetto.dev/).
