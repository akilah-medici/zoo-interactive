# Animal Creation Handler Implementation Log

**Date:** 21-11-2025  
**Status:** Completed  
**Endpoint Added:** `POST /animals`

---

## Summary

Implemented a new POST endpoint to allow the frontend to create animals in the SQL Server database. Adjusted routing to follow REST conventions by combining GET and POST on the `/animals` path. Fixed multiple issues during development related to table schema (non-IDENTITY primary key) and SQL parameter numbering.

---

## Changes Made

### 1. Added Create Handler

- File: `backend/src/handlers/animals.rs`
- Added `add_animal` function:
  - Accepts JSON payload (`CreateAnimal` DTO)
  - Parses optional `date_of_birth` supporting both `dd/MM/yyyy` and `yyyy-MM-dd`
  - Computes next `animal_id` manually since table column does not auto-increment
  - Inserts record and returns created `Animal` with `201 Created`

### 2. Updated Routing

- File: `backend/src/main.rs`
- Replaced separate add route `/animal/add` with combined method route: `GET /animals` and `POST /animals`
- Removed unused `post` import after switching to builder-style `.post(...)`

### 3. Model Usage

- File: `backend/src/models/animal.rs`
- Reused existing `CreateAnimal` struct; no changes required.

---

## Implementation Details

### Manual ID Generation

The `Animal` table rejected `NULL` for `animal_id` during insert, indicating **no IDENTITY** property. Insert error:

```
Cannot insert the value NULL into column 'animal_id'
```

Solution:

```sql
SELECT ISNULL(MAX(animal_id),0)+1 AS next_id FROM Animal;
```

Used this value in the insert statement.

### SQL Parameter Numbering Bug

Initial attempt used placeholders `@P0..@P6` which failed:

```
Must declare the scalar variable "@P0".
```

Tiberius expects parameters starting at `@P1`. Fixed query:

```sql
INSERT INTO Animal (...) VALUES (@P1,@P2,@P3,@P4,@P5,@P6,@P7)
```

### Date Parsing Logic

Handled two formats:

- Brazilian: `%d/%m/%Y`
- ISO: `%Y-%m-%d`
  Parsed into `Option<NaiveDate>` for insertion.

### Response Format

Returned JSON:

```json
{
  "animal_id": 4,
  "name": "Leão",
  "specie": "Panthera leo",
  "habitat": "Savana",
  "description": "Rei da selva",
  "country_of_origin": "África",
  "date_of_birth": "2020-11-21"
}
```

Dates serialize automatically via `serde` from `NaiveDate`.

---

## Testing Performed

### Build

```bash
cd backend
cargo build
```

Result: build succeeded.

### Docker Rebuild & Restart

```bash
sudo docker compose -f docker-compose.dev.yml build rust-backend
sudo docker compose -f docker-compose.dev.yml up -d rust-backend sqlserver
```

### POST Request

```bash
curl -X POST http://localhost:3000/animals \
  -H 'Content-Type: application/json' \
  -d '{
    "name":"Leão",
    "specie":"Panthera leo",
    "habitat":"Savana",
    "description":"Rei da selva",
    "country_of_origin":"África",
    "date_of_birth":"21/11/2020"
  }'
```

Returned 201 with created JSON.

### Verification

```bash
curl http://localhost:3000/animals | grep Leão
```

Confirmed row persisted.

---

## Errors Encountered & Resolutions

| Error                                      | Cause                                           | Resolution                          |
| ------------------------------------------ | ----------------------------------------------- | ----------------------------------- |
| NULL insert into animal_id                 | No IDENTITY on table                            | Manual `MAX+1` id generation        |
| Must declare @P0                           | Wrong parameter index start                     | Changed placeholders to `@P1..@P7`  |
| Initial success without identity retrieval | OUTPUT clause invalid for non-identity scenario | Removed OUTPUT, manual id selection |

---

## Why This Approach

- Non-invasive: Does not alter existing table schema (no migrations applied).
- Safe concurrency (acceptable for low-volume): `MAX+1` can theoretically race; acceptable for current dev stage. Future improvement: add IDENTITY or use locking/transaction.
- Flexible date parsing: Supports both frontend localization and ISO storage.
- REST consistency: Unified `/animals` path for list + create.

---

## Future Improvements

1. Add `IDENTITY` to `animal_id` to remove manual id logic.
2. Wrap insert in transaction to mitigate race on `MAX+1`.
3. Validate input (non-empty name/specie) and return `422` on invalid data.
4. Implement `PUT /animals/{id}` and `DELETE /animals/{id}` handlers.
5. Add logging (structured) instead of `println!/eprintln!`.

---

## Frontend Integration Notes

Frontend should `POST` to `/animals` with JSON matching `CreateAnimal`. For date selected via DatePicker, serialize to either ISO (`YYYY-MM-DD`) or Brazilian (`DD/MM/YYYY`); backend handles both.

Sample fetch:

```js
await fetch("http://localhost:3000/animals", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    name: newAnimal.name,
    specie: newAnimal.specie,
    habitat: newAnimal.habitat,
    description: newAnimal.description,
    country_of_origin: newAnimal.country_of_origin,
    date_of_birth: newAnimal.date_of_birth?.toISOString().slice(0, 10),
  }),
});
```

---

## Conclusion

The create handler is operational and persists new animals correctly. Issues encountered were resolved with minimal schema assumptions. System now supports frontend-driven creation of records.

---

**Author:** Automated implementation via AI assistant
