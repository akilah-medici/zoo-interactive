# Issue #5: Frontend Docker Build Failed - Dockerfile Not Found

## Problem Description

When attempting to build the Docker Compose services, the frontend service failed to build with the following error:

```
failed to solve: failed to read dockerfile: open /var/lib/docker/tmp/buildkit-mount
<RANDOM_ID>/Dockerfile: no such file or directory
```

The Docker Compose build process was looking for a Dockerfile in the wrong location, causing the build to fail before it could even start.

## Technical Details

### What Happened

1. Ran `sudo docker compose build`
2. Backend service built successfully
3. Frontend service build started
4. Docker looked for Dockerfile in specified location
5. Dockerfile not found
6. Build failed with "no such file or directory" error

### docker-compose.yml Configuration

**Original (broken) configuration:**

```yaml
version: "3.8"

services:
  rust-backend:
    build: ./backend
    container_name: rust-backend
    ports:
      - "3000:3000"

  react-frontend:
    build: ./frontend/static # ← Problem: Wrong path
    container_name: react-frontend
    ports:
      - "80:80"
    depends_on:
      - rust-backend
```

### Actual Directory Structure

```
/home/nijiuu/Documentos/Desafio-CIEE/
├── backend/
│   ├── Dockerfile        ← Exists
│   ├── src/
│   └── Cargo.toml
├── frontend/
│   └── static/           ← Frontend code is here
│       ├── src/
│       ├── index.html
│       ├── package.json
│       └── vite.config.js
│       └── Dockerfile    ← But Dockerfile is HERE
```

### The Path Problem

**What Docker Compose does:**

When you specify `build: ./frontend/static`, Docker:

1. Changes to that directory: `cd ./frontend/static`
2. Looks for `Dockerfile` in the current directory
3. Uses that directory as the **build context**

**Build context:**

- All files Docker can access during build
- Specified directory and its subdirectories
- Docker can't access files outside this

**Expected structure:**

```
./frontend/static/
├── Dockerfile           ← Docker expects it here
├── src/
├── index.html
└── package.json
```

**Actual situation:**

```
./frontend/static/
├── src/                 ← No Dockerfile!
├── index.html
└── package.json
```

Dockerfile **didn't exist yet** in `./frontend/static/`.

## Root Cause Analysis

### Multiple Issues Combined

#### Issue 1: Dockerfile Didn't Exist

At the time of this error, the frontend Dockerfile hadn't been created yet. The docker-compose.yml was configured, but the corresponding Dockerfile was missing.

**Timeline of events:**

1. Backend Dockerfile created
2. docker-compose.yml created with both services
3. Frontend Dockerfile **not yet created**
4. Attempted to build → error

#### Issue 2: Directory Structure Confusion

The directory structure had confusion:

```
Was it:
  frontend/Dockerfile ?
  frontend/static/Dockerfile ?
  static/Dockerfile ?
```

**The problem:** Inconsistent organization where:

- Backend: `backend/Dockerfile` (clean)
- Frontend: `frontend/static/...` (nested)

This nesting created confusion about where Dockerfile should live.

### Why This Structure Existed

**Original thought process:**

"I have a backend and multiple potential frontends (React, Angular, Vue), so I'll organize like this:"

```
backend/           ← One backend
frontend/
  ├── static/      ← React frontend
  ├── admin/       ← (Future) Admin panel
  └── mobile/      ← (Future) Mobile app
```

**Reality:**

For this project, there's only one frontend. The extra nesting added complexity without benefit.

### Docker Build Context Implications

**When Dockerfile is in a subdirectory:**

```yaml
build: ./frontend/static
```

Docker's build context is `./frontend/static/`. During build:

```dockerfile
COPY package.json ./    # Copies from ./frontend/static/package.json
COPY src ./src          # Copies from ./frontend/static/src/
```

All paths are relative to `./frontend/static/`.

**Problem if Dockerfile is elsewhere:**

- Build context: `./frontend/static/`
- Dockerfile: `./frontend/Dockerfile`
- Result: Can't find Dockerfile

## How the Error Occurred

**Step-by-step breakdown:**

1. **Initial Project Setup:**

   ```bash
   mkdir -p backend frontend/static
   cd backend && cargo init
   cd ../frontend/static && npm create vite@latest . -- --template react
   ```

2. **Created docker-compose.yml:**

   ```yaml
   services:
     rust-backend:
       build: ./backend # Works - has Dockerfile
     react-frontend:
       build: ./frontend/static # Expects Dockerfile here
   ```

3. **Created Backend Dockerfile:**

   ```bash
   touch backend/Dockerfile
   # ... wrote Dockerfile content ...
   ```

4. **Forgot to Create Frontend Dockerfile:**
   Frontend Dockerfile was never created.

