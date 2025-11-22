# Issue #4: Docker Runtime Error - Binary Not Found

## Problem Description

After successfully building the Docker image, when attempting to run the container using Docker Compose, the backend service failed immediately with:

```
rust-backend-1  | exec: "./backend": stat ./backend: no such file or directory
rust-backend-1 exited with code 1
```

The container would start, attempt to execute the compiled binary, fail to find it, and exit immediately. This happened despite the Docker build completing successfully and showing the binary was created during the build process.

## Technical Details

### What Happened

1. Docker Compose brought up services: `sudo docker compose up`
2. Backend container started
3. Container attempted to run CMD `["./backend"]`
4. Got error: "stat ./backend: no such file or directory"
5. Container exited with code 1
6. Docker Compose tried to restart the container (restart loop)

### Container State Analysis

**Checking what's inside the running container:**

```bash
$ sudo docker compose up -d
$ sudo docker exec rust-backend-1 ls -la /app/
total 8
drwxr-xr-x 3 root root 4096 Nov 20 17:52 .
drwxr-xr-x 1 root root 4096 Nov 20 18:10 ..
drwxr-xr-x 3 root root   96 Nov 20 17:45 src  ← Only source code!
```

**Expected state** (from successful standalone build):

```bash
$ sudo docker run --rm backend ls -la /app/
total 432
drwxr-xr-x 1 root root   4096 Nov 20 17:14 .
drwxr-xr-x 1 root root   4096 Nov 20 17:41 ..
-rwxr-xr-x 1 root root 431320 Nov 20 17:14 backend  ← Binary should be here!
drwxr-xr-x 3 root root     96 Nov 20 17:45 src
```

**Key observation:** Binary was missing from the runtime container when using Docker Compose, but present when running the image directly.

### Docker Compose Configuration

**docker-compose.yml:**

```yaml
version: "3.8"

services:
  rust-backend:
    build: ./backend
    container_name: rust-backend
    ports:
      - "3000:3000"
    volumes:
      - ./backend:/app # ← This is the problem!
    depends_on:
      - sqlserver
    environment:
      - DATABASE_URL=Server=sqlserver,1433;Database=zoo_db;User Id=sa;Password=YourStrong@Passw0rd;
```

**Dockerfile (for reference):**

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
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend /app/backend  # ← Binary copied here
RUN chmod +x /app/backend
EXPOSE 3000
CMD ["/app/backend"]  # ← Trying to run this
```

## Root Cause Analysis

### The Volume Mount Overwrite

Docker volume mounts work by **overlaying** the host directory onto the container directory. This happens **after** the image is built but **before** the container starts.

**Step-by-step breakdown:**

1. **Build Time:**

   ```
   [Runtime Stage]
   COPY --from=builder /app/target/release/backend /app/backend
   ```

   Result: Image has `/app/backend` binary inside

2. **Container Start:**

   ```yaml
   volumes:
     - ./backend:/app
   ```

   Result: Host's `./backend` directory **replaces** container's `/app` directory

3. **What's in Host's `./backend` Directory:**

   ```
   ./backend/
   ├── Cargo.toml
   ├── Cargo.lock
   ├── Dockerfile
   └── src/
       ├── main.rs
       └── handlers/
   ```

   **Notice:** No compiled binary! The binary only exists inside the Docker image.

4. **Container Runtime:**
   - Docker mounts `./backend` from host
   - This **overwrites** the `/app` directory in container
   - `/app/backend` binary (from image) is now gone
   - Only source code is visible
   - CMD tries to run `./backend`
   - File doesn't exist → error

### Visual Representation

**Without Volume Mount:**

```
Container /app:
├── backend (executable)  ← From Docker COPY
└── src/ (if copied)
```

**With Volume Mount (`./backend:/app`):**

```
Container /app:  ← Replaced by host directory
├── Cargo.toml
├── Cargo.lock
├── Dockerfile
└── src/
    └── main.rs

backend executable is GONE!
```

### Why This Happened

**Intended Purpose of Volume Mount:**

The developer added the volume mount thinking it would enable hot-reload during development:

```yaml
volumes:
  - ./backend:/app # "Let the container see code changes"
