# Docker Container Rebuild - Quick Reference Guide

**Created:** 21 de novembro de 2025  
**Purpose:** Quick commands for managing Docker containers during development

---

## When to Rebuild Containers

Rebuild your Docker containers whenever you:

- ✅ Add or remove npm packages (`package.json` changes)
- ✅ Modify `Dockerfile` or `Dockerfile.dev`
- ✅ Change environment variables in docker-compose files
- ✅ Update base images or system dependencies
- ✅ Experience persistent import/module errors

---

## Quick Commands

### Stop All Services

```bash
cd /home/nijiuu/Documentos/Desafio-CIEE
sudo docker compose -f docker-compose.dev.yml down
```

### Rebuild Specific Service

```bash
# Frontend only
sudo docker compose -f docker-compose.dev.yml build react-frontend

# Backend only
sudo docker compose -f docker-compose.dev.yml build rust-backend

# Database (rarely needed)
sudo docker compose -f docker-compose.dev.yml build sqlserver
```

### Rebuild All Services

```bash
sudo docker compose -f docker-compose.dev.yml build
```

### Force Rebuild (No Cache)

```bash
# Use when normal rebuild doesn't fix the issue
sudo docker compose -f docker-compose.dev.yml build --no-cache react-frontend
```

### Start Services

```bash
sudo docker compose -f docker-compose.dev.yml up -d
```

### Start with Logs (Foreground)

```bash
sudo docker compose -f docker-compose.dev.yml up
```

---

## Complete Rebuild Workflow

**Full Reset (Clean Slate):**

```bash
cd /home/nijiuu/Documentos/Desafio-CIEE

# Stop and remove containers, networks, volumes
sudo docker compose -f docker-compose.dev.yml down -v

# Rebuild all services from scratch
sudo docker compose -f docker-compose.dev.yml build --no-cache

# Start services
sudo docker compose -f docker-compose.dev.yml up -d
```

---

## Troubleshooting Commands

### View Logs

```bash
# All services
sudo docker compose -f docker-compose.dev.yml logs

# Specific service
sudo docker compose -f docker-compose.dev.yml logs react-frontend
sudo docker compose -f docker-compose.dev.yml logs rust-backend
sudo docker compose -f docker-compose.dev.yml logs sqlserver

# Follow logs in real-time
sudo docker compose -f docker-compose.dev.yml logs -f react-frontend
```

### Check Running Containers

```bash
sudo docker compose -f docker-compose.dev.yml ps
```

### Access Container Shell

```bash
# Frontend (Node.js)
sudo docker compose -f docker-compose.dev.yml exec react-frontend sh

# Backend (Rust)
sudo docker compose -f docker-compose.dev.yml exec rust-backend sh

# Database (SQL Server)
sudo docker compose -f docker-compose.dev.yml exec sqlserver bash
```

### Restart Specific Service

```bash
sudo docker compose -f docker-compose.dev.yml restart react-frontend
```

---

## Common Scenarios

### Added New npm Package

```bash
# 1. Add package locally to update package.json
cd /home/nijiuu/Documentos/Desafio-CIEE/frontend
npm install <package-name>

# 2. Rebuild frontend container
cd /home/nijiuu/Documentos/Desafio-CIEE
sudo docker compose -f docker-compose.dev.yml build react-frontend

# 3. Restart services
sudo docker compose -f docker-compose.dev.yml up -d
```

### Modified Rust Code

```bash
# No rebuild needed - cargo watch handles hot reload
# Just restart if needed
sudo docker compose -f docker-compose.dev.yml restart rust-backend
```

### Changed Frontend Code (JSX/JS/CSS)

```bash
# No rebuild needed - Vite handles hot reload automatically
# Just save the file and refresh browser
```

### Database Issues

```bash
# Reset database and restart
sudo docker compose -f docker-compose.dev.yml down -v
sudo docker compose -f docker-compose.dev.yml up -d
```

---

## Performance Tips

### Skip Unnecessary Services

```bash
# Start only what you need
sudo docker compose -f docker-compose.dev.yml up -d sqlserver rust-backend
```

### Clean Up Unused Images

```bash
# Free up disk space
sudo docker system prune -a

# Remove only stopped containers
sudo docker container prune
```

### Check Container Resources

```bash
sudo docker stats
```

---

## Service URLs

- **Frontend (Vite):** http://localhost:5173
- **Backend (Axum):** http://localhost:3000
- **Database (SQL Server):** localhost:1433
  - User: `sa`
  - Password: `YourStrong@Passw0rd`

---

## Health Checks

### Verify Services Are Running

```bash
# Check if containers are up
sudo docker compose -f docker-compose.dev.yml ps

# Test backend API
curl http://localhost:3000/animals

# Test frontend
curl http://localhost:5173
```

### Check Container Health

```bash
# SQL Server has health check
sudo docker inspect sqlserver | grep -A 10 Health
```

---

## Best Practices

1. **Always rebuild after package.json changes**

   - Docker containers don't see local npm installs
   - Rebuild ensures container has latest dependencies

2. **Use -d flag for background mode**

   - Keeps terminal free for other commands
   - Use `logs -f` to view output when needed

3. **Clean up regularly**

   - Remove old images and containers
   - Prevents disk space issues

4. **Check logs first**

   - Most issues have clear error messages in logs
   - Start here before rebuilding

5. **Use specific service names**
   - Faster than rebuilding everything
   - Only rebuild what changed

---

## Emergency Commands

### Nothing Works - Complete Reset

```bash
cd /home/nijiuu/Documentos/Desafio-CIEE
sudo docker compose -f docker-compose.dev.yml down -v
sudo docker system prune -a -f
sudo docker compose -f docker-compose.dev.yml build --no-cache
sudo docker compose -f docker-compose.dev.yml up -d
```

### Container Won't Stop

```bash
# Force remove
sudo docker rm -f react-frontend-dev rust-backend-dev sqlserver
```

### Port Already in Use

```bash
# Find process using port
sudo lsof -i :5173
sudo lsof -i :3000
sudo lsof -i :1433

# Kill process
sudo kill -9 <PID>
```

---

## Related Documentation

- [docker-startup-fix-log.md](./docker-startup-fix-log.md) - Docker Compose startup issues
- [docker-quick-reference.md](./docker-quick-reference.md) - General Docker commands
- [react-datepicker-docker-issue.md](./react-datepicker-docker-issue.md) - npm package Docker installation

---

**Last Updated:** 21 de novembro de 2025
