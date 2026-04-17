# Crap Todo

A kanban-style task manager built with [Crap CMS](https://github.com/dkluhzeb/crap-cms) and [Leptos](https://leptos.dev).

**This is an example project.** It exists to show how a full-stack app can be built on top of Crap CMS — using its gRPC API for data, hooks for server-side logic, and collections for schema definition. It's not a production task manager; it's a reference for how the pieces fit together.

## What it demonstrates

- **Collections as schema** — tasks, comments, categories, users defined in Lua
- **gRPC API** — Leptos server functions talk to Crap CMS via gRPC (find, create, update, delete, undelete)
- **Hooks** — auto-slug, title trimming, overdue computation run server-side on every write
- **Soft delete + trash** — tasks are soft-deleted with a trash view and restore
- **Live updates** — SSE bridge subscribes to gRPC events, pushes mutations to the browser
- **Access control** — role-based access defined in Lua, enforced by the CMS
- **Leptos SSR + hydration** — server-rendered HTML with client-side interactivity

## Stack

| Layer | Tech |
|-------|------|
| CMS | [Crap CMS](https://github.com/dkluhzeb/crap-cms) (gRPC + admin UI) |
| Frontend | [Leptos 0.8](https://leptos.dev) (Rust, SSR + WASM hydration) |
| Styling | [Tailwind CSS v4](https://tailwindcss.com) |
| Transport | gRPC (tonic) + SSE for live updates |

## Quick start

```bash
docker compose up
```

That's it. On first run it installs tooling (~30s), then compiles the frontend (~3-5min).

> **Note:** This is a development setup. It runs `cargo leptos watch` with unoptimized builds and hot reload. For production you'd build a release binary and serve it behind a reverse proxy — but that's out of scope for an example project.

| Service | URL |
|---------|-----|
| App | http://localhost:8080 |
| CMS admin | http://localhost:3000 |

Login: `admin@craptodo.local` / `crap123`

## Project structure

```
crap-todos/
  crap.toml              # CMS configuration
  collections/           # Collection schemas (tasks, comments, users, ...)
  hooks/                 # Server-side hooks (auto-slug, overdue check, ...)
  access/                # Access control rules
  migrations/            # Seed data
  frontend/              # Leptos app
    src/
      api/               # Server functions (gRPC calls)
      app/
        components/      # Reusable UI components
        pages/           # Page components (board, trash, login)
      sse.rs             # SSE bridge (gRPC events -> browser)
      grpc.rs            # gRPC client wrapper
      types.rs           # Type helpers
      prelude.rs         # Shared imports
    proto/               # Protobuf definitions (synced from crap-cms)
    style/               # Tailwind entry point
```

## Development

The `docker-compose.yml` mounts the frontend source and runs `cargo leptos watch` — file changes trigger automatic rebuilds.

```bash
# Format Leptos view! macros
docker compose exec frontend leptosfmt src/**/*.rs

# Restart just the frontend
docker compose restart frontend

# Rebuild CMS (after pulling a new image)
docker compose pull cms && docker compose up -d cms --force-recreate
```

## How it connects to Crap CMS

The frontend never touches a database. All data flows through the CMS:

```
Browser  <-->  Leptos SSR (port 8080)  <-->  Crap CMS gRPC (port 50051)
                     |
                     +-- SSE /events  <--  gRPC Subscribe stream
```

Server functions in `src/api/` make gRPC calls. The `src/sse.rs` bridge subscribes to CMS mutation events and forwards them as SSE to connected browsers.

## License

MIT