```

**The Problem:**

This works for interpreted languages (Node.js, Python) where you run source directly:

```yaml
# Works for Node.js:
volumes:
  - ./app:/app
command: node index.js # Runs source directly
```

But for **compiled** languages like Rust:

- Source code needs to be compiled → binary
- Binary is created during Docker build
- Source code alone can't be executed
- Volume mount removes the compiled binary

### Production vs Development Confusion

This is a **production Docker Compose configuration** trying to do **development tasks**.

**What it was attempting:**

- Run compiled production binary
- Enable hot-reload for development
- Both at the same time

**Reality:**

- Can't have both simultaneously
- Need separate configurations
- Production: no volumes, run binary
- Development: volumes + rebuild on change

## How the Error Occurred

**Development Process:**

1. **Developer's Local Setup:**

   ```bash
   $ cargo run  # Works fine locally
       Compiling backend v0.1.0
       Finished dev [unoptimized] target(s)
       Running `target/debug/backend`
   Server running on 0.0.0.0:3000
   ```

2. **Docker Build:**

   ```bash
   $ sudo docker build -t backend ./backend
   # Build succeeds, binary created
   ```

3. **Test Docker Image Directly:**

   ```bash
   $ sudo docker run -p 3000:3000 backend
   Server running on 0.0.0.0:3000  # Works!
   ```

4. **Use Docker Compose:**
   ```bash
   $ sudo docker compose up
   rust-backend-1  | exec: "./backend": stat ./backend: no such file or directory
   rust-backend-1 exited with code 1
   ```

**Why the difference?**

| Command              | Volume Mount        | Binary Present | Result   |
| -------------------- | ------------------- | -------------- | -------- |
| `docker run backend` | ❌ None             | ✅ Yes         | ✅ Works |
| `docker compose up`  | ✅ `./backend:/app` | ❌ Overwritten | ❌ Fails |

The Docker Compose configuration had the volume mount, which overwrote the binary.

## Solution Implementation

### Solution 1: Remove Volume Mount (Production)

For production deployment, remove the volume mount entirely.

**Updated docker-compose.yml:**

```yaml
version: "3.8"

services:
  rust-backend:
    build: ./backend
    container_name: rust-backend
    ports:
      - "3000:3000"
    # volumes:  ← REMOVED!
    #   - ./backend:/app
    depends_on:
      - sqlserver
    environment:
      - DATABASE_URL=Server=sqlserver,1433;Database=zoo_db;User Id=sa;Password=YourStrong@Passw0rd;

  react-frontend:
    build: ./frontend/static
    container_name: react-frontend
    ports:
      - "80:80"
    depends_on:
      - rust-backend

  sqlserver:
    image: mcr.microsoft.com/mssql/server:2022-latest
    container_name: sqlserver
    environment:
      - ACCEPT_EULA=Y
      - SA_PASSWORD=YourStrong@Passw0rd
    ports:
      - "1433:1433"
```

**How this works:**

1. Docker build creates image with binary at `/app/backend`
2. Docker Compose starts container from image
3. No volume mount → container's `/app` directory intact
4. Binary exists at `/app/backend`
5. CMD `["/app/backend"]` executes successfully

**Rebuilding after code changes:**

```bash
# Rebuild image
sudo docker compose build

# Restart containers with new image
sudo docker compose up -d
```

### Solution 2: Create Development Compose File (Better)

Create separate configuration for development with hot-reload.

**docker-compose.dev.yml:**

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
      - ./backend:/app # Now OK because we use cargo-watch
    depends_on:
      - sqlserver
    environment:
      - DATABASE_URL=Server=sqlserver,1433;Database=zoo_db;User Id=sa;Password=YourStrong@Passw0rd;

  # ... other services
```

**Dockerfile.dev (for hot-reload):**

```dockerfile
FROM rustlang/rust:nightly
WORKDIR /app

# Install cargo-watch for hot-reload
RUN cargo install cargo-watch

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies
RUN cargo fetch

# Volume will mount source code here
# No need to COPY src - it comes from volume

# Run with auto-reload
CMD ["cargo", "watch", "-x", "run"]
```

**How to use:**

Development:

```bash
sudo docker compose -f docker-compose.dev.yml up
```

Production:

```bash
sudo docker compose up
```

