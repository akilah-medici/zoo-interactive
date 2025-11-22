# SQL Server Integration Fix - Tiberius Implementation

**Date:** November 20, 2025  
**Type:** Bug Fix / Architecture Change

---

## Problem Statement

The frontend was showing CORS errors when trying to fetch animals:

```
Cross-Origin Request Blocked: The Same Origin Policy disallows reading the remote resource at http://localhost:3000/animals.
(Reason: CORS request did not succeed). Status code: (null).
```

### Root Cause Analysis:

1. **Wrong Database Driver**: Code was using SQLx with MySQL features
2. **SQL Server Running**: Docker container was running SQL Server 2022
3. **Connection Mismatch**: Backend couldn't connect to SQL Server using MySQL driver
4. **Backend Crash**: Server wasn't starting, causing CORS errors

---

## Solution: Switched to Tiberius

SQLx 0.8.6 doesn't support the `mssql` feature. We switched to **Tiberius**, the native SQL Server driver for Rust.

---

## Changes Made

### 1. **Dependencies** (`backend/Cargo.toml`)

#### Before (Failed):

```toml
sqlx = { version = "0.8.6", features = ["runtime-tokio", "mysql", "chrono"] }
```

#### After (Working):

```toml
tiberius = { version = "0.12", features = ["chrono", "tds73"] }
tokio-util = { version = "0.7", features = ["compat"] }
```

**Why:**

- `tiberius`: Native SQL Server driver for Rust
- `tds73`: TDS (Tabular Data Stream) protocol version 7.3 for SQL Server
- `tokio-util`: Provides compatibility layer for async streams

---

### 2. **Environment Configuration** (`backend/.env`)

#### Before:

```env
DATABASE_URL=mysql://root:Password123@sqlserver:3306/zoo_db
```

#### After:

```env
DB_HOST=sqlserver
DB_PORT=1433
DB_USER=SA
DB_PASSWORD=Password123
DB_NAME=zoo_db
```

**Why:**

- Tiberius doesn't use connection strings
- Configuration is built programmatically
- Separate variables for better clarity

---

### 3. **Database Module** (`backend/src/db.rs`)

Complete rewrite from SQLx pool to Tiberius client:

```rust
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub struct Database {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
}

impl Database {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load config from environment variables
    }

    pub async fn connect(&self) -> Result<Client<...>, Box<dyn std::error::Error>> {
        let mut config = Config::new();
        config.host(&self.host);
        config.port(self.port);
        config.authentication(AuthMethod::sql_server(&self.user, &self.password));
        config.trust_cert();  // Accept self-signed certs (dev only)
        config.database(&self.database);

        let tcp = TcpStream::connect(format!("{}:{}", self.host, self.port)).await?;
        let client = Client::connect(config, tcp.compat_write()).await?;

        Ok(client)
    }
}
```

**Key Changes:**

- ❌ Removed: Connection pooling (Tiberius creates connections on-demand)
- ✅ Added: Direct TCP connection with TDS protocol
- ✅ Added: `trust_cert()` for self-signed certificates (development)
- ✅ Changed: `new()` is now synchronous (only config validation)
- ✅ Changed: `connect()` creates new connection each time

---

### 4. **Animal Model** (`backend/src/models/animal.rs`)

#### Removed SQLx Dependency:

```rust
// Before
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]

// After
#[derive(Debug, Clone, Serialize, Deserialize)]
```

**Why:**

- `FromRow` is SQLx-specific
- Tiberius uses manual row mapping

---

### 5. **API Handlers** (`backend/src/handlers/animals.rs`)

Complete rewrite to use Tiberius query API:

#### Before (SQLx):

```rust
let animals = sqlx::query_as::<_, Animal>(
    "SELECT ... FROM animals ORDER BY id"
)
.fetch_all(db.pool())
.await?;
```

#### After (Tiberius):

