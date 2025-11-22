# Docker Quick Reference Guide

## Common Commands

### Starting the Development Environment

```bash
# Start all services in detached mode (recommended)
sudo docker compose -f "docker-compose.dev.yml" up -d

# Start all services and view logs
sudo docker compose -f "docker-compose.dev.yml" up

# Start specific service
sudo docker compose -f "docker-compose.dev.yml" up rust-backend
```

### Stopping Services

```bash
# Stop all services (keeps containers)
sudo docker compose -f "docker-compose.dev.yml" stop

# Stop and remove all containers
sudo docker compose -f "docker-compose.dev.yml" down

# Stop and remove containers + volumes (CAUTION: deletes database data)
sudo docker compose -f "docker-compose.dev.yml" down -v
```

### Rebuilding Services

```bash
# Rebuild specific service (use after code changes in Dockerfile)
sudo docker compose -f "docker-compose.dev.yml" build rust-backend

# Rebuild without cache (use if having issues)
sudo docker compose -f "docker-compose.dev.yml" build --no-cache rust-backend

# Rebuild and restart
sudo docker compose -f "docker-compose.dev.yml" up -d --build rust-backend
```

### Viewing Logs

```bash
# View all logs
sudo docker compose -f "docker-compose.dev.yml" logs

# View logs for specific service
sudo docker compose -f "docker-compose.dev.yml" logs rust-backend

# Follow logs in real-time
sudo docker compose -f "docker-compose.dev.yml" logs -f

# View last 50 lines
sudo docker compose -f "docker-compose.dev.yml" logs --tail=50
```

### Service Status

```bash
# Check running containers
sudo docker ps

# Check all containers (including stopped)
sudo docker ps -a

# Check service health
sudo docker compose -f "docker-compose.dev.yml" ps
```

### Executing Commands Inside Containers

```bash
# Open bash shell in Rust backend
sudo docker exec -it rust-backend-dev bash

# Open SQL Server interactive shell
sudo docker exec -it sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P 'Password123' -C

# Run a one-off SQL query
sudo docker exec sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P 'Password123' -Q "USE zoo_db; SELECT * FROM Animal;" -C
```

## Troubleshooting

### Rust Backend Not Starting

```bash
# Check logs
sudo docker logs rust-backend-dev

# Rebuild the service
sudo docker compose -f "docker-compose.dev.yml" build --no-cache rust-backend
sudo docker compose -f "docker-compose.dev.yml" up -d rust-backend
```

### Database Connection Issues

```bash
# Check if SQL Server is healthy
sudo docker inspect sqlserver | grep -A 5 Health

# Restart SQL Server
sudo docker restart sqlserver

# Wait for SQL Server to be ready
sudo docker exec sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P 'Password123' -Q "SELECT 1" -C
```

### Frontend Issues

```bash
# Rebuild frontend
sudo docker compose -f "docker-compose.dev.yml" build react-frontend
sudo docker compose -f "docker-compose.dev.yml" up -d react-frontend

# Check if node_modules are installed
sudo docker exec react-frontend-dev ls -la /app/node_modules
```

### Complete Reset (Nuclear Option)

```bash
# Stop everything
sudo docker compose -f "docker-compose.dev.yml" down

# Remove containers
sudo docker rm sqlserver rust-backend-dev react-frontend-dev

# Remove images
sudo docker rmi desafio-ciee-rust-backend desafio-ciee-react-frontend

# Rebuild everything
sudo docker compose -f "docker-compose.dev.yml" build --no-cache

# Start fresh
sudo docker compose -f "docker-compose.dev.yml" up -d
```

## SQL Server Management

### Running SQL Scripts

```bash
# Copy script into container
sudo docker cp /home/nijiuu/Documentos/Desafio-CIEE/sql/insert-data.sql sqlserver:/tmp/insert-data.sql

# Execute script
sudo docker exec sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P 'Password123' -i /tmp/insert-data.sql -C
```

### Common SQL Operations

```bash
# List all tables
sudo docker exec sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P 'Password123' -Q "USE zoo_db; SELECT name FROM sys.tables;" -C

# Select all animals
sudo docker exec sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P 'Password123' -Q "USE zoo_db; SELECT * FROM Animal;" -C

# Drop a table
sudo docker exec sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P 'Password123' -Q "USE zoo_db; DROP TABLE <table_name>;" -C
```

## Development Workflow

### Normal Development (No Docker Changes)

1. Make changes to your Rust/React code
2. **Rust**: Changes are auto-detected by `cargo watch` - no restart needed
3. **React**: Changes are auto-detected by Vite - no restart needed

### After Changing Dockerfile

```bash
# Rebuild the affected service
sudo docker compose -f "docker-compose.dev.yml" build <service-name>

# Restart the service
sudo docker compose -f "docker-compose.dev.yml" up -d <service-name>
```

### After Changing docker-compose.dev.yml

```bash
# Restart all services
sudo docker compose -f "docker-compose.dev.yml" down
sudo docker compose -f "docker-compose.dev.yml" up -d
```

## Service URLs

- **Frontend**: http://localhost:5173
- **Backend API**: http://localhost:3000
- **SQL Server**: localhost:1433

## Important Notes

1. **Always wait for SQL Server to be healthy** before the Rust backend starts (this is now automatic with health checks)

2. **Database data is persistent** in the `sqlserver-data` volume - it survives container restarts

3. **Code changes are live-reloaded** for both frontend and backend in development mode

4. **Don't modify files in `target/` or `node_modules/`** inside containers - they are managed automatically

5. **Use `-C` flag with sqlcmd** to skip SSL certificate verification in development
