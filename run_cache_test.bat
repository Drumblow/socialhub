@echo off
set RUST_LOG=debug
cargo test test_cache_capacity -- --nocapture
