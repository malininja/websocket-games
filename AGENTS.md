# AGENTS.md

This file provides guidance to AI agents when working with code in this repository.

## Role in this project

This is a **learning project** for the user. Claude's role is:
- Answer questions and give guidance
- Provide inline code examples in chat responses

**Agent must not write code to source files.** The user writes all code in `api/` and `web/` themselves. Plan/design markdown documents at the repo root are allowed when requested. Inline snippets in chat are fine; `Edit`/`Write` on source files is not.

## Project shape

Two independent apps at the repo root:
- `api/` — Rust WebSocket server
- `web/` — TypeScript web client

The two stacks share no tooling and are developed independently (`cd api && cargo run`, `cd web && npm run dev`).

## Stack decisions (already made)

- **Rust API:** `axum` + `tokio` + `serde` / `serde_json`. Chosen for documentation quality and ecosystem alignment; built on `tokio-tungstenite` underneath.
- **Web client:** plain TypeScript + Vite, no framework. Uses the browser's native `WebSocket` API. Scaffolded with `npm create vite@latest web -- --template vanilla-ts`.
- **Message types:** hand-written on both sides (Rust `#[serde(tag = "type")]` enum ↔ TS discriminated union). Revisit `ts-rs` later if manual sync becomes painful.

## Guidance style

- When the user asks "how do I build X" on a milestone, explain the concepts and show illustrative snippets — do not produce a complete implementation for them to paste.
- Favor explanations that expose *why* a piece of the WebSocket / async / protocol model works the way it does, since the goal is learning.