### Why Solution 2 Is Better

**Separation of Concerns:**

| Aspect     | docker-compose.yml | docker-compose.dev.yml |
| ---------- | ------------------ | ---------------------- |
| Purpose    | Production         | Development            |
| Volumes    | None               | Source code mounted    |
| Binary     | Pre-compiled       | Built on-demand        |
| Changes    | Require rebuild    | Auto-reload            |
| Speed      | Fast startup       | Slow first start       |
| Image Size | Small (~100MB)     | Large (~2GB)           |

**Development Workflow:**

```bash
# Start dev environment
sudo docker compose -f docker-compose.dev.yml up

# Edit code in ./backend/src/main.rs
# cargo-watch detects change
# Automatically recompiles
# Server restarts
# Changes visible immediately
```

**Production Deployment:**

```bash
# Build optimized images
sudo docker compose build

# Deploy
sudo docker compose up -d

# No volumes = no local dependencies
# Fully self-contained images
```

## Verification

### Verify Production Setup

After removing volume mount:

```bash
$ sudo docker compose down
$ sudo docker compose build
$ sudo docker compose up -d
$ sudo docker compose logs rust-backend
```

**Expected output:**

```
rust-backend-1  | Server running on 0.0.0.0:3000
```

**Check binary exists:**

```bash
$ sudo docker exec rust-backend ls -la /app/
total 432
-rwxr-xr-x 1 root root 431320 Nov 20 17:14 backend  ← Binary present!
```

**Test endpoint:**

```bash
$ curl http://localhost:3000/message
Hello from backend!
```

✅ Container stays running  
✅ Binary present and executable  
✅ Server responds to requests

### Verify Development Setup

With `docker-compose.dev.yml`:

```bash
$ sudo docker compose -f docker-compose.dev.yml up -d
$ sudo docker compose -f docker-compose.dev.yml logs -f rust-backend
```

**Expected output:**

```
rust-backend-dev-1  | [Running 'cargo run']
rust-backend-dev-1  |    Compiling backend v0.1.0 (/app)
rust-backend-dev-1  |     Finished dev [unoptimized] target(s)
rust-backend-dev-1  |      Running `target/debug/backend`
rust-backend-dev-1  | Server running on 0.0.0.0:3000
```

**Edit code:**

```bash
$ echo '// Modified' >> ./backend/src/main.rs
```

**Watch logs:**

```
rust-backend-dev-1  | [Finished running. Exit status: signal: 15]
rust-backend-dev-1  | [Running 'cargo run']
rust-backend-dev-1  |    Compiling backend v0.1.0 (/app)
rust-backend-dev-1  |     Finished dev [unoptimized] target(s)
rust-backend-dev-1  |      Running `target/debug/backend`
rust-backend-dev-1  | Server running on 0.0.0.0:3000
```

✅ Hot-reload working  
✅ Changes detected automatically  
✅ Server restarts on change

## Alternative Solutions

### Option A: Named Volumes

Use Docker-managed volume instead of bind mount:

```yaml
volumes:
  - backend-data:/app
```

**Pros:**

- Persists data
- Doesn't overwrite container contents

**Cons:**

- Data is Docker-managed, not on host
- Can't edit files easily
- Not good for development

**When to use:** Database persistence, cache storage

### Option B: Mount Specific Subdirectory

Mount only what's needed:

```yaml
volumes:
  - ./backend/src:/app/src:ro # Read-only source
```

**Pros:**

- Doesn't overwrite binary
- Container can access source

**Cons:**

- Still need to rebuild to see changes
- Complexity increases

**When to use:** Debugging, inspecting source in container

### Option C: Development Container

Use VS Code Dev Containers:

**.devcontainer/devcontainer.json:**

```json
{
  "name": "Rust",
  "image": "rustlang/rust:nightly",
  "mounts": [
    "source=${localWorkspaceFolder}/backend,target=/workspace,type=bind"
  ],
  "postCreateCommand": "cargo build"
}
```

**Pros:**

- Full IDE integration
- Hot-reload built-in
- Consistent environment

**Cons:**

- Requires VS Code
- Learning curve

**When to use:** Team development, onboarding

## Understanding Volume Mounts