5. **Attempted to Build:**
   ```bash
   $ sudo docker compose build
   [+] Building 42.1s (18/18) FINISHED    rust-backend
   [+] Building 0.1s (0/0) FINISHED       react-frontend
   failed to solve: failed to read dockerfile
   ```

### Why It Wasn't Noticed Earlier

- Backend was built first (worked fine)
- Error only appeared when building frontend
- Focus was on getting backend working
- Frontend Dockerfile creation was delayed

## Solution Implementation

### Solution 1: Create Dockerfile in Expected Location (Initial Fix)

Create the missing Dockerfile where Docker expects it.

**Created: `./frontend/static/Dockerfile`**

```dockerfile
# Build stage
FROM node:20-alpine AS builder
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci

# Copy source code
COPY . .

# Build the application
RUN npm run build

# Production stage
FROM nginx:alpine
WORKDIR /usr/share/nginx/html

# Remove default nginx static assets
RUN rm -rf ./*

# Copy built assets from builder stage
COPY --from=builder /app/dist .

# Copy nginx configuration if you have one
# COPY nginx.conf /etc/nginx/nginx.conf

# Expose port 80
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
```

**Result:** Docker Compose can now find the Dockerfile and build successfully.

**Verification:**

```bash
$ sudo docker compose build react-frontend
[+] Building 67.2s (16/16) FINISHED
 => [builder 1/6] FROM docker.io/library/node:20-alpine
 => [builder 5/6] COPY . .
 => [builder 6/6] RUN npm run build
 => [stage-1] COPY --from=builder /app/dist .
 => exporting to image
```

✅ Build completes successfully

### Solution 2: Restructure Directories (Better Long-term Solution)

Later in the project, the directory structure was reorganized to be cleaner:

**Before:**

```
frontend/
  └── static/
      ├── Dockerfile
      ├── src/
      ├── package.json
      └── ...
```

**After:**

```
frontend/
  ├── Dockerfile
  ├── src/
  ├── package.json
  └── ...
```

**Updated docker-compose.yml:**

```yaml
services:
  rust-backend:
    build: ./backend

  react-frontend:
    build: ./frontend # Simplified path
    container_name: react-frontend
    ports:
      - "80:80"
```

**Why this is better:**

- Consistent structure (backend/ and frontend/ at same level)
- Simpler paths
- Easier to understand
- Matches standard project layouts

**Migration steps:**

```bash
# Move all files up one level
cd frontend
mv static/* .
mv static/.gitignore . 2>/dev/null || true

# Remove now-empty static directory
rmdir static

# Update docker-compose.yml
sed -i 's|./frontend/static|./frontend|g' docker-compose.yml
```

### Solution 3: Alternative - Explicit Dockerfile Path

If you want to keep the nested structure but have Dockerfile elsewhere:

**Option A: Specify dockerfile explicitly:**

```yaml
react-frontend:
  build:
    context: ./frontend/static
    dockerfile: ../Dockerfile # One level up
```

**Option B: Use root context:**

```yaml
react-frontend:
  build:
    context: ./frontend
    dockerfile: Dockerfile
```

The build context is `./frontend/`, and it finds `Dockerfile` there.

**When to use:**

- Monorepo with shared Dockerfiles
- Multiple variants (Dockerfile.dev, Dockerfile.prod)
- Complex build requirements

## Verification

### Verify Solution 1 (Dockerfile in static/)

```bash
$ ls -la frontend/static/Dockerfile
-rw-r--r-- 1 user user 523 Nov 20 18:25 frontend/static/Dockerfile

$ sudo docker compose build react-frontend
[+] Building 67.2s (16/16) FINISHED
 => exporting to image

$ sudo docker compose up -d react-frontend
[+] Running 1/1
 ✔ Container react-frontend  Started
```

✅ Dockerfile found  
✅ Build successful  
✅ Container starts

### Verify Solution 2 (Restructured)

```bash
$ tree -L 2 frontend/
frontend/
├── Dockerfile
├── node_modules/
├── package.json
├── package-lock.json
├── public/
├── src/
├── index.html
└── vite.config.js

$ sudo docker compose build react-frontend
[+] Building 45.3s (16/16) FINISHED
 => exporting to image

$ curl http://localhost:80
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    ...
```

✅ Clean structure  
✅ Build works  
✅ Frontend accessible

## Alternative Solutions

### Option A: Use a Standard Structure Generator

Use create-react-app or Vite with Docker template:

```bash
npm create vite@latest frontend -- --template react
cd frontend
```

This creates a standard structure that's well-documented and expected.

**Pros:**

- Follows conventions
- Community support
- Examples available

**Cons:**

- Less customization
- Opinionated structure

### Option B: Monorepo Structure

Use a monorepo tool like Nx or Turborepo:

```
repo/
├── apps/
│   ├── backend/
│   │   └── Dockerfile
│   └── frontend/
│       └── Dockerfile
├── libs/
│   └── shared/
└── docker-compose.yml
```

**Pros:**

