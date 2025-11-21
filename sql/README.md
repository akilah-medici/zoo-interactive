# SQL Server Management Guide

## Setup (One-time)

1. **Install SQL Server tools in the container:**

```bash
./sql/setup-tools.sh
```

This installs `sqlcmd` and other SQL Server tools inside the Docker container.

## Daily Workflow

### Option 1: Run SQL Scripts

1. **Create or edit SQL scripts** in the `sql/` directory (e.g., `sql/my-script.sql`)

2. **Run the script:**

```bash
./sql/run-sql.sh sql/my-script.sql
```

Example:

```bash
./sql/run-sql.sh sql/01-create-database.sql
```

### Option 2: Interactive SQL Shell

**Connect to SQL Server interactively:**

```bash
./sql/connect.sh
```

Then you can type SQL commands directly:

```sql
USE zoo_db;
GO

SELECT * FROM animals;
GO

INSERT INTO animals (name, species) VALUES ('Tiger', 'Bengal Tiger');
GO

exit
```

### Option 3: Using GUI Tools (Recommended)

You can connect to the database using:

- **Azure Data Studio** (free, cross-platform)
- **DBeaver** (free, cross-platform)
- **SQL Server Management Studio** (Windows only)

**Connection details:**

- Server: `localhost,1433`
- Username: `SA`
- Password: `YourStrong@Passw0rd`
- Database: `zoo_db`

## Common Tasks

### View all tables

```bash
./sql/connect.sh
```

```sql
USE zoo_db;
GO
SELECT * FROM INFORMATION_SCHEMA.TABLES;
GO
```

### Backup database

```bash
docker exec sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P 'YourStrong@Passw0rd' -C -Q "BACKUP DATABASE zoo_db TO DISK = '/tmp/zoo_db.bak'"
docker cp sqlserver:/tmp/zoo_db.bak ./backups/zoo_db.bak
```

### Quick query

```bash
docker exec sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P 'YourStrong@Passw0rd' -C -Q "USE zoo_db; SELECT * FROM animals;"
```

## File Structure

```
sql/
├── 01-create-database.sql  # Initial database setup
├── setup-tools.sh          # Install SQL tools (run once)
├── run-sql.sh              # Execute SQL files
├── connect.sh              # Interactive SQL shell
└── README.md               # This file
```

## Tips

- **Edit SQL files** in your favorite editor (VS Code, vim, etc.)
- **Save changes** and run `./sql/run-sql.sh <file.sql>` to apply them
- **Use version control** - commit your SQL scripts to git
- **Number your scripts** (01-, 02-, etc.) to maintain order
- **Use transactions** for safety:
  ```sql
  BEGIN TRANSACTION;
  -- your changes
  COMMIT;
  -- or ROLLBACK; if something goes wrong
  ```
