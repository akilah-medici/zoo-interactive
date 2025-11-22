# Heroku Deployment Guide

This guide covers deploying the Zoo Management System to Heroku using Docker containers.

## Overview

The application consists of:

- **Backend**: Rust/Axum API (deployed as Docker container)
- **Frontend**: React/Vite SPA (deployed separately or served by backend)
- **Database**: SQL Server (requires external provider)

---

## Prerequisites

1. **Heroku Account**: Sign up at https://heroku.com
2. **Heroku CLI**: Install from https://devcenter.heroku.com/articles/heroku-cli
3. **Git**: Ensure your code is in a Git repository
4. **Docker**: For local testing (optional)

---

## Option 1: Deploy Backend to Heroku (Recommended)

### Step 1: Login to Heroku

```bash
heroku login
```

### Step 2: Create Heroku App

```bash
# Create app for backend
heroku create zoo-backend-api

# Set stack to container
heroku stack:set container -a zoo-backend-api
```

### Step 3: Configure Database

Since Heroku doesn't natively support SQL Server, you have two options:

**Option A: Use Azure SQL Database**

1. Create Azure SQL Database: https://portal.azure.com
2. Note connection details
3. Set environment variables:

```bash
heroku config:set DATABASE_HOST=your-server.database.windows.net -a zoo-backend-api
heroku config:set DATABASE_PORT=1433 -a zoo-backend-api
heroku config:set DATABASE_NAME=zoo_db -a zoo-backend-api
heroku config:set DATABASE_USER=your-username -a zoo-backend-api
heroku config:set DATABASE_PASSWORD=your-password -a zoo-backend-api
```

**Option B: Use JawsDB MySQL (Requires Code Changes)**

```bash
heroku addons:create jawsdb:kitefin -a zoo-backend-api
```

Note: This requires converting SQL Server queries to MySQL syntax.

### Step 4: Update Backend for Heroku

Update `backend/src/main.rs` to use Heroku's PORT:

```rust
// Get port from environment or default to 3000
let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
let addr = format!("0.0.0.0:{}", port);

println!("Binding to {}...", addr);
let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
```

### Step 5: Deploy Backend

```bash
# Add Heroku remote
heroku git:remote -a zoo-backend-api

# Deploy using Docker
git push heroku main
```

### Step 6: Verify Backend Deployment

```bash
# Check logs
heroku logs --tail -a zoo-backend-api

# Test endpoint
curl https://zoo-backend-api.herokuapp.com/message
```

---

## Option 2: Deploy Frontend to Heroku

### Step 1: Create Frontend App

```bash
heroku create zoo-frontend-app
```

### Step 2: Add Node.js Buildpack

```bash
heroku buildpacks:set heroku/nodejs -a zoo-frontend-app
```

### Step 3: Configure Frontend Build

Update `frontend/package.json`:

```json
{
  "scripts": {
    "dev": "vite --host 0.0.0.0",
    "build": "vite build",
    "preview": "vite preview",
    "start": "vite preview --port $PORT --host 0.0.0.0"
  },
  "engines": {
    "node": "18.x",
    "npm": "9.x"
  }
}
```

### Step 4: Update API URL

Update frontend to use backend URL:

**Create `frontend/.env.production`**:

```env
VITE_API_URL=https://zoo-backend-api.herokuapp.com
```

**Update API calls** to use:

```javascript
const API_URL = import.meta.env.VITE_API_URL || "http://localhost:3000";

// Use in fetch calls
fetch(`${API_URL}/animals/list`);
```

### Step 5: Deploy Frontend

```bash
cd frontend
git init
heroku git:remote -a zoo-frontend-app
git add .
git commit -m "Initial frontend deployment"
git push heroku main
```

---

## Option 3: Serve Frontend from Backend (Single Dyno)

This is more cost-effective as it uses only one Heroku dyno.

### Step 1: Build Frontend

```bash
cd frontend
npm run build
```

This creates `frontend/dist/` with static files.

### Step 2: Update Backend to Serve Static Files

Add to `backend/Cargo.toml`:

```toml
[dependencies]
tower-http = { version = "0.5", features = ["fs", "cors"] }
```

Update `backend/src/main.rs`:

```rust
use tower_http::services::ServeDir;

// After building API routes
let app = Router::new()
    .route("/message", get(initial_page))
    .nest("/animals", animals_router)
    .nest("/cares", cares_router)
    .nest("/animal-cares", animal_cares_router)
    .nest_service("/", ServeDir::new("../frontend/dist"))
    .with_state(database)
    .layer(cors);
```

### Step 3: Update Dockerfile

Update `backend/Dockerfile.heroku`:

```dockerfile
# Add after COPY backend binary
COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist
```

### Step 4: Deploy Combined App

```bash
git add .
git commit -m "Serve frontend from backend"
git push heroku main
```

Now both frontend and backend are on: https://zoo-backend-api.herokuapp.com

---

## Database Setup on Azure

### Step 1: Create Azure SQL Database

