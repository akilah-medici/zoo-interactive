# Heroku Deployment Quick Start

## Prerequisites

- [ ] Heroku account created
- [ ] Heroku CLI installed
- [ ] Git repository initialized
- [ ] Database (Azure SQL or similar) set up

## Quick Deploy (5 minutes)

### 1. Make scripts executable

```bash
chmod +x scripts/deploy-heroku.sh
chmod +x scripts/test-heroku-build.sh
```

### 2. Test build locally (optional)

```bash
./scripts/test-heroku-build.sh
```

### 3. Deploy to Heroku

```bash
./scripts/deploy-heroku.sh
```

Follow the prompts to enter:

- App name
- Database credentials

### 4. Verify deployment

```bash
# Check status
heroku ps -a your-app-name

# View logs
heroku logs --tail -a your-app-name

# Test endpoint
curl https://your-app-name.herokuapp.com/message
```

## Manual Deployment

If you prefer manual control:

```bash
# Login
heroku login

# Create app
heroku create zoo-backend-api
heroku stack:set container -a zoo-backend-api

# Set environment variables
heroku config:set \
    DATABASE_HOST=your-server.database.windows.net \
    DATABASE_NAME=zoo_db \
    DATABASE_USER=your-username \
    DATABASE_PASSWORD=your-password \
    -a zoo-backend-api

# Deploy
heroku git:remote -a zoo-backend-api
git push heroku main
```

## Troubleshooting

### Build fails

```bash
# Check logs
heroku logs --tail -a zoo-backend-api

# Try local build
./scripts/test-heroku-build.sh
```

### Database connection fails

```bash
# Verify config
heroku config -a zoo-backend-api

# Test from dyno
heroku run bash -a zoo-backend-api
env | grep DATABASE
```

### App crashes on startup

```bash
# Check logs
heroku logs --tail -a zoo-backend-api

# Restart
heroku restart -a zoo-backend-api
```

## Next Steps

1. **Set up frontend**: Follow `docs/HEROKU-DEPLOYMENT.md` section on frontend deployment
2. **Configure domain**: Add custom domain in Heroku dashboard
3. **Enable SSL**: Automatic with custom domain on paid plans
4. **Set up monitoring**: Use Heroku metrics or external service
5. **Configure CI/CD**: See `docs/HEROKU-DEPLOYMENT.md` GitHub Actions section

## Cost Estimate

- **Heroku Eco Dyno**: $5/month (backend)
- **Azure SQL Basic**: $5/month (database)
- **Total**: ~$10/month

For free alternatives, see `docs/HEROKU-DEPLOYMENT.md` Render.com section.

## Support

Full documentation: `docs/HEROKU-DEPLOYMENT.md`