```rust
let mut client = db.connect().await?;

let stream = client.query(
    "SELECT id, name, species, habitat, endangered, created_at FROM animals ORDER BY id",
    &[]
).await?;

let rows = stream.into_first_result().await?;

let animals: Vec<Animal> = rows.iter().map(|row| {
    Animal {
        id: row.get::<i32, _>(0).unwrap_or(0),
        name: row.get::<&str, _>(1).unwrap_or("").to_string(),
        species: row.get::<&str, _>(2).unwrap_or("").to_string(),
        habitat: row.get::<&str, _>(3).map(|s| s.to_string()),
        endangered: row.get::<bool, _>(4).unwrap_or(false),
        created_at: row.get::<NaiveDateTime, _>(5).unwrap_or_else(|| chrono::Utc::now().naive_utc()),
    }
}).collect();
```

**Key Differences:**

- **Connection**: Create new client per request (vs shared pool)
- **Query**: Returns stream (vs direct rows)
- **Mapping**: Manual column-by-index mapping (vs automatic FromRow)
- **Nulls**: Explicit handling with `Option` and defaults
- **Parameters**: Use `@P1, @P2` syntax (not `?` or `$1`)

#### Parameterized Queries:

```rust
// For get_animal_by_id
let stream = client.query(
    "SELECT ... FROM animals WHERE id = @P1",
    &[&id]
).await?;
```

**Why @P1:**

- SQL Server uses named parameters
- `@P1` = first parameter, `@P2` = second, etc.
- Prevents SQL injection

---

### 6. **Main Application** (`backend/src/main.rs`)

#### Route Path Syntax Fix:

```rust
// Before (Axum 0.7 syntax)
.route("/animals/:id", get(get_animal_by_id))

// After (Axum 0.8 syntax)
.route("/animals/{id}", get(get_animal_by_id))
```

**Why:**

- Axum 0.8 changed path parameter syntax
- `:id` → `{id}` (follows web standards)

#### Database Initialization:

```rust
// Before
let database = Database::new().await?;

// After
let database = Database::new()?;
```

**Why:**

- `new()` is now sync (only config loading)
- Connection happens on first request

---

## How It Works Now

### Request Flow:

```
1. Frontend → GET http://localhost:3000/animals
   ↓
2. Axum routes to get_animals handler
   ↓
3. Handler calls db.connect() → creates new TCP connection
   ↓
4. Tiberius sends TDS query to SQL Server
   ↓
5. SQL Server returns result rows
   ↓
6. Handler maps rows to Animal structs manually
   ↓
7. Axum serializes to JSON
   ↓
8. Frontend ← JSON array with CORS headers
```

### Connection Lifecycle:

```
Each Request:
  ├─ Create new TCP connection
  ├─ Authenticate with SQL Server
  ├─ Execute query
  ├─ Get results
  ├─ Close connection
  └─ Return to handler
```

**Note:** No connection pooling (simpler but less efficient for high traffic)

---

## Database Type Mapping

| SQL Server         | Tiberius      | Rust           | Notes              |
| ------------------ | ------------- | -------------- | ------------------ |
| INT                | i32           | i32            | Primary key        |
| NVARCHAR(100)      | &str          | String         | via `.to_string()` |
| NVARCHAR(100) NULL | Option<&str>  | Option<String> | Nullable field     |
| BIT                | bool          | bool           | Boolean flag       |
| DATETIME           | NaiveDateTime | NaiveDateTime  | No timezone        |

---

## Testing Results

### ✅ Backend Logs:

```
Starting backend server...
Database config: sqlserver:1433/zoo_db
Connected to SQL Server successfully
Database connection test successful
Binding to 0.0.0.0:3000...
Server listening on port 3000
```

### ✅ API Response:

```bash
$ curl http://localhost:3000/animals
```

