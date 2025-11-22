# Development Log - Zoo Interactive Project

## Issues Encountered & Solutions

### 1. Git Authentication Error

**Problem:**

- Error: "fatal: Could not read from remote repository"
- Unable to push/pull from GitHub

**Cause:**

- No SSH keys configured for GitHub authentication
- Repository configured to use SSH (`git@github.com`)

**Solution:**

- Generated ED25519 SSH key pair
- Added public key to GitHub account
- Successfully authenticated with GitHub

---

### 2. Git Push to Main Branch Failed

**Problem:**

- Could not push to `main` branch
- Error: "failed to push some refs"
- Local branch was `master`, remote default was `main`

**Cause:**

- Different commit histories between local `master` and remote `main`
- Both had different "Initial commit" commits (unrelated histories)

**Solution:**

- Created local `main` branch tracking remote `main`
- Merged `master` into `main` with `--allow-unrelated-histories`
- Successfully pushed to `main` branch

---

### 3. Docker Build - Rust Edition 2024 Not Supported

**Problem:**

- Docker build failed with: "feature `edition2024` is required"
- `Cargo.toml` specified `edition = "2024"`
- Docker image used Rust 1.85 (doesn't support edition 2024)

**Cause:**

- Edition 2024 requires Rust nightly
- Original Dockerfile used `rust:1.85` (stable version)

**Solution:**

- Changed Dockerfile to use `rustlang/rust:nightly`
- Edition 2024 now compiles successfully

---

### 4. Docker Runtime Error - Binary Not Found

**Problem:**

- Error: "exec: './backend': stat ./backend: no such file or directory"
- Container failed to start

**Cause:**

- Volume mount `./backend:/app` was overwriting the compiled binary
- The mount replaced container's `/app` with local source directory (no binary)

**Solution:**

- Removed volume mount from production `docker-compose.yml`
- Binary now accessible in container

---

### 5. Frontend Dockerfile Path Error

**Problem:**

- Error: "failed to read dockerfile: open Dockerfile: no such file or directory"
- Docker Compose couldn't find frontend Dockerfile

**Cause:**

- `docker-compose.yml` pointed to `./frontend`
- Dockerfile was actually in `./frontend/static`

**Solution:**

- Updated `docker-compose.yml` to `build: ./frontend/static`
- Later reorganized: moved all files from `frontend/static/` to `frontend/`

---

### 6. CORS Error - Frontend Can't Reach Backend

**Problem:**

- Browser error: "Cross-Origin Request Blocked"
- Frontend couldn't fetch from backend

**Cause:**

- No CORS layer configured in backend
- Browser blocks cross-origin requests (localhost:5173 → localhost:3000)

**Solution:**

- Added `tower_http::cors::CorsLayer::permissive()` to backend
- Applied layer to Axum router
- CORS now allows all origins in development

---

### 7. Backend Handler Returns Nothing

**Problem:**

- Backend compiled but server immediately exited
- No HTTP response sent to clients

**Cause:**

- Handler function `initial_page()` had no return type
- Axum requires handlers to return valid response types

**Solution:**

- Changed handler to return `-> &'static str`
- Handler now returns "Hello from backend!"
- Server stays running and responds to requests

---

### 8. Frontend Fetch URL Malformed

**Problem:**

- Frontend request went to: `http://localhost:5173/0.0.0.0:3000/`
- 404 error

**Cause:**

- Fetch URL missing `http://` protocol
- Used `0.0.0.0` instead of `localhost`
- Browser treated it as relative path

**Solution:**

- Changed to `fetch("http://localhost:3000/message")`
- Requests now reach backend correctly

---

### 9. Backend Crashes in Docker - No Source Code

**Problem:**

- Backend container exited immediately
- Error: "no targets specified in the manifest"
- "either src/lib.rs, src/main.rs... must be present"

**Cause:**

- Running production Dockerfile with `cargo watch` command
- Source code not mounted in production containers
- Mismatched Docker setup

**Solution:**

- Created separate development Docker setup:
  - `docker-compose.dev.yml` - mounts source code, uses cargo-watch
  - `Dockerfile.dev` - for backend and frontend
- Use `sudo docker compose -f docker-compose.dev.yml up` for development

---

### 10. Connection Reset Error After Directory Restructure

**Problem:**

- Browser showed "The connection was reset"
- Containers not accessible

**Cause:**

- Used wrong docker-compose file after restructuring
- Production containers couldn't find source code

**Solution:**

- Ensured using `docker-compose.dev.yml` for development
- All paths updated after moving from `frontend/static/` to `frontend/`

---

## Final Working Setup

### Directory Structure

```
Desafio-CIEE/
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── handlers/
│   │   └── animal/
│   ├── Cargo.toml
│   ├── Dockerfile              # Production
│   └── Dockerfile.dev          # Development
├── frontend/
│   ├── src/
│   ├── public/
│   ├── package.json
│   ├── Dockerfile              # Production
│   └── Dockerfile.dev          # Development
├── docker-compose.yml          # Production config
└── docker-compose.dev.yml      # Development config
```

### Key Configuration Files

**Backend (`main.rs`):**

- Uses Axum web framework
- CORS enabled with `CorsLayer::permissive()`
- Listens on `0.0.0.0:3000`
- Route: `/message` returns "Hello from backend!"

**Backend (`Cargo.toml`):**

- Edition: `2024` (requires Rust nightly)
- Dependencies: axum, tokio, tower-http, serde, sqlx, uuid

**Docker Compose Dev:**

- Backend: Mounts source, runs `cargo watch -x run`
- Frontend: Mounts source, runs `npm run dev --host 0.0.0.0`
- SQL Server: Persistent data volume

### Development Workflow

**Start development environment:**

```bash
sudo docker compose -f docker-compose.dev.yml up
```

**Access:**

- Frontend: http://localhost:5173
- Backend: http://localhost:3000
- SQL Server: localhost:1433

**Features:**

- ✅ Hot reload for both frontend and backend
- ✅ Source code changes reflect immediately
- ✅ CORS configured for cross-origin requests
- ✅ All services running in Docker

**Stop:**

```bash
sudo docker compose -f docker-compose.dev.yml down
```

---

## Technologies Used

- **Backend:** Rust, Axum, Tokio, Tower-HTTP
- **Frontend:** React, Vite
- **Database:** Microsoft SQL Server 2022
- **Containerization:** Docker, Docker Compose
- **Version Control:** Git, GitHub