1. Go to https://portal.azure.com
2. Create new SQL Database
3. Choose pricing tier (Basic is sufficient for testing)
4. Note server name, database name, username, password

### Step 2: Configure Firewall

1. In Azure portal, go to SQL Server
2. Navigate to "Firewalls and virtual networks"
3. Add client IP address
4. Enable "Allow Azure services"

### Step 3: Run Database Scripts

```bash
# Connect using sqlcmd or Azure Data Studio
sqlcmd -S your-server.database.windows.net -d zoo_db -U your-username -P your-password

# Run schema creation
:r sql/create-database.sql

# Run initial data
:r sql/initial-population-data.sql
```

---

## Environment Variables Reference

Set all required environment variables on Heroku:

```bash
heroku config:set RUST_LOG=info -a zoo-backend-api
heroku config:set ENVIRONMENT=production -a zoo-backend-api
heroku config:set DATABASE_HOST=your-server.database.windows.net -a zoo-backend-api
heroku config:set DATABASE_PORT=1433 -a zoo-backend-api
heroku config:set DATABASE_NAME=zoo_db -a zoo-backend-api
heroku config:set DATABASE_USER=your-username -a zoo-backend-api
heroku config:set DATABASE_PASSWORD=your-password -a zoo-backend-api
```

View current config:

```bash
heroku config -a zoo-backend-api
```

---

## Monitoring and Logs

### View Logs

```bash
# Real-time logs
heroku logs --tail -a zoo-backend-api

# Last 1000 lines
heroku logs -n 1000 -a zoo-backend-api

# Filter by source
heroku logs --source app -a zoo-backend-api
```

### Scale Dynos

```bash
# Scale up
heroku ps:scale web=2 -a zoo-backend-api

# Scale down
heroku ps:scale web=1 -a zoo-backend-api
```

### Check App Status

```bash
heroku ps -a zoo-backend-api
```

---

## Troubleshooting

### Issue: R10 Boot Timeout

**Problem**: App doesn't start within 60 seconds

**Solutions**:

1. Optimize build time
2. Check database connectivity
3. Review startup logs

```bash
heroku logs --tail -a zoo-backend-api
```

### Issue: Database Connection Failed

**Check**:

1. Environment variables are set correctly
2. Azure SQL firewall allows Heroku IPs
3. Connection string format is correct

**Test connection**:

```bash
heroku run bash -a zoo-backend-api
# Inside dyno:
env | grep DATABASE
```

### Issue: Frontend Can't Reach Backend

**Solutions**:

1. Update CORS configuration to allow frontend domain
2. Check API URL in frontend `.env.production`
3. Ensure backend is running: `heroku ps -a zoo-backend-api`

---

## Cost Optimization

### Free Tier Options

- **Heroku Free Dyno**: 550-1000 free dyno hours/month (deprecated as of Nov 2022)
- **Heroku Eco Dyno**: $5/month (sleeps after 30min inactivity)
- **Azure SQL Basic**: ~$5/month

### Recommended Setup for Production

- **Backend**: Heroku Basic dyno ($7/month)
- **Database**: Azure SQL Basic ($5/month)
- **Frontend**:
  - Option 1: Serve from backend (included)
  - Option 2: Netlify/Vercel (free tier)

**Total**: ~$12/month

---

## Alternative: Deploy to Render.com

Render.com is a Heroku alternative with better free tier:

### Backend (Render)

1. Create account at https://render.com
2. New Web Service
3. Connect GitHub repository
4. Configure:
   - **Build Command**: `cd backend && cargo build --release`
   - **Start Command**: `./backend/target/release/backend`
   - **Environment**: Add DATABASE\_\* variables

### Frontend (Render)

1. New Static Site
2. Connect repository
3. Configure:
   - **Build Command**: `cd frontend && npm install && npm run build`
   - **Publish Directory**: `frontend/dist`

---

## CI/CD with GitHub Actions

Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to Heroku

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Deploy to Heroku
        uses: akhileshns/heroku-deploy@v3.12.14
        with:
          heroku_api_key: ${{secrets.HEROKU_API_KEY}}
          heroku_app_name: "zoo-backend-api"
          heroku_email: ${{secrets.HEROKU_EMAIL}}
          usedocker: true
```

---

## Post-Deployment Checklist

- [ ] Backend accessible at Heroku URL
- [ ] Database connection working
- [ ] All environment variables set
- [ ] Frontend can call backend API
- [ ] CORS configured correctly
- [ ] Logs show no errors
- [ ] Test all CRUD operations
- [ ] Monitoring/alerting set up

---

## Support and Resources

- **Heroku Docs**: https://devcenter.heroku.com/
- **Heroku Container Registry**: https://devcenter.heroku.com/articles/container-registry-and-runtime
- **Azure SQL**: https://azure.microsoft.com/services/sql-database/
- **Render.com**: https://render.com/docs

---

**Last Updated**: November 22, 2025  
**Status**: Ready for deployment
