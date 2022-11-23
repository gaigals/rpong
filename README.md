# RPONG

Ping-Pong mini game written in Rust and compiled in webassembly.

## Setup

Installing `wasm-pack` to compile rust code in webassembly.

```
cargo install wasm-pack
```

## Building

```
wasm-pack build
```

Optional flags for logging: `--log-level [level]` (info, warn, error), `--quiet`, `--verbose`.