#!/bin/bash

# Heroku Deployment Script for Backend
# This script deploys the Rust backend to Heroku using Docker

set -e

echo "🚀 Deploying Zoo Management Backend to Heroku..."
echo ""

# Check if Heroku CLI is installed
if ! command -v heroku &> /dev/null; then
    echo "❌ Heroku CLI not found. Please install it from:"
    echo "   https://devcenter.heroku.com/articles/heroku-cli"
    exit 1
fi

# Check if logged in to Heroku
if ! heroku auth:whoami &> /dev/null; then
    echo "❌ Not logged in to Heroku. Running 'heroku login'..."
    heroku login
fi

# Get app name from user or use default
read -p "Enter Heroku app name (default: zoo-backend-api): " APP_NAME
APP_NAME=${APP_NAME:-zoo-backend-api}

echo ""
echo "📦 App name: $APP_NAME"
echo ""

# Check if app exists
if heroku apps:info -a $APP_NAME &> /dev/null; then
    echo "✅ App '$APP_NAME' already exists"
else
    echo "📝 Creating new Heroku app: $APP_NAME..."
    heroku create $APP_NAME
    
    # Set stack to container
    echo "🐳 Setting stack to container..."
    heroku stack:set container -a $APP_NAME
fi

# Configure environment variables
echo ""
echo "🔧 Configuring environment variables..."
read -p "Enter DATABASE_HOST: " DB_HOST
read -p "Enter DATABASE_NAME (default: zoo_db): " DB_NAME
DB_NAME=${DB_NAME:-zoo_db}
read -p "Enter DATABASE_USER: " DB_USER
read -s -p "Enter DATABASE_PASSWORD: " DB_PASSWORD
echo ""

heroku config:set \
    DATABASE_HOST=$DB_HOST \
    DATABASE_PORT=1433 \
    DATABASE_NAME=$DB_NAME \
    DATABASE_USER=$DB_USER \
    DATABASE_PASSWORD=$DB_PASSWORD \
    RUST_LOG=info \
    ENVIRONMENT=production \
    -a $APP_NAME

echo ""
echo "🔄 Adding Heroku remote..."
heroku git:remote -a $APP_NAME

echo ""
echo "🚢 Deploying to Heroku..."
git push heroku main

echo ""
echo "📊 Checking app status..."
heroku ps -a $APP_NAME

echo ""
echo "📝 Recent logs:"
heroku logs --tail -n 50 -a $APP_NAME

echo ""
echo "✅ Deployment complete!"
echo ""
echo "🌐 Your app is available at: https://$APP_NAME.herokuapp.com"
echo ""
echo "📚 Useful commands:"
echo "   heroku logs --tail -a $APP_NAME          # View logs"
echo "   heroku ps -a $APP_NAME                   # Check status"
echo "   heroku config -a $APP_NAME               # View config"
echo "   heroku open -a $APP_NAME                 # Open in browser"
echo ""
