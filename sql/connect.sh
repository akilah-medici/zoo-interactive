#!/bin/bash

# Script to connect to SQL Server interactively
# Usage: ./sql/connect.sh

CONTAINER_NAME="sqlserver"
SA_PASSWORD="Password123"

echo "Connecting to SQL Server..."
echo "Type SQL commands and press GO to execute them"
echo "Type 'exit' to quit"
echo ""

sudo docker exec -it $CONTAINER_NAME /opt/mssql-tools18/bin/sqlcmd \
    -S localhost \
    -U SA \
    -P "$SA_PASSWORD" \
    -C
