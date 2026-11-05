# SigmaOS-alpm-rs — Documentation

## Purpose

Rust bindings for the ALPM library

This component mirrors the responsibility of `archlinux/alpm-rs` inside the
SigmaOS ecosystem, implemented in memory-safe Rust.

## Architecture

- `src/lib.rs` — public API and core logic
- `tests/` — integration tests
- Follows separation of mechanism (this crate) and policy (configuration).

## Engineering principles applied

Abstraction, component isolation, loose coupling, high cohesion, KISS, DRY,
secure defaults, fail-fast, defensive programming, deterministic testing,
stable APIs, open standards.
