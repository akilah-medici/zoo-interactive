#!/bin/bash

# Script to run SQL files in Docker SQL Server container
# Usage: ./sql/run-sql.sh <sql-file.sql>

if [ -z "$1" ]; then
    echo "Usage: ./sql/run-sql.sh <sql-file.sql>"
    exit 1
fi

SQL_FILE="$1"
CONTAINER_NAME="sqlserver"
SA_PASSWORD="Password123"

# Check if SQL file exists
if [ ! -f "$SQL_FILE" ]; then
    echo "Error: SQL file $SQL_FILE not found"
    exit 1
fi

echo "Running SQL script: $SQL_FILE"

# Copy SQL file to container
sudo docker cp "$SQL_FILE" $CONTAINER_NAME:/tmp/script.sql

# Execute SQL script using sqlcmd
sudo docker exec -it $CONTAINER_NAME /opt/mssql-tools18/bin/sqlcmd \
    -S localhost \
    -U SA \
    -P "$SA_PASSWORD" \
    -C \
    -i /tmp/script.sql

echo "SQL script executed successfully!"
