#!/bin/bash
/opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P "Password123" -C -i /sql/create-database.sql
/opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P "Password123" -C -i /sql/initial-population-data.sql
# Add more scripts as needed
exec /opt/mssql/bin/sqlservr