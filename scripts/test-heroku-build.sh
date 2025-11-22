#!/bin/bash

# Quick deployment test script
# Tests local Docker build before deploying to Heroku

set -e

echo "🧪 Testing Heroku deployment locally..."
echo ""

cd "$(dirname "$0")/.."

# Build Docker image
echo "🐳 Building Docker image..."
docker build -f backend/Dockerfile.heroku -t zoo-backend-test backend/

# Run container
echo "🚀 Starting container..."
docker run -d \
    --name zoo-backend-test \
    -p 8080:8080 \
    -e PORT=8080 \
    -e DATABASE_HOST=${DATABASE_HOST:-localhost} \
    -e DATABASE_PORT=${DATABASE_PORT:-1433} \
    -e DATABASE_NAME=${DATABASE_NAME:-zoo_db} \
    -e DATABASE_USER=${DATABASE_USER:-SA} \
    -e DATABASE_PASSWORD=${DATABASE_PASSWORD:-Password123} \
    -e RUST_LOG=info \
    zoo-backend-test

echo ""
echo "⏳ Waiting for server to start..."
sleep 5

# Test endpoint
echo "🔍 Testing /message endpoint..."
if curl -f http://localhost:8080/message; then
    echo ""
    echo "✅ Server is responding correctly!"
else
    echo ""
    echo "❌ Server test failed"
    docker logs zoo-backend-test
fi

echo ""
echo "🧹 Cleaning up..."
docker stop zoo-backend-test
docker rm zoo-backend-test

echo ""
echo "✅ Local test complete!"
echo "   You can now deploy to Heroku with: ./scripts/deploy-heroku.sh"
echo ""
