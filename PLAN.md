# websocket-games — Learning Plan

A learning project to build WebSocket-based multiplayer games, split into a Rust API and a TypeScript web client.

## Stack decisions

- **Repo layout:** `api/` (Rust) and `web/` (TypeScript) at the repo root, as two independent projects. No monorepo tooling — the two stacks don't share a package manager or build system.
- **Rust API:** `axum` + `tokio` + `serde` / `serde_json`.
  - Chosen for best-in-class documentation, large community, and alignment with the mainstream Tokio ecosystem. Built on `tokio-tungstenite` under the hood.
- **Web client:** plain TypeScript + Vite.
  - No framework — keeps focus on the WebSocket protocol rather than UI state management. Uses the browser's native `WebSocket` API.
  - Scaffold: `npm create vite@latest web -- --template vanilla-ts`.
- **Message types:** hand-written on both sides (Rust enum with `serde`, TS discriminated union). Revisit `ts-rs` later if manual sync becomes a chore.

## Milestones

Each milestone builds on the previous one. Don't skip ahead — the concepts stack.

### Milestone 1: Echo server
- Rust: axum server exposing `GET /ws` that upgrades to a WebSocket and echoes every text message back.
- Web: single page with a text input and a log; connects, sends, displays responses.
- **Teaches:** project setup on both sides, the WebSocket handshake, reading/writing frames.

### Milestone 2: Broadcast (chat)
- Server holds a `tokio::sync::broadcast` channel; every connected client receives every message sent by any client.
- Web: scrolling list of messages from all connected users.
- **Teaches:** shared state across connections, fan-out, the Arc/Mutex vs. channels decision.

### Milestone 3: Typed messages
- Replace raw strings with JSON envelopes, e.g. `{ "type": "chat", "text": "..." }`.
- Rust: `#[serde(tag = "type")]` enum. TS: discriminated union with `type` as the tag.
- **Teaches:** protocol design, serde tagging, TS type narrowing.

### Milestone 4: Rooms / sessions
- Clients join a named room; broadcasts are scoped to that room.
- Server tracks a `HashMap<RoomId, broadcast::Sender<Msg>>` behind an async-aware lock.
- Handle connection lifecycle cleanly: clean up empty rooms on disconnect.
- **Teaches:** per-room state, connection lifecycle, disconnect handling.

### Milestone 5: First real game — Tic-Tac-Toe
- Two players per room. Server holds authoritative game state, validates moves, broadcasts state updates.
- Handle game-over, rematch, and a player leaving mid-game.
- **Teaches:** authoritative server model, turn-based flow, input validation, terminal states.

## Later milestones (pick based on interest)

- Reconnection with session resume
- Player identity / display names
- Real-time game with a server tick loop (e.g., Pong)
- Spectators
- Multiple concurrent games per server
- Protocol evolution: versioning messages without breaking old clients

## Working style for this project

- This project is for learning. Claude's role is to plan, guide, and answer questions — never write code to source files. Inline code examples in chat are fine; code in `api/` and `web/` is written by the user.
