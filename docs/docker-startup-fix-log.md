# Docker Startup Fix - Detailed Log

**Date:** November 21, 2025  
**Issue:** Docker Compose failing to start, Rust backend exiting immediately

---

## Problems Identified

### 1. **Incorrect Container Command**

**What was wrong:**

- The Rust backend container was trying to execute `/app/backend` as a binary
- Error: `exec: "/app/backend": stat /app/backend: no such file or directory`
- This happened because of a stale Docker image with cached configuration

**Why it happened:**

- When you build Docker images, Docker caches layers and configurations
- If you change the Dockerfile but don't rebuild cleanly, old commands can persist
- The container had a leftover `CMD ["/app/backend"]` from a previous configuration
- The current `Dockerfile.dev` uses `cargo watch -x run` for development with hot reload

**How it was fixed:**

- Stopped all containers: `docker compose down`
- Rebuilt the image without cache: `docker compose build --no-cache rust-backend`
- This ensured the correct `CMD ["cargo", "watch", "-x", "run"]` from the Dockerfile was used

---

### 2. **Database Connection Timing Issue**

**What was wrong:**

- Rust backend started before SQL Server was ready to accept connections
- Error: `Failed to connect to database: Os { code: 111, kind: ConnectionRefused, message: "Connection refused" }`
- The backend would crash immediately on startup

**Why it happened:**

- Docker Compose `depends_on` only waits for the container to start, not for the service inside to be ready
- SQL Server takes 20-30 seconds to initialize its database engine after the container starts
- The Rust backend tried to connect immediately and failed

**How it was fixed:**

- Added a health check to the SQL Server service in `docker-compose.dev.yml`:
  ```yaml
  healthcheck:
    test:
      [
        "CMD-SHELL",
        "/opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P 'Password123' -Q 'SELECT 1' -C || exit 1",
      ]
    interval: 10s
    timeout: 5s
    retries: 5
    start_period: 30s
  ```
- Changed `depends_on` for rust-backend to wait for SQL Server health:
  ```yaml
  depends_on:
    sqlserver:
      condition: service_healthy
  ```
- Added `restart: on-failure` to automatically retry if the connection fails temporarily

**Technical details:**

- Health check runs every 10 seconds
- Starts checking after 30 seconds (start_period)
- Retries 5 times before marking as unhealthy
- Uses `sqlcmd` to verify SQL Server is actually accepting connections, not just that the container is running

---

### 3. **Missing React Dependencies**

**What was wrong:**

- React frontend couldn't find `react-router-dom`
- Warning: `Failed to run dependency scan. Skipping dependency pre-bundling.`
- Error: `react-router-dom (imported by /app/src/pages/Dashboard.jsx) Are they installed?`

**Why it happened:**

- When using bind mounts (mounting host directory into container), the `node_modules` from the host can conflict with the container
- The Dockerfile only copied `package.json` and ran `npm install`, but didn't copy the source code
- Without copying source code during build, the dependencies weren't properly linked

**How it was fixed:**

- Updated `frontend/Dockerfile.dev` to copy all source code after installing dependencies:

  ```dockerfile
  # Copy package files
  COPY package*.json ./

  # Install dependencies
  RUN npm install

  # Copy the rest of the application
  COPY . .
  ```

- The docker-compose already had a volume for `node_modules` to prevent host/container conflicts:
  ```yaml
  volumes:
    - type: bind
      source: ./frontend
      target: /app
    - type: volume
      target: /app/node_modules # Keeps node_modules isolated in container
  ```

---

## Summary of Changes

### Files Modified:

1. **docker-compose.dev.yml**

   - Added health check to `sqlserver` service
   - Changed `rust-backend` depends_on to use `condition: service_healthy`
   - Added `restart: on-failure` to rust-backend

2. **frontend/Dockerfile.dev**
   - Added `COPY . .` step to copy all source code into the image

### Commands Run:

```bash
# Stop all containers
sudo docker compose -f "docker-compose.dev.yml" down

# Remove stale containers
sudo docker stop sqlserver rust-backend-dev react-frontend-dev
sudo docker rm sqlserver rust-backend-dev react-frontend-dev

# Rebuild without cache
sudo docker compose -f "docker-compose.dev.yml" build --no-cache rust-backend
sudo docker compose -f "docker-compose.dev.yml" build react-frontend

# Start services
sudo docker compose -f "docker-compose.dev.yml" up
```

---

## Why You Had to Make Manual Changes Before

**Original problem:**

- When starting Docker Compose normally, the Rust backend would start and immediately exit
- Making a small change (adding a space/tab) and saving a Rust file would trigger `cargo watch` to rebuild
- By the time `cargo watch` rebuilt, SQL Server was already ready
- The timing worked out by accident, masking the real issue

**The real issue was:**

1. Stale Docker image with wrong command
2. No health check - backend tried to connect before database was ready
3. Race condition between services

**Why the fix works:**

- Health checks ensure SQL Server is actually ready before starting the backend
- Clean rebuild removes stale configurations
- Proper dependency ordering prevents race conditions
- No more need for manual file changes to trigger rebuilds

---

## Testing the Fix

After applying these fixes, you can verify everything works:

```bash
# Start all services
sudo docker compose -f "docker-compose.dev.yml" up

# Check logs - should see:
# - SQL Server: "SQL Server is now ready for client connections"
# - Rust backend: "Connected to SQL Server successfully"
# - React frontend: "ready in XXX ms"

# Test backend
curl http://localhost:3000/animals

# Test frontend
curl http://localhost:5173
```

---

## Future Improvements

To avoid this issue in the future:

1. **Always rebuild after Dockerfile changes:**

   ```bash
   sudo docker compose -f "docker-compose.dev.yml" build --no-cache <service>
   ```

2. **Use health checks for all services that have startup time:**

   - Databases (SQL Server, PostgreSQL, MySQL)
   - Message queues (RabbitMQ, Kafka)
   - Cache services (Redis)

3. **Set proper restart policies:**

   - `restart: on-failure` for services that might fail temporarily
   - `restart: unless-stopped` for services that should always run

4. **Monitor logs during startup:**
   ```bash
   sudo docker compose -f "docker-compose.dev.yml" up --watch
   ```

---

## Lessons Learned

1. **Docker caching is powerful but can cause issues** when you change build configurations without rebuilding cleanly

2. **Service dependencies are not just about start order** - they're about readiness. Use health checks.

3. **Bind mounts + package managers need special handling** - Always use volume overlays for `node_modules`, `target`, etc.

4. **Timing-based fixes are symptoms, not solutions** - If you need to manually trigger a rebuild to make something work, there's an underlying issue.

---

## Related Documentation

- [changelog-tiberius-sql-server.md](./changelog-tiberius-sql-server.md) - SQL Server integration
- [changelog-frontend-integration.md](./changelog-frontend-integration.md) - Frontend API integration