```json
[
  {
    "id": 1,
    "name": "Leo",
    "species": "Lion",
    "habitat": "Savanna",
    "endangered": true,
    "created_at": "2025-11-20T22:28:59.976666666"
  },
  {
    "id": 2,
    "name": "Ella",
    "species": "Elephant",
    "habitat": "Savanna",
    "endangered": true,
    "created_at": "2025-11-20T22:28:59.976666666"
  },
  {
    "id": 3,
    "name": "Panda",
    "species": "Giant Panda",
    "habitat": "Bamboo Forest",
    "endangered": true,
    "created_at": "2025-11-20T22:28:59.976666666"
  }
]
```

### ✅ Frontend Now Works:

- ❌ CORS error → ✅ Data loads successfully
- ❌ NetworkError → ✅ Animals displayed in grid
- ❌ Backend crash → ✅ Stable server

---

## Problems Resolved

### 1. **CORS Error**

**Cause:** Backend wasn't running (crashed on startup)  
**Solution:** Fixed database driver → backend starts → CORS headers work

### 2. **SQLx MSSQL Not Supported**

**Cause:** SQLx 0.8.6 doesn't have `mssql` feature  
**Solution:** Switched to Tiberius (native SQL Server driver)

### 3. **Connection Pool Timeout**

**Cause:** Trying to connect to MySQL port with SQL Server running  
**Solution:** Use Tiberius with SQL Server protocol (TDS)

### 4. **Route Path Syntax Error**

**Cause:** Axum 0.8 changed syntax from `:id` to `{id}`  
**Solution:** Updated route definitions

### 5. **Async Connection Issues**

**Cause:** TcpStream needs compatibility layer  
**Solution:** Used `tokio_util::compat::TokioAsyncWriteCompatExt`

---

## Performance Considerations

### ⚠️ No Connection Pooling:

- **Current:** New connection per request
- **Impact:** ~50-100ms overhead per request
- **Trade-off:** Simpler code, fewer dependencies

### 💡 Future Optimization:

```rust
// Could implement basic pooling with:
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct DatabasePool {
    connections: Arc<Mutex<Vec<Client<...>>>>,
}
```

**For Now:** Single-user development, performance is acceptable

---

## Security Notes

### ✅ Implemented:

- Parameterized queries (prevents SQL injection)
- Connection encryption with `trust_cert()`
- Environment variables for credentials

### ⚠️ Development Only:

```rust
config.trust_cert();  // Accepts self-signed certificates
```

### 🔒 Production TODO:

- Use proper SSL certificates
- Remove `trust_cert()` in production
- Implement connection pooling
- Add request rate limiting
- Use Azure Key Vault for credentials

---

## File Changes Summary

| File                      | Change Type | Description                   |
| ------------------------- | ----------- | ----------------------------- |
| `Cargo.toml`              | Modified    | Replaced sqlx with tiberius   |
| `.env`                    | Modified    | Changed to individual DB vars |
| `src/db.rs`               | Rewritten   | New Tiberius connection logic |
| `src/models/animal.rs`    | Modified    | Removed FromRow derive        |
| `src/handlers/animals.rs` | Rewritten   | Manual row mapping            |
| `src/main.rs`             | Modified    | Fixed route syntax, sync init |

---

## Next Steps

### ✅ Completed:

- Backend connects to SQL Server
- API endpoints return data
- Frontend displays animals
- CORS working

### 🔄 Recommended:

1. Add connection pooling for production
2. Implement error logging (tracing crate)
3. Add integration tests
4. Set up proper SSL certificates
5. Add database migrations
6. Implement POST/PUT/DELETE endpoints

---

## Commands to Test

### Start Services:

```bash
sudo docker-compose -f docker-compose.dev.yml up -d
```

### Test Backend:

```bash
curl http://localhost:3000/animals
curl http://localhost:3000/animals/1
```

### Check Logs:

```bash
sudo docker logs rust-backend-dev
```

### Frontend:

Open `http://localhost:5173` in browser

---

**End of Changelog**
