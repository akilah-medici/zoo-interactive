# Project Documentation Index

## Quick Start

- **[Docker Quick Reference](./docker-quick-reference.md)** - Essential Docker commands for daily development
- **[Docker Startup Fix Summary](./docker-startup-fix-summary.md)** - Executive summary of the Docker startup issue resolution

## Major Changes & Migrations

- **[Tiberius SQL Server Migration](./changelog-tiberius-sql-server.md)** - Switch from SQLx/MySQL to Tiberius/SQL Server
- **[Frontend Integration](./changelog-frontend-integration.md)** - React frontend API integration
- **[Database Integration](./changelog-database-integration.md)** - Initial database setup
- **[MySQL Switch](./changelog-mysql-switch.md)** - Earlier MySQL migration (superseded by SQL Server)

## Detailed Issue Logs

### Infrastructure & Docker

- **[Docker Startup Fix](./docker-startup-fix-log.md)** - Detailed analysis of Docker startup issues and solutions
- **[Issue #04: Docker Binary Not Found](./issue-04-docker-binary-not-found.md)**
- **[Issue #05: Frontend Dockerfile Path](./issue-05-frontend-dockerfile-path.md)**
- **[Issue #09: Docker No Hot Reload](./issue-09-docker-no-hot-reload.md)**
- **[Issue #10: Wrong Compose File](./issue-10-wrong-compose-file.md)**

### Backend Issues

- **[Issue #03: Rust Edition 2024](./issue-03-rust-edition-2024.md)**
- **[Issue #07: Handler No Return](./issue-07-handler-no-return.md)**

### Frontend Issues

- **[Issue #06: CORS Blocking](./issue-06-cors-blocking.md)**
- **[Issue #08: Fetch URL Malformed](./issue-08-fetch-url-malformed.md)**

### Version Control

- **[Issue #01: Git Authentication](./issue-01-git-authentication.md)**
- **[Issue #02: Git Branch Merge](./issue-02-git-branch-merge.md)**

## Development Log

- **[DEVELOPMENT_LOG.md](./DEVELOPMENT_LOG.md)** - Comprehensive development timeline

## Document Purposes

### For Daily Use

- Use **docker-quick-reference.md** for common commands
- Check **docker-startup-fix-summary.md** if startup issues occur

### For Understanding Issues

- Read **docker-startup-fix-log.md** for deep technical understanding
- Review specific issue-XX files for historical context

### For Onboarding

- Start with **DEVELOPMENT_LOG.md** for project overview
- Review changelog files to understand architecture decisions

## Recent Updates (November 21, 2025)

### Fixed

- ✅ Docker startup race condition
- ✅ Rust backend immediate exit issue
- ✅ SQL Server health check implementation
- ✅ Frontend dependency resolution

### Added

- 📝 Comprehensive Docker documentation
- 📝 Detailed troubleshooting guides
- 📝 Quick reference for daily operations

## Project Status

**Environment:** Development  
**Stack:**

- Frontend: React + Vite (Port 5173)
- Backend: Rust + Axum (Port 3000)
- Database: SQL Server 2022 (Port 1433)

**Current State:** All services running successfully with auto-reload

## How to Navigate This Documentation

1. **New to the project?** Start with [DEVELOPMENT_LOG.md](./DEVELOPMENT_LOG.md)
2. **Need to use Docker?** Go to [docker-quick-reference.md](./docker-quick-reference.md)
3. **Having startup issues?** Check [docker-startup-fix-summary.md](./docker-startup-fix-summary.md)
4. **Want technical details?** Read the changelog and issue files
5. **Understanding architecture?** Review [changelog-tiberius-sql-server.md](./changelog-tiberius-sql-server.md) and [changelog-frontend-integration.md](./changelog-frontend-integration.md)
