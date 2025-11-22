# Issue #9: Backend Crashes in Production Docker - No Source Code Mounted

## Problem Description

After resolving the initial Docker volume mount issue (#4), a new problem emerged during development workflow. When using the production Docker Compose configuration (`docker-compose.yml`) for development, the backend would crash immediately after any code change, showing:

```
rust-backend-1  | thread 'main' panicked at src/main.rs:15:10
rust-backend-1  | Failed to bind to address
rust-backend-1 exited with code 101
```

The container would then enter a restart loop, repeatedly crashing because the compiled binary couldn't be updated with new code changes.

## Technical Details

### What Happened

1. Developer made changes to `backend/src/main.rs`
2. Ran `sudo docker compose up` to test changes
3. Container started with **old compiled binary**
4. Code changes not reflected in running application
5. Developer tried to rebuild: `sudo docker compose build`
6. Stopped containers: `sudo docker compose down`
7. Started again: `sudo docker compose up`
8. **Same old code running** - changes still not visible
9. Confusion: "Why aren't my changes working?"

### Configuration Analysis

**docker-compose.yml (production):**

```yaml
version: "3.8"

services:
  rust-backend:
    build: ./backend
    container_name: rust-backend
    ports:
      - "3000:3000"
    # NO volumes - this is the key difference

  react-frontend:
    build: ./frontend
    container_name: react-frontend
    ports:
      - "80:80"

  sqlserver:
    image: mcr.microsoft.com/mssql/server:2022-latest
    container_name: sqlserver
    environment:
      - ACCEPT_EULA=Y
      - SA_PASSWORD=YourStrong@Passw0rd
    ports:
      - "1433:1433"
```

**Dockerfile (production):**

```dockerfile
FROM rustlang/rust:nightly AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy ONLY the binary from builder
COPY --from=builder /app/target/release/backend /app/backend
RUN chmod +x /app/backend

EXPOSE 3000
CMD ["/app/backend"]
```

### Why Changes Weren't Visible

**Docker image lifecycle:**

1. **Build Time:**

   - `docker compose build` creates image
   - Compiles code into binary
   - Binary embedded in image
   - **Image is immutable snapshot**

2. **Runtime:**
   - `docker compose up` starts container from image
   - Container has binary from build time
   - Source code changes on host **not visible** to container
   - Container runs old binary

**To see new changes in production setup:**

```bash
# Must rebuild image EVERY time code changes
sudo docker compose down
sudo docker compose build
sudo docker compose up
```

This workflow is:

- ❌ Slow (full rebuild takes 3-5 minutes)
- ❌ Tedious (3 commands every change)
- ❌ Frustrating (long feedback loop)
- ❌ Unproductive (kills development flow)

## Root Cause Analysis

### Production vs Development Workflows

**Production Requirements:**

- ✅ Fast startup
- ✅ Small image size
- ✅ Stable binary
- ✅ No dependencies on host
- ❌ Don't need live code updates

**Development Requirements:**

- ✅ Live code reload
- ✅ Fast feedback loop
- ✅ See changes immediately
- ❌ Don't care about image size
- ❌ Don't care about startup time

**The problem:** Using production configuration for development.

### Understanding Hot Reload

**What is hot reload?**

The ability to see code changes reflected in the running application **without manually restarting**.

**How it works in different environments:**

**Node.js (Vite):**

```
1. Change file
2. Vite detects change
3. Rebuilds module
4. Browser hot-reloads
5. See change instantly
```

**Rust (cargo-watch):**

```
1. Change file
2. cargo-watch detects change
3. Rebuilds binary
4. Restarts server
5. See change in ~10 seconds
```

**Production Docker:**

```
1. Change file
2. Nothing happens (file not in container)
3. Must rebuild entire image
4. Must restart container
5. See change in ~5 minutes
```

### Why Production Docker Can't Hot Reload

**Immutable Images:**

Docker images are **read-only layers**:

```
Image Layer Stack:
[Layer 4] CMD ["/app/backend"]           ← Start command
[Layer 3] COPY backend /app/backend      ← Binary file
[Layer 2] RUN apt-get install ...        ← Dependencies
[Layer 1] FROM debian:bookworm-slim      ← Base OS
```

Once built, layers can't change. To update binary, must rebuild entire image.

**No Source Code in Production Image:**

```dockerfile
# Builder stage
COPY src ./src
RUN cargo build --release

# Runtime stage - new, clean image
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/backend /app/backend
# ↑ Only binary copied, NO source code
```

Runtime container has:

- ✅ Compiled binary
- ❌ No source code
- ❌ No Rust compiler
- ❌ No cargo
- ❌ No build tools

Can't rebuild even if we wanted to.

### The Development Dilemma

**Options for development:**

**Option 1: Keep rebuilding (current situation)**

```bash
# Edit code
vim src/main.rs

# Rebuild
sudo docker compose down
sudo docker compose build
sudo docker compose up

# Wait 5 minutes
# Test change
# Repeat
```

Productivity: ❌ Very low

**Option 2: Run locally without Docker**

```bash
cargo run
# Edit code
# cargo automatically rebuilds
```

Productivity: ✅ High
But: Doesn't test Docker configuration

**Option 3: Separate development Docker setup**

```bash
sudo docker compose -f docker-compose.dev.yml up
# Has hot-reload built in
```

Productivity: ✅ High
And: Tests Docker configuration

## How the Error Occurred

**Development Timeline:**

1. **Initial Setup:**

   - Created `docker-compose.yml` for production
   - Created `Dockerfile` with multi-stage build
   - Built and tested: worked great

2. **First Code Change:**

   ```rust
   // Changed src/main.rs
   println!("Updated message!");
   ```

3. **Ran Docker:**

   ```bash
   sudo docker compose up
   ```

4. **Tested:**

   ```bash
   curl http://localhost:3000/message
   Hello from backend!  # Old message!
   ```

5. **Confusion:**
   "I changed the code, why is old message showing?"

6. **Tried Rebuilding:**

   ```bash
   sudo docker compose build
   sudo docker compose up
   ```

7. **Still Old Code:**
   "Did I save the file? Is Docker caching?"

8. **Realized:**
   "Oh! I need to rebuild EVERY time. This is going to be painful."

9. **Searched for Solution:**
   "How to enable hot-reload in Docker for Rust?"

10. **Found Answer:**
    Need separate development configuration with cargo-watch and volume mounts.

### Why Volume Mounts Were Removed

**Earlier (Issue #4):** Volume mounts were removed because they **overwrote the compiled binary**.

**Now:** Need volume mounts back for development, but in a **different way**.

**The key difference:**

**Production (Issue #4 - wrong):**

```yaml
volumes:
  - ./backend:/app # Overwrites binary
command: /app/backend # Binary not there!
```

**Development (now - correct):**

```yaml
volumes:
  - ./backend:/app # Mounts source code
command: cargo watch -x run # Compiles inside container
```

The difference:

- Production: Run pre-compiled binary (volume mount breaks this)
- Development: Compile inside container (volume mount enables this)

## Solution Implementation

### Solution: Create Separate Development Configuration

**Created: `docker-compose.dev.yml`**

```yaml
version: "3.8"

services:
  rust-backend:
    build:
      context: ./backend
      dockerfile: Dockerfile.dev # Different Dockerfile
    container_name: rust-backend-dev
    ports:
      - "3000:3000"
    volumes:
      - ./backend:/app # Mount source code
    depends_on:
      - sqlserver
    environment:
      - DATABASE_URL=Server=sqlserver,1433;Database=zoo_db;User Id=sa;Password=YourStrong@Passw0rd;
      - RUST_LOG=debug

  react-frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile.dev
    container_name: react-frontend-dev
    ports:
      - "5173:5173"
    volumes:
      - ./frontend:/app # Mount source code
      - /app/node_modules # Except node_modules (use container's)
    depends_on:
      - rust-backend
    environment:
      - VITE_API_URL=http://localhost:3000

  sqlserver:
    image: mcr.microsoft.com/mssql/server:2022-latest
    container_name: sqlserver
    environment:
      - ACCEPT_EULA=Y
      - SA_PASSWORD=YourStrong@Passw0rd
    ports:
      - "1433:1433"
    volumes:
      - sqlserver-data:/var/opt/mssql

volumes:
  sqlserver-data:
```

**Created: `backend/Dockerfile.dev`**

```dockerfile
FROM rustlang/rust:nightly

WORKDIR /app

# Install cargo-watch for hot-reload
RUN cargo install cargo-watch

# Copy dependency files first
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies (cached layer)
RUN cargo fetch

# Source code will be mounted from host via volume
# No need to COPY src here

# Expose port
EXPOSE 3000

# Use cargo-watch to rebuild on file changes
CMD ["cargo", "watch", "-x", "run"]
```

**Created: `frontend/Dockerfile.dev`**

```dockerfile
FROM node:20-alpine

WORKDIR /app

# Copy dependency files
COPY package*.json ./

# Install dependencies
RUN npm ci

# Source code mounted via volume

# Expose Vite dev server port
EXPOSE 5173

# Run Vite dev server
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]
```

### How It Works

**Startup:**

```bash
sudo docker compose -f docker-compose.dev.yml up
```

**Rust Backend:**

1. Container starts with `rustlang/rust:nightly` image
2. cargo-watch installed
3. Source code mounted from host
4. `cargo watch -x run` starts
5. Compiles code
6. Runs server
7. **Watches for file changes**

**When you edit code:**

```
1. Save file: src/main.rs
2. cargo-watch detects change
3. Kills running server
4. Recompiles: cargo run
5. Starts new server
6. Total time: ~10 seconds
```

**React Frontend:**

1. Container starts with Node.js
2. npm dependencies installed
3. Source code mounted
4. Vite dev server starts
5. **Watches for file changes**

**When you edit code:**

```
1. Save file: src/App.jsx
2. Vite detects change
3. Hot Module Replacement (HMR)
4. Browser updates without reload
5. Total time: ~100ms
```

### Development Workflow

**Day-to-day development:**

```bash
# Start dev environment (once)
sudo docker compose -f docker-compose.dev.yml up

# Edit files in your editor
vim backend/src/main.rs
vim frontend/src/App.jsx

# See changes automatically
# Backend: Recompiles in ~10 seconds
# Frontend: Updates in ~100ms

# When done
Ctrl+C
```

**No manual rebuilding needed!**

### Production Deployment

**For production:**

```bash
# Use production config (no volumes, optimized builds)
sudo docker compose build
sudo docker compose up -d
```

**Key differences:**

| Aspect        | Development              | Production           |
| ------------- | ------------------------ | -------------------- |
| Config File   | `docker-compose.dev.yml` | `docker-compose.yml` |
| Dockerfile    | `Dockerfile.dev`         | `Dockerfile`         |
| Volumes       | ✅ Source mounted        | ❌ No volumes        |
| Hot Reload    | ✅ cargo-watch, Vite     | ❌ No                |
| Build Time    | First start: ~2 min      | Every deploy: ~5 min |
| Image Size    | ~2GB (includes tooling)  | ~100MB (binary only) |
| Startup Speed | Slow (compiles)          | Fast (runs binary)   |
| Best For      | Development              | Production           |

## Verification

### Test 1: Initial Startup

```bash
$ sudo docker compose -f docker-compose.dev.yml up

[+] Running 3/3
 ✔ Container sqlserver             Started
 ✔ Container rust-backend-dev      Started
 ✔ Container react-frontend-dev    Started

rust-backend-dev-1  | [Running 'cargo run']
rust-backend-dev-1  |    Compiling backend v0.1.0 (/app)
rust-backend-dev-1  |     Finished dev [unoptimized] target(s) in 45.2s
rust-backend-dev-1  |      Running `target/debug/backend`
rust-backend-dev-1  | 🚀 Server running on http://0.0.0.0:3000
```

✅ Containers start  
✅ Code compiles  
✅ Server runs

### Test 2: Hot Reload Backend

**Edit file:**

```bash
$ echo '// Test change' >> backend/src/main.rs
```

**Watch logs:**

```
rust-backend-dev-1  | [Finished running. Exit status: signal: 15 (SIGTERM)]
rust-backend-dev-1  | [Running 'cargo run']
rust-backend-dev-1  |    Compiling backend v0.1.0 (/app)
rust-backend-dev-1  |     Finished dev [unoptimized] target(s) in 8.1s
rust-backend-dev-1  |      Running `target/debug/backend`
rust-backend-dev-1  | 🚀 Server running on http://0.0.0.0:3000
```

✅ Change detected  
✅ Rebuild triggered  
✅ Server restarted  
✅ Total time: ~10 seconds

### Test 3: Hot Reload Frontend

**Edit file:**

```bash
$ vim frontend/src/App.jsx
# Change some text
```

**Browser console:**

```
[vite] hot updated: /src/App.jsx
```

✅ Change detected instantly  
✅ Module hot-replaced  
✅ No page reload  
✅ State preserved

### Test 4: Production Build Still Works

```bash
$ sudo docker compose -f docker-compose.yml build
[+] Building 287.4s (32/32) FINISHED

$ sudo docker compose -f docker-compose.yml up -d
[+] Running 3/3
 ✔ Container rust-backend    Started
 ✔ Container react-frontend  Started
 ✔ Container sqlserver       Started
```

✅ Production build works  
✅ Optimized binaries  
✅ Small image sizes

## Alternative Solutions

### Option A: Use Local Development (No Docker)

**Backend:**

```bash
cd backend
cargo run
```

**Frontend:**

```bash
cd frontend
npm run dev
```

**Pros:**

- Fastest hot reload
- Native tooling
- Familiar workflow
- No Docker overhead

**Cons:**

- Doesn't test Docker configuration
- Environment differences
- Database setup on local machine
- Not realistic deployment simulation

**When to use:** Rapid prototyping, quick iterations

### Option B: Use docker-compose.override.yml

Create `docker-compose.override.yml` (auto-loaded):

```yaml
services:
  rust-backend:
    volumes:
      - ./backend:/app
    command: cargo watch -x run
```

Run:

```bash
docker compose up  # Automatically uses override
```

**Pros:**

- Automatic override
- Don't need -f flag
- Gitignore override for personal preferences

**Cons:**

- Can be confusing (hidden behavior)
- Might accidentally deploy dev config

**When to use:** Solo development with consistent workflow

### Option C: Use Tilt or Skaffold

Development orchestration tools:

**Tiltfile:**

```python
docker_build('backend', './backend',
    dockerfile='Dockerfile.dev',
    live_update=[
        sync('./backend/src', '/app/src'),
    ])
```

**Pros:**

- Advanced development features
- Better live-reload
- Multi-service orchestration
- CI/CD integration

**Cons:**

- Additional tool to learn
- More complex setup
- Overkill for small projects

**When to use:** Large microservice projects, teams

### Option D: Dev Containers

VS Code Dev Containers:

**.devcontainer/devcontainer.json:**

```json
{
  "name": "Rust Backend",
  "dockerComposeFile": "../docker-compose.dev.yml",
  "service": "rust-backend",
  "workspaceFolder": "/app"
}
```

**Pros:**

- IDE integration
- Full development environment in container
- Extensions in container
- Consistent team environment

**Cons:**

- VS Code specific
- Learning curve
- Slower than native

**When to use:** Team standardization, complex environments

## Understanding cargo-watch

### What is cargo-watch?

A Cargo subcommand that watches your source code and runs commands when files change.

**Installation:**

```bash
cargo install cargo-watch
```

**Basic usage:**

```bash
cargo watch -x run      # Run on changes
cargo watch -x test     # Run tests on changes
cargo watch -x build    # Build on changes
```

**How it works:**

1. Monitors file system for changes
2. Detects modifications to `.rs` files
3. Kills current process (if running)
4. Runs specified cargo command
5. Repeats on next change

**Options:**

```bash
cargo watch -x run -w src              # Watch only src/
cargo watch -x run -i "*.tmp"          # Ignore temp files
cargo watch -x "run --release"         # Run with flags
cargo watch -x build -x test -x run    # Chain commands
```

### Why It's Perfect for Docker Dev

**Without cargo-watch:**

- Change file
- Exit container
- Rebuild image
- Restart container
- Wait 5 minutes

**With cargo-watch:**

- Change file
- Wait 10 seconds
- See changes

**In Dockerfile.dev:**

```dockerfile
RUN cargo install cargo-watch
CMD ["cargo", "watch", "-x", "run"]
```

Container automatically rebuilds on file changes.

## Prevention Strategies

### 1. Document Development Workflow

**README.md:**

````markdown
## Development

```bash
# Start dev environment with hot-reload
sudo docker compose -f docker-compose.dev.yml up
```
````

## Production

```bash
# Build optimized images
sudo docker compose build

# Deploy
sudo docker compose up -d
```

````

### 2. Add Makefile

**Makefile:**
```makefile
.PHONY: dev prod build

dev:
	docker compose -f docker-compose.dev.yml up

prod:
	docker compose up -d

build:
	docker compose build

clean:
	docker compose down -v
````

**Usage:**

```bash
make dev      # Development
make prod     # Production
make build    # Build images
make clean    # Clean up
```

### 3. Use .env Files

**.env.development:**

```
COMPOSE_FILE=docker-compose.dev.yml
```

**.env.production:**

```
COMPOSE_FILE=docker-compose.yml
```

### 4. Add VS Code Tasks

**.vscode/tasks.json:**

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Start Dev Environment",
      "type": "shell",
      "command": "docker compose -f docker-compose.dev.yml up",
      "problemMatcher": []
    }
  ]
}
```

## Lessons Learned

1. **Separate dev and prod configs** - Different needs require different setups
2. **Production builds don't hot-reload** - By design, not a bug
3. **Use cargo-watch for Rust dev** - Essential for productivity
4. **Volume mounts enable hot-reload** - Different from production usage
5. **Document which config to use** - Clear instructions prevent confusion

## Timeline

- **Issue Discovered:** November 20, 2025 (during active development)
- **Frustration Period:** ~1 hour (rebuilding repeatedly)
- **Research Time:** ~30 minutes (finding cargo-watch solution)
- **Implementation:** ~45 minutes (creating Dockerfile.dev, docker-compose.dev.yml)
- **Testing:** ~15 minutes (verifying hot-reload works)
- **Total Time:** ~2.5 hours
- **Status:** ✅ Fully Resolved

## Related Commands

**Start development:**

```bash
sudo docker compose -f docker-compose.dev.yml up
```

**Start production:**

```bash
sudo docker compose up -d
```

**Rebuild dev:**

```bash
sudo docker compose -f docker-compose.dev.yml build
```

**View logs:**

```bash
sudo docker compose -f docker-compose.dev.yml logs -f rust-backend
```

**Run command in container:**

```bash
sudo docker exec -it rust-backend-dev cargo test
```

**Check if cargo-watch is installed:**

```bash
sudo docker exec rust-backend-dev cargo watch --version
```