- Code sharing between apps
- Unified tooling
- Better for multiple frontends

**Cons:**

- More complex setup
- Steeper learning curve
- May be overkill for small projects

### Option C: Keep Nested, Document Clearly

If you must keep nested structure:

**README.md:**

```markdown
## Project Structure

frontend/
static/ ← React application
Dockerfile ← Frontend Docker build
src/ ← React components
```

**docker-compose.yml with comments:**

```yaml
react-frontend:
  build: ./frontend/static # React app is in nested directory
```

**Pros:**

- Flexibility
- Can organize however you want

**Cons:**

- Requires documentation
- Non-standard
- Confusing for new contributors

## Understanding Docker Build Context

### What is Build Context?

The build context is the set of files Docker can access during a build.

**Example:**

```yaml
build: ./frontend
```

Build context = `./frontend/` directory and all subdirectories.

**During build:**

```dockerfile
COPY package.json ./      # Copies ./frontend/package.json
COPY src ./src            # Copies ./frontend/src/* to /app/src
```

All COPY/ADD commands are relative to build context.

### Context vs Dockerfile Location

**These are different things:**

1. **Build Context:** Where files are (directory)
2. **Dockerfile Location:** Where instructions are (file)

**Usually they're the same:**

```yaml
build: ./frontend
# Context: ./frontend/
# Dockerfile: ./frontend/Dockerfile
```

**But they can differ:**

```yaml
build:
  context: ./frontend
  dockerfile: ../docker/Frontend.dockerfile
# Context: ./frontend/
# Dockerfile: ./docker/Frontend.dockerfile
```

### Why Context Matters

**You CAN'T access files outside context:**

```dockerfile
# If context is ./frontend:
COPY ../backend/shared/utils.rs ./  # ❌ ERROR!
# Can't access ../backend (outside context)
```

**Solution:** Change context:

```yaml
build:
  context: .
  dockerfile: frontend/Dockerfile
```

Now context is root, so:

```dockerfile
COPY backend/shared/utils.rs ./  # ✅ OK
COPY frontend/src ./src          # ✅ OK
```

### .dockerignore

Like `.gitignore`, but for Docker build context:

**frontend/.dockerignore:**

```
node_modules
.git
.env
*.log
dist
.cache
```

Files matching these patterns are excluded from build context, making builds faster and images smaller.

## Prevention Strategies

### 1. Create Dockerfiles Early

When setting up docker-compose.yml, create Dockerfiles immediately:

```bash
# Create compose file
touch docker-compose.yml

# Create Dockerfiles
touch backend/Dockerfile
touch frontend/Dockerfile

# Now write the configurations
```

### 2. Use Consistent Structure

Follow a pattern:

```
service-name/
├── Dockerfile
├── src/
└── config files
```

All services follow the same pattern.

### 3. Validate Before Building

**validate.sh:**

```bash
#!/bin/bash
for service in $(docker compose config --services); do
  context=$(docker compose config | grep -A 5 "$service:" | grep context | awk '{print $2}')
  if [ ! -f "$context/Dockerfile" ]; then
    echo "❌ Missing Dockerfile for $service in $context"
    exit 1
  fi
done
echo "✅ All Dockerfiles present"
```

### 4. Documentation

**README.md:**

```markdown
## Directory Structure
```

project/
├── backend/ # Rust API server
│ └── Dockerfile
├── frontend/ # React application
│ └── Dockerfile
└── docker-compose.yml

```

## Building

Each service directory must contain a Dockerfile.
```

## Lessons Learned

1. **Create Dockerfiles before configuring Compose** - Avoid referencing non-existent files
2. **Keep structure simple and consistent** - Don't over-nest without good reason
3. **Understand build context** - Know what files Docker can access
4. **Validate configurations** - Check files exist before running builds
5. **Follow conventions** - Standard layouts are easier for everyone

## Timeline

- **Issue Discovered:** November 20, 2025
- **Diagnosis Time:** ~2 minutes (error message was clear)
- **Initial Fix:** ~15 minutes (created Dockerfile in expected location)
- **Restructuring:** ~10 minutes (moved files, updated compose)
- **Total Time:** ~27 minutes
- **Status:** ✅ Fully Resolved

## Related Commands

**Check if Dockerfile exists:**

```bash
ls -la ./frontend/static/Dockerfile
```

**View build context that Docker will use:**

```bash
sudo docker compose config
```

**Build specific service:**

```bash
sudo docker compose build react-frontend
```

**Build with specific Dockerfile:**

```bash
docker build -f path/to/Dockerfile -t myimage .
```

**See what files are in build context:**

```bash
# In the context directory:
find . -type f | grep -v node_modules
```

**Validate docker-compose.yml syntax:**

```bash
sudo docker compose config --quiet
echo $?  # Should be 0 if valid
```

**List all services in compose file:**

```bash
sudo docker compose config --services
```
