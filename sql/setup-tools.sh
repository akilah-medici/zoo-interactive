#!/bin/bash

# Script to install SQL Server tools in the container
# Run this once after starting the SQL Server container

CONTAINER_NAME="sqlserver"

echo "Installing SQL Server tools in container..."

sudo docker exec $CONTAINER_NAME bash -c "
    apt-get update && \
    apt-get install -y curl apt-transport-https gnupg && \
    curl https://packages.microsoft.com/keys/microsoft.asc | apt-key add - && \
    curl https://packages.microsoft.com/config/ubuntu/22.04/prod.list > /etc/apt/sources.list.d/mssql-release.list && \
    apt-get update && \
    ACCEPT_EULA=Y apt-get install -y msodbcsql18 mssql-tools18 && \
    echo 'Installation complete!'
"

echo ""
echo "SQL Server tools installed successfully!"
echo "You can now use ./sql/connect.sh or ./sql/run-sql.sh"
