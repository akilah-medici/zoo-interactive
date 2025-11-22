# Heroku Deployment Checklist

Use this checklist to ensure a smooth deployment to Heroku.

## Pre-Deployment

### Code Preparation

- [ ] All code committed to Git
- [ ] Latest changes pushed to `main` branch
- [ ] Tests passing locally
- [ ] Backend builds successfully with `cargo build --release`
- [ ] Frontend builds successfully with `npm run build`

### Environment Setup

- [ ] Heroku account created
- [ ] Heroku CLI installed and working
- [ ] Logged in to Heroku CLI (`heroku auth:whoami`)
- [ ] Git repository initialized and pushed to GitHub (optional, for CI/CD)

### Database Preparation

- [ ] Database provider selected (Azure SQL, AWS RDS, etc.)
- [ ] Database created and accessible
- [ ] Database tables created (run `sql/create-database.sql`)
- [ ] Sample data populated (run `sql/initial-population-data.sql`)
- [ ] Database firewall configured to allow Heroku IPs
- [ ] Connection string tested

### Configuration Files

- [ ] `heroku.yml` present in project root
- [ ] `Procfile` present in project root (if not using container)
- [ ] `backend/Dockerfile.heroku` configured
- [ ] `backend/.env.example` filled with template values
- [ ] `frontend/.env.production` configured with production API URL
- [ ] `app.json` configured (optional, for Heroku button)

## Deployment Steps

### Backend Deployment

- [ ] Create Heroku app: `heroku create your-app-name`
- [ ] Set stack to container: `heroku stack:set container -a your-app-name`
- [ ] Configure environment variables:
  - [ ] `DATABASE_HOST`
  - [ ] `DATABASE_PORT`
  - [ ] `DATABASE_NAME`
  - [ ] `DATABASE_USER`
  - [ ] `DATABASE_PASSWORD`
  - [ ] `RUST_LOG`
  - [ ] `ENVIRONMENT`
- [ ] Add Heroku remote: `heroku git:remote -a your-app-name`
- [ ] Deploy: `git push heroku main`
- [ ] Wait for build to complete (watch logs)

### Verification

- [ ] Check dyno status: `heroku ps -a your-app-name`
- [ ] View logs: `heroku logs --tail -a your-app-name`
- [ ] Test health endpoint: `curl https://your-app-name.herokuapp.com/message`
- [ ] Test animals endpoint: `curl https://your-app-name.herokuapp.com/animals/list`
- [ ] Verify database connectivity (should see animals in response)
- [ ] Check for errors in logs

### Frontend Deployment (if deploying separately)

- [ ] Update `frontend/.env.production` with backend URL
- [ ] Build frontend: `cd frontend && npm run build`
- [ ] Create frontend Heroku app or use Netlify/Vercel
- [ ] Deploy frontend
- [ ] Test that frontend can reach backend API

### Post-Deployment

- [ ] Test all CRUD operations through web interface
- [ ] Verify search functionality
- [ ] Test modify workflow
- [ ] Test delete functionality
- [ ] Test care management
- [ ] Check browser console for errors
- [ ] Test on mobile devices (responsive design)

## Monitoring & Maintenance

### Immediate Post-Deploy

- [ ] Monitor logs for first 10 minutes: `heroku logs --tail -a your-app-name`
- [ ] Check for any crashes or errors
- [ ] Verify metrics in Heroku dashboard
- [ ] Set up alerts for dyno crashes

### Ongoing

- [ ] Configure logging add-on (e.g., Papertrail)
- [ ] Set up uptime monitoring (e.g., UptimeRobot, Pingdom)
- [ ] Configure application performance monitoring (APM)
- [ ] Set up database backups (if not automatic)
- [ ] Review and optimize dyno size based on usage

## Troubleshooting Checklist

If deployment fails, check:

### Build Issues

- [ ] Dockerfile syntax correct
- [ ] All dependencies in Cargo.toml
- [ ] Rust version compatible
- [ ] Build logs for specific errors: `heroku logs -a your-app-name`

### Runtime Issues

- [ ] PORT environment variable handled in code
- [ ] Database credentials correct
- [ ] Database firewall allows Heroku IPs
- [ ] CORS configured for frontend domain
- [ ] All required environment variables set

### Database Issues

- [ ] Connection string format correct
- [ ] Database server reachable from Heroku
- [ ] Tables exist in database
- [ ] User has correct permissions
- [ ] SSL/TLS configuration if required

### Performance Issues

- [ ] Dyno size appropriate for load
- [ ] Database plan adequate
- [ ] Connection pooling configured
- [ ] Queries optimized
- [ ] Indexes on frequently queried columns

## Rollback Plan

If something goes wrong:

1. **Quick Rollback**:

   ```bash
   heroku releases -a your-app-name
   heroku rollback v{previous-version} -a your-app-name
   ```

2. **Check Previous Version**:

   ```bash
   heroku releases:info v{version} -a your-app-name
   ```

3. **Restore Database** (if needed):
   - Restore from backup
   - Re-run migration scripts

## Security Checklist

- [ ] Environment variables not committed to Git
- [ ] .env files in .gitignore
- [ ] Database password strong and unique
- [ ] CORS restricted to frontend domain only (not permissive in production)
- [ ] HTTPS enforced (automatic on Heroku)
- [ ] No sensitive data in logs
- [ ] API rate limiting configured (if applicable)
- [ ] SQL injection protection verified (parameterized queries)

## Optimization Checklist

### Before Deploy

- [ ] Remove debug logs from production code
- [ ] Set `RUST_LOG=warn` or `info` (not `debug`)
- [ ] Enable release optimizations in Cargo.toml
- [ ] Minimize frontend bundle size
- [ ] Optimize database queries

### After Deploy

- [ ] Monitor response times
- [ ] Check database query performance
- [ ] Review dyno metrics
- [ ] Optimize based on actual usage patterns
- [ ] Consider caching strategy if needed

## Documentation Updates

After successful deployment:

- [ ] Update README with production URL
- [ ] Document any deployment-specific configuration
- [ ] Update API documentation with production endpoint
- [ ] Create runbook for common operations
- [ ] Document rollback procedure
- [ ] Share deployment credentials with team (securely)

## Cost Management

- [ ] Verify dyno plan ($5-$7/month for Eco/Basic)
- [ ] Verify database plan (~$5/month for basic)
- [ ] Monitor usage to avoid overages
- [ ] Set up billing alerts
- [ ] Review Heroku bill monthly
- [ ] Consider free alternatives for testing (Render.com)

---

## Quick Commands Reference

```bash
# Deploy
git push heroku main

# View logs
heroku logs --tail -a your-app-name

# Check status
heroku ps -a your-app-name

# View config
heroku config -a your-app-name

# Set config
heroku config:set KEY=VALUE -a your-app-name

# Restart
heroku restart -a your-app-name

# Open app
heroku open -a your-app-name

# Run bash in dyno
heroku run bash -a your-app-name

# View releases
heroku releases -a your-app-name

# Rollback
heroku rollback -a your-app-name
```

---

**Deployment Date**: ******\_\_\_******  
**Deployed By**: ******\_\_\_******  
**App URL**: ******\_\_\_******  
**Status**: ⬜ Success ⬜ Failed ⬜ Partial  
**Notes**: **********************\_\_\_**********************
