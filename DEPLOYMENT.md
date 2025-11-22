# Deployment Configuration

This directory contains deployment configurations and scripts for various platforms.

## Files Overview

### Heroku Deployment

- `Procfile` - Process type declaration for Heroku
- `heroku.yml` - Heroku container build configuration
- `app.json` - Heroku app manifest
- `backend/Dockerfile.heroku` - Docker configuration for Heroku deployment
- `scripts/deploy-heroku.sh` - Automated deployment script
- `scripts/test-heroku-build.sh` - Local build testing script

### Environment Files

- `backend/.env.example` - Backend environment variables template
- `frontend/.env.production` - Frontend production configuration
- `frontend/.env.development` - Frontend development configuration
- `frontend/.env.example` - Frontend environment template

### CI/CD

- `.github/workflows/deploy.yml` - GitHub Actions workflow for automated deployment

## Quick Start

1. **Set up environment**:

   ```bash
   cp backend/.env.example backend/.env
   cp frontend/.env.example frontend/.env.local
   # Edit files with your values
   ```

2. **Test locally**:

   ```bash
   ./scripts/test-heroku-build.sh
   ```

3. **Deploy to Heroku**:
   ```bash
   ./scripts/deploy-heroku.sh
   ```

For detailed instructions, see `HEROKU-QUICKSTART.md` and `docs/HEROKU-DEPLOYMENT.md`.

## GitHub Secrets Required (for CI/CD)

If using GitHub Actions for automatic deployment, add these secrets to your repository:

- `HEROKU_API_KEY` - Your Heroku API key (from Account Settings)
- `HEROKU_APP_NAME` - Your Heroku app name
- `HEROKU_EMAIL` - Your Heroku account email

## Alternative Deployment Platforms

While this setup is optimized for Heroku, you can also deploy to:

- **Render.com**: See `docs/HEROKU-DEPLOYMENT.md` for instructions
- **Railway.app**: Similar to Heroku, uses Dockerfile
- **Fly.io**: Container-based deployment
- **Azure Container Apps**: Direct Azure integration
- **AWS ECS/Fargate**: Enterprise-grade container hosting

## Support

For issues or questions, refer to:

- `HEROKU-QUICKSTART.md` - Quick deployment guide
- `docs/HEROKU-DEPLOYMENT.md` - Comprehensive deployment documentation
- `docs/implementation-summary.md` - Full system documentation