### Bind Mounts vs Volumes

**Bind Mount:**

```yaml
volumes:
  - ./backend:/app # Host path : Container path
```

- Links specific host directory
- Bidirectional sync
- Host directory **replaces** container directory

**Named Volume:**

```yaml
volumes:
  - backend-data:/app

volumes:
  backend-data:
```

- Docker-managed storage
- Persists across container restarts
- **Initializes from container** if empty

### Mount Behavior

**When container starts:**

1. Image has `/app/backend` binary
2. Mount `./backend:/app` specified
3. Docker **replaces** `/app` with host's `./backend`
4. Original `/app/backend` is **hidden** (not deleted)
5. When you unmount, original contents reappear

**Demonstration:**

```bash
# Start without mount
$ docker run --rm backend ls /app/
backend  # Binary is here

# Start with mount
$ docker run --rm -v ./backend:/app backend ls /app/
Cargo.toml  Dockerfile  src  # Only host files
```

### Multi-Stage Build Implications

**Build stages:**

```dockerfile
FROM rust:nightly AS builder  # Stage 1: Build
WORKDIR /app
# ... compile binary ...

FROM debian:bookworm-slim       # Stage 2: Runtime
COPY --from=builder /app/target/release/backend /app/backend
```

**What's in final image:**

- Only what's in Stage 2
- Binary from `COPY --from=builder`
- No build tools
- No source code (unless you COPY it)

**Volume mount effect:**

```yaml
volumes:
  - ./backend:/app
```

- Replaces `/app` from Stage 2
- Binary (from COPY) is gone
- Source code (from host) appears

## Prevention Strategies

### 1. Use Naming Conventions

```
docker-compose.yml       → Production (no volumes)
docker-compose.dev.yml   → Development (with volumes)
docker-compose.test.yml  → Testing
```

### 2. Document Volume Usage

Add comments:

```yaml
services:
  rust-backend:
    build: ./backend
    # NO volumes in production - binary is in image
    # For development, use docker-compose.dev.yml
```

### 3. Validation Script

**check-config.sh:**

```bash
#!/bin/bash
if grep -q "volumes:" docker-compose.yml | grep -q "./backend:/app"; then
  echo "❌ ERROR: Volume mount in production config!"
  exit 1
fi
echo "✅ Production config OK"
```

### 4. CI/CD Checks

In `.github/workflows/validate.yml`:

```yaml
- name: Check for volume mounts in production
  run: |
    if grep -A 5 "rust-backend:" docker-compose.yml | grep "volumes:"; then
      echo "Volume mounts not allowed in production compose"
      exit 1
    fi
```

## Lessons Learned

1. **Volume mounts replace, not merge** - Always aware of what gets overwritten
2. **Compiled languages need binaries** - Can't run source code directly
3. **Separate dev and prod configs** - Different needs require different setups
4. **Test Docker Compose, not just Docker** - Compose adds complexity
5. **Understand mount behavior** - Know bind mounts vs volumes

## Timeline

- **Issue Discovered:** November 20, 2025 (after fixing Rust edition issue)
- **Diagnosis Time:** ~5 minutes (checked container contents, found missing binary)
- **Understanding Root Cause:** ~10 minutes (realized volume mount was overwriting)
- **Solution Time:** ~5 minutes (removed volume mount line)
- **Verification:** ~2 minutes (rebuild and test)
- **Creating Dev Setup:** ~20 minutes (wrote Dockerfile.dev, docker-compose.dev.yml)
- **Total Time:** ~42 minutes
- **Status:** ✅ Fully Resolved

## Related Commands

**Check what's in a running container:**

```bash
sudo docker exec <container> ls -la /app/
```

**Compare image vs container:**

```bash
# What's in the image
sudo docker run --rm <image> ls /app/

# What's in the running container
sudo docker exec <container> ls /app/
```

**Inspect volume mounts:**

```bash
sudo docker inspect <container> | grep -A 10 "Mounts"
```

**List volumes:**

```bash
sudo docker volume ls
```

**Remove all volumes:**

```bash
sudo docker volume prune
```

**Rebuild without cache:**

```bash
sudo docker compose build --no-cache
```

**View compose configuration:**

```bash
sudo docker compose config
```
