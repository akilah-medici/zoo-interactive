# Issue #10: Connection Reset After Directory Restructure

## Problem Description

After restructuring the project directory from `frontend/static/` to `frontend/`, the Docker Compose configuration was updated but the application wouldn't start properly. When attempting to run the containers, the backend would crash with connection errors:

```
rust-backend-1  | Error: Connection reset by peer (os error 104)
rust-backend-1 exited with code 1
```

The issue occurred because the wrong Docker Compose file was being used after the restructure.

## Technical Details

### What Happened

**Sequence of events:**

1. **Original structure:**

   ```
   frontend/
     └── static/
         ├── Dockerfile
         ├── Dockerfile.dev
         ├── src/
         └── package.json
   ```

2. **Updated structure:**

   ```
   frontend/
     ├── Dockerfile
     ├── Dockerfile.dev
     ├── src/
     └── package.json
   ```

3. **Updated docker-compose.dev.yml:**

   ```yaml
   react-frontend:
     build:
       context: ./frontend # Changed from ./frontend/static
   ```

4. **Ran old command:**

   ```bash
   sudo docker compose up
   ```

5. **Error occurred:**
   ```
   rust-backend-1  | Error: Connection reset by peer (os error 104)
   ```

### What Was Really Happening

**The confusion:**

After creating `docker-compose.dev.yml` for development, there were now **two** compose files:

1. **docker-compose.yml** - Production config (old paths)
2. **docker-compose.dev.yml** - Development config (new paths)

**docker-compose.yml (production - not updated):**

```yaml
services:
  react-frontend:
    build: ./frontend/static # ← Old path, doesn't exist anymore!
```

**docker-compose.dev.yml (development - updated):**

```yaml
services:
  react-frontend:
    build: ./frontend # ← New path, correct!
```

**What happened when running `sudo docker compose up`:**

1. Docker Compose loaded `docker-compose.yml` (default)
2. Tried to build frontend from `./frontend/static`
3. Path doesn't exist (files moved to `./frontend`)
4. Build failed or used stale cached image
5. Container networking misconfigured
6. Backend couldn't connect properly
7. Connection reset errors

### Error Analysis

**"Connection reset by peer" meaning:**

```
Error: Connection reset by peer (os error 104)
```

This error occurs when:

- A network connection is established
- Then abruptly closed by the remote side
- Or network configuration is broken

**In this context:**

- Backend tried to connect to database (or other service)
- Network configuration was wrong due to build failures
- Or container networking broken due to partial build
- Connection attempts failed

**Why networking broke:**

1. Frontend build failed (wrong path)
2. Docker Compose continued anyway
3. Network configuration incomplete
4. Services couldn't communicate properly
5. Backend connection attempts → reset

## Root Cause Analysis

### The Real Problem

The issue wasn't actually about connections being reset. The **root cause** was:

1. **Using wrong Docker Compose file**
2. **File paths in compose file didn't match actual structure**
3. **Didn't notice because builds were cached**

### Timeline of Confusion

**Phase 1: Original Setup**

```
frontend/static/  ← Files here
docker-compose.yml points to ./frontend/static  ✅
```

**Phase 2: Create Dev Config**

```
Created docker-compose.dev.yml
Updated to point to ./frontend  (planning to restructure)
```

**Phase 3: Restructure**

```
Moved frontend/static/* to frontend/
docker-compose.dev.yml ✅ Correct
docker-compose.yml ❌ Still pointing to old path
```

**Phase 4: Run Wrong Config**

```bash
sudo docker compose up  # Uses docker-compose.yml (wrong paths)
# Instead of:
sudo docker compose -f docker-compose.dev.yml up
```

Result: Errors!

### Docker Compose File Selection

**How Docker Compose chooses files:**

**Default behavior (no -f flag):**

```bash
docker compose up
```

Looks for files in this order:

1. `docker-compose.yml`
2. `docker-compose.yaml`
3. `compose.yml`
4. `compose.yaml`

Uses first one found.

**Explicit file:**

```bash
docker compose -f docker-compose.dev.yml up
```

Uses specified file.

**Multiple files:**

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml up
```

Merges files (dev overrides prod).

### Why Caching Made It Worse

Docker caches build layers. When path was wrong:

1. Docker tried to build from `./frontend/static`
2. Path doesn't exist
3. **But cached layers from old build still exist**
4. Docker used cached image
5. **Old, stale frontend image ran**
6. Networking issues arose from version mismatch

**To developer:** Everything seemed to build, but wasn't actually rebuilding.

### The Directory Restructure Motivation

**Why restructure?**

**Before (confusing):**

```
frontend/
  └── static/         ← Why this extra level?
      ├── src/
      ├── public/
      └── package.json

