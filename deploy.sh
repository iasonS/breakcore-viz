#!/bin/bash
# Deployment script for breakcore-viz on server

set -e

echo "=== Breakcore Visualizer Deployment ==="
echo ""
echo "This script deploys the visualizer to your server."
echo "Usage: ./deploy.sh [server-user@server-host]"
echo ""

if [ -z "$1" ]; then
    echo "Default deployment to tt@192.168.1.9"
    SERVER="tt@192.168.1.9"
else
    SERVER="$1"
fi

DEPLOY_DIR="/home/tt/breakcore-viz"

echo "Deploying to: $SERVER:$DEPLOY_DIR"
echo ""

# Check if we can reach the server
if ! ssh "$SERVER" "echo 'Connection OK'" > /dev/null 2>&1; then
    echo "ERROR: Cannot connect to $SERVER"
    echo "Make sure you can SSH to the server first."
    exit 1
fi

echo "[1/4] Creating deployment directory..."
ssh "$SERVER" "mkdir -p $DEPLOY_DIR"

echo "[2/4] Uploading project files..."
rsync -avz --delete \
    --exclude='target' \
    --exclude='.git' \
    --exclude='node_modules' \
    --exclude='*.lock' \
    . "$SERVER:$DEPLOY_DIR/"

# Ensure web directory is synced (rsync sometimes skips empty dirs)
rsync -avz web/ "$SERVER:$DEPLOY_DIR/web/"

echo "[3/4] Building Docker image..."
ssh "$SERVER" "cd $DEPLOY_DIR && docker build --no-cache -t breakcore-viz:latest ."

echo "[4/4] Starting Docker container..."
ssh "$SERVER" "cd $DEPLOY_DIR && docker-compose down || true && docker-compose up -d"

echo ""
echo "=== Deployment Complete ==="
echo ""
echo "Access the visualizer at: http://192.168.1.9:8080"
echo ""
echo "To view logs:"
echo "  ssh $SERVER 'cd $DEPLOY_DIR && docker-compose logs -f'"
echo ""
echo "To stop:"
echo "  ssh $SERVER 'cd $DEPLOY_DIR && docker-compose down'"