backend/              ← No extra level
  ├── src/
  └── Cargo.toml
```

**After (consistent):**

```
frontend/             ← Clean, same level as backend
  ├── src/
  ├── public/
  └── package.json

backend/
  ├── src/
  └── Cargo.toml
```

**Benefits:**

- ✅ Consistent structure
- ✅ Clearer organization
- ✅ Standard convention
- ✅ Easier to navigate

## How the Error Occurred

**Step-by-step breakdown:**

1. **Created development config:**

   ```bash
   vim docker-compose.dev.yml
   # Set path to ./frontend (planning ahead for restructure)
   ```

2. **Tested dev config:**

   ```bash
   sudo docker compose -f docker-compose.dev.yml up
   # Didn't work yet - files still in frontend/static
   ```

3. **Restructured directories:**

   ```bash
   cd frontend
   mv static/* .
   mv static/.* . 2>/dev/null
   rmdir static
   ```

4. **Updated docker-compose.dev.yml:**

   ```yaml
   # Already pointed to ./frontend - now correct!
   ```

5. **Tested:**

   ```bash
   sudo docker compose -f docker-compose.dev.yml up
   # Worked! ✅
   ```

6. **Later, forgot which command:**

   ```bash
   # Muscle memory from earlier
   sudo docker compose up  # ❌ Used wrong file!
   ```

7. **Got errors:**

   ```
   rust-backend-1  | Error: Connection reset by peer
   ```

8. **Confusion:**
   "It was working! Why is it broken now?"

9. **Debugging:**

   - Checked code: ✅ Fine
   - Checked Docker: ❌ Using wrong compose file
   - Checked paths: ❌ docker-compose.yml not updated

10. **Realization:**
    "I'm running the production config with old paths!"

11. **Remembered:**
    "I need to use docker-compose.dev.yml for development"

12. **Fixed:**
    ```bash
    sudo docker compose -f docker-compose.dev.yml up
    # Works! ✅
    ```

### Why This Was Confusing

**Multiple sources of truth:**

1. **docker-compose.yml** - Production (old paths)
2. **docker-compose.dev.yml** - Development (new paths)
3. **Actual file structure** - Matches dev, not prod

**Inconsistent state:**

| File                   | Points To           | Exists?       |
| ---------------------- | ------------------- | ------------- |
| docker-compose.yml     | `./frontend/static` | ❌ No (moved) |
| docker-compose.dev.yml | `./frontend`        | ✅ Yes        |
| Actual files           | `./frontend/*`      | ✅ Yes        |

**Result:** Depending on which compose file you used, things worked or didn't.

## Solution Implementation

### Immediate Fix: Use Correct Compose File

**Development:**

```bash
sudo docker compose -f docker-compose.dev.yml up
```

✅ Uses correct paths  
✅ Hot reload works  
✅ Everything runs

### Long-term Fix 1: Update Production Config

Update `docker-compose.yml` to match new structure:

**docker-compose.yml:**

```yaml
version: "3.8"

services:
  rust-backend:
    build: ./backend
    container_name: rust-backend
    ports:
      - "3000:3000"

  react-frontend:
    build: ./frontend # ← Updated from ./frontend/static
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

Now both files have correct paths.

### Long-term Fix 2: Create Helper Scripts

**dev.sh:**

```bash
#!/bin/bash
echo "Starting development environment..."
sudo docker compose -f docker-compose.dev.yml up
```

**prod.sh:**

```bash
#!/bin/bash
echo "Starting production environment..."
sudo docker compose up
```

**Make executable:**

```bash
chmod +x dev.sh prod.sh
```

**Usage:**

```bash
./dev.sh    # Development
./prod.sh   # Production
```

No confusion about which command to use!

### Long-term Fix 3: Add Makefile

**Makefile:**

```makefile
.PHONY: dev prod build clean help

help:
	@echo "Available commands:"
	@echo "  make dev    - Start development environment with hot-reload"
	@echo "  make prod   - Start production environment"
	@echo "  make build  - Build production images"
	@echo "  make clean  - Stop all containers and remove volumes"

dev:
	@echo "Starting development environment..."
	docker compose -f docker-compose.dev.yml up

prod:
	@echo "Starting production environment..."
	docker compose up -d

build:
	@echo "Building production images..."
	docker compose build

clean:
	@echo "Cleaning up..."
	docker compose -f docker-compose.dev.yml down -v
	docker compose down -v
```

**Usage:**

```bash
make help   # Show commands
make dev    # Development
make prod   # Production
```

### Long-term Fix 4: Environment-Based Selection

**Use .env file:**

**.env:**

```
COMPOSE_FILE=docker-compose.dev.yml
```

**Now simple command works:**

```bash
docker compose up  # Uses file from .env
```

**For production, change .env:**

```
COMPOSE_FILE=docker-compose.yml
```

### Long-term Fix 5: Document Clearly

**README.md:**

````markdown
## Running the Application

### Development (with hot-reload)

```bash
sudo docker compose -f docker-compose.dev.yml up
```
````

Changes to code will automatically rebuild and restart services.

### Production

```bash
sudo docker compose up -d
```

Uses optimized builds without hot-reload.

### Project Structure

```
frontend/          ← React application
  ├── src/
  ├── public/
  └── package.json

backend/           ← Rust API
  ├── src/
  └── Cargo.toml
```

Note: Previously `frontend/static`, now `frontend/` (restructured Nov 2025)

````

## Verification

### Test 1: Development Config

```bash
$ sudo docker compose -f docker-compose.dev.yml up
[+] Running 3/3
 ✔ Container sqlserver             Started
 ✔ Container rust-backend-dev      Started
 ✔ Container react-frontend-dev    Started

rust-backend-dev-1  | 🚀 Server running on http://0.0.0.0:3000
react-frontend-dev-1| VITE v5.0.0  ready in 423 ms
````

✅ All containers start  
✅ No connection errors  
✅ Services communicate

### Test 2: Production Config (Updated)

```bash
$ sudo docker compose build
[+] Building 245.2s (42/42) FINISHED

$ sudo docker compose up -d
[+] Running 3/3
 ✔ Container rust-backend      Started
 ✔ Container react-frontend    Started
 ✔ Container sqlserver         Started
```

✅ Builds from correct paths  
✅ Containers start  
✅ No errors

### Test 3: Verify Paths

**Check docker-compose.yml:**

```bash
$ grep "context:" docker-compose.yml
      context: ./frontend  ✅ Correct
      context: ./backend   ✅ Correct
```

**Check docker-compose.dev.yml:**

```bash
$ grep "context:" docker-compose.dev.yml
      context: ./frontend  ✅ Correct
      context: ./backend   ✅ Correct
```

✅ Both files have matching, correct paths

### Test 4: Frontend Files Location

```bash
$ ls -la frontend/
total 472
drwxr-xr-x  8 user user   4096 Nov 20 21:30 .
drwxr-xr-x 10 user user   4096 Nov 20 21:25 ..
-rw-r--r--  1 user user    523 Nov 20 20:15 Dockerfile
-rw-r--r--  1 user user    312 Nov 20 20:45 Dockerfile.dev
-rw-r--r--  1 user user    427 Nov 20 19:12 index.html
drwxr-xr-x  2 user user   4096 Nov 20 19:12 public
-rw-r--r--  1 user user 451234 Nov 20 19:12 package-lock.json
-rw-r--r--  1 user user    823 Nov 20 19:12 package.json
drwxr-xr-x  4 user user   4096 Nov 20 19:25 src
-rw-r--r--  1 user user    215 Nov 20 19:12 vite.config.js
```

✅ Files in correct location  
✅ No `static/` subdirectory

## Alternative Solutions

### Option A: Use docker-compose.override.yml

Instead of separate dev file, use override:

**docker-compose.yml (base):**

```yaml
services:
  rust-backend:
    build: ./backend
    ports:
      - "3000:3000"
```

**docker-compose.override.yml (local dev only):**

```yaml
services:
  rust-backend:
    build:
      dockerfile: Dockerfile.dev
    volumes:
      - ./backend:/app
```

**Auto-merge:**

```bash
docker compose up  # Automatically uses both
```

**Pros:**

- Simpler commands
- Auto-loaded for dev
- Production ignores override

**Cons:**

- Less explicit
- Can be confusing
- Override might get committed

### Option B: Use Branches

Different branch for different configs:

```bash
# Production branch
git checkout production
docker compose up

# Development branch
git checkout main
docker compose up
```

**Pros:**

- Clear separation
- Git history tracks changes
- Easy to switch

**Cons:**

- Extra git complexity
- Merge conflicts
- Not recommended for this use case

### Option C: Single Config with Profiles

**docker-compose.yml:**

```yaml
services:
  rust-backend-prod:
    profiles: ["production"]
    build:
      dockerfile: Dockerfile
    # Production settings

  rust-backend-dev:
    profiles: ["development"]
    build:
      dockerfile: Dockerfile.dev
    volumes:
      - ./backend:/app
    # Development settings
```

**Usage:**

```bash
docker compose --profile development up     # Dev
docker compose --profile production up      # Prod
```

**Pros:**

- Single file
- Clear separation
- Explicit profiles

**Cons:**

- More complex config
- Duplication of common settings

## Understanding Docker Compose File Loading

### Default File Resolution

**When you run `docker compose up`:**

1. Look in current directory for:

   - `compose.yaml`
   - `compose.yml`
   - `docker-compose.yaml`
   - `docker-compose.yml`

2. Use first one found

3. Also check for:

   - `compose.override.yaml`
   - `compose.override.yml`
   - `docker-compose.override.yaml`
   - `docker-compose.override.yml`

4. If override exists, merge with base

### Explicit File Selection

**-f flag:**

```bash
docker compose -f docker-compose.dev.yml up
```

Only uses specified file, ignores defaults.

**Multiple files:**

```bash
docker compose -f docker-compose.yml -f docker-compose.override.yml up
```

Merges files in order (later files override earlier).

### Environment Variable

**COMPOSE_FILE:**

```bash
export COMPOSE_FILE=docker-compose.dev.yml
docker compose up  # Uses dev file
```

Or in `.env`:

```
COMPOSE_FILE=docker-compose.dev.yml
```

### Project Name

**COMPOSE_PROJECT_NAME:**

```bash
export COMPOSE_PROJECT_NAME=myapp
docker compose up
```

Prefixes all container names with `myapp_`.

## Prevention Strategies

### 1. Keep Configs in Sync

When changing paths, update **all** compose files:

**Checklist:**

- [ ] docker-compose.yml
- [ ] docker-compose.dev.yml
- [ ] docker-compose.test.yml (if exists)
- [ ] Any override files

### 2. Use Symlinks (Carefully)

```bash
ln -s docker-compose.dev.yml docker-compose.yml
```

Now default command uses dev config.

**Warning:** Easy to accidentally commit symlink!

### 3. Git Hooks

**pre-commit hook:**

```bash
#!/bin/bash
# Check that all compose files have matching paths

frontend_prod=$(grep "context:.*frontend" docker-compose.yml | awk '{print $2}')
frontend_dev=$(grep "context:.*frontend" docker-compose.dev.yml | awk '{print $2}')

if [ "$frontend_prod" != "$frontend_dev" ]; then
  echo "ERROR: Compose files have different frontend paths!"
  echo "  Production: $frontend_prod"
  echo "  Development: $frontend_dev"
  exit 1
fi
```

### 4. Validation Script

**validate-compose.sh:**

```bash
#!/bin/bash

echo "Validating compose files..."

# Check syntax
docker compose -f docker-compose.yml config > /dev/null || exit 1
docker compose -f docker-compose.dev.yml config > /dev/null || exit 1

# Check paths exist
for path in $(grep "context:" docker-compose.yml | awk '{print $2}'); do
  if [ ! -d "$path" ]; then
    echo "ERROR: Path $path doesn't exist!"
    exit 1
  fi
done

echo "✅ All compose files valid"
```

### 5. Documentation in Files

Add comments to compose files:

```yaml
# docker-compose.yml
# Production configuration
# Updated: 2025-11-20 after restructure
# Note: Use docker-compose.dev.yml for development

version: "3.8"

services:
  react-frontend:
    build: ./frontend # Restructured from ./frontend/static on 2025-11-20
```

## Lessons Learned

1. **Update all configs when restructuring** - Don't leave some files with old paths
2. **Use helper scripts or Makefile** - Prevent using wrong commands
3. **Document which compose file to use** - Clear instructions in README
4. **Validate after restructuring** - Test both dev and prod configs
5. **Be consistent** - Keep all compose files in sync

## Timeline

- **Restructure Performed:** November 20, 2025
- **docker-compose.dev.yml updated:** Immediately
- **docker-compose.yml updated:** ❌ Forgot!
- **Issue Discovered:** ~1 hour later (used wrong compose file)
- **Diagnosis Time:** ~10 minutes (checked which file was being used)
- **Fix Time:** ~2 minutes (updated docker-compose.yml paths)
- **Documentation:** ~15 minutes (added helper scripts, Makefile)
- **Total Time:** ~27 minutes (excluding initial confusion)
- **Status:** ✅ Fully Resolved

## Related Commands

**Check which compose file is loaded:**

```bash
docker compose config --files
```

**Show merged configuration:**

```bash
docker compose config
```

**Validate compose file:**

```bash
docker compose -f docker-compose.yml config --quiet
echo $?  # 0 = valid
```

**List services in compose file:**

```bash
docker compose config --services
```

**Show specific service config:**

```bash
docker compose config --services rust-backend
```

**Force rebuild (ignore cache):**

```bash
docker compose build --no-cache
```

**Remove everything and start fresh:**

```bash
docker compose down -v
docker compose build --no-cache
docker compose up
```
