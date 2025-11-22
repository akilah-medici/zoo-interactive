# Zoo Management System - Complete Implementation Guide

## Overview

This document provides comprehensive documentation for the zoo management system with frontend (React) and backend (Rust/Axum) integration. The system includes complete CRUD operations, soft-delete functionality, care management, and robust field validation.

**Key Features:**

- ✅ Full CRUD operations for animals
- ✅ Soft-delete (preserves data with `is_active` flag)
- ✅ Care type management with animal associations
- ✅ Sequential modify workflow for batch updates
- ✅ Field validation (frontend + backend)
- ✅ Date handling in multiple formats
- ✅ Responsive UI with centered layouts
- ✅ Popup positioning modes (inline/fixed)

---

## Table of Contents

1. [Database Schema](#database-schema)
2. [Backend API Reference](#backend-api-reference)
3. [Frontend Components](#frontend-components)
4. [Validation Rules](#validation-rules)
5. [Testing Guide](#testing-guide)
6. [Deployment](#deployment)
7. [Troubleshooting](#troubleshooting)
8. [Future Enhancements](#future-enhancements)

---

## Database Schema

---

## Database Schema

### Schema Overview

The database uses SQL Server 2022 with three main tables interconnected through foreign key relationships.

### Schema Updates

- **is_active column**: Added `BIT` type column to Animal table for soft-delete functionality
- All animals default to `is_active = 1` (active)
- Deactivated animals have `is_active = 0`
- **No IDENTITY columns**: All primary keys use manual ID generation with `ISNULL(MAX(id),0)+1`

### Tables Structure

#### 1. Animal Table

**Purpose**: Stores all animal information with soft-delete support

| Column            | Type    | Nullable | Description                     |
| ----------------- | ------- | -------- | ------------------------------- |
| animal_id         | INT     | NO       | Primary key, manually generated |
| name              | VARCHAR | NO       | **Required** - Animal name      |
| specie            | VARCHAR | NO       | **Required** - Species type     |
| habitat           | VARCHAR | YES      | Living environment              |
| description       | VARCHAR | YES      | Additional details              |
| country_of_origin | VARCHAR | YES      | Country of origin               |
| date_of_birth     | DATE    | YES      | Birth date                      |
| is_active         | BIT     | NO       | Default: 1 (active)             |

**Constraints:**

- `name` cannot be empty or whitespace only
- `specie` cannot be empty or whitespace only
- All GET operations filter by `is_active = 1`
- UPDATE operations only affect rows where `is_active = 1`

#### 2. Cares Table

**Purpose**: Defines types of care that can be provided to animals

| Column       | Type    | Nullable | Description                                    |
| ------------ | ------- | -------- | ---------------------------------------------- |
| cares_id     | INT     | NO       | Primary key, manually generated                |
| type_of_care | VARCHAR | NO       | **Required** - Care type name                  |
| description  | VARCHAR | YES      | Care details                                   |
| frequency    | VARCHAR | NO       | **Required** - How often (daily, weekly, etc.) |

**Constraints:**

- `type_of_care` cannot be empty or whitespace only
- `frequency` cannot be empty or whitespace only

#### 3. Animal_Care_have Table

**Purpose**: Junction table linking animals to their assigned cares

| Column              | Type | Nullable | Description                     |
| ------------------- | ---- | -------- | ------------------------------- |
| animal_care_id      | INT  | NO       | Primary key, manually generated |
| date_of_care        | DATE | YES      | When care was/will be performed |
| fk_Cares_cares_id   | INT  | NO       | Foreign key → Cares.cares_id    |
| fk_Animal_animal_id | INT  | NO       | Foreign key → Animal.animal_id  |

**Relationships:**

- One animal can have multiple care assignments
- One care type can be assigned to multiple animals
- Each assignment can have a specific date

### Initial Data

- **100 Sample Animals**: Populated with diverse species, all active (`is_active = 1`)
- **10 Care Types**: Feeding, grooming, medical checkup, vaccination, exercise, bathing, nail trimming, dental care, health monitoring, enrichment activities
- **Sample Care Assignments**: Pre-configured relationships demonstrating the system

---

## Backend API Reference

### Base URL

```
http://localhost:3000
```

### Response Codes

- `200 OK` - Successful GET request
- `201 CREATED` - Successful POST request (resource created)
- `204 NO CONTENT` - Successful DELETE/deactivate operation
- `400 BAD REQUEST` - Validation error or malformed request
- `404 NOT FOUND` - Resource not found
- `500 INTERNAL SERVER ERROR` - Database or server error

### Animals Endpoints

#### GET /animals/list

**Description**: Retrieve all active animals

**Response**: `200 OK`

```json
[
  {
    "animal_id": 2,
    "name": "Luna",
    "specie": "Pantera",
    "habitat": "Floresta",
    "description": "Animal dócil e calmo.",
    "country_of_origin": "Brasil",
    "date_of_birth": "2019-07-22"
  }
]
```

**Notes**:

- Only returns animals where `is_active = 1`
- Results ordered by `animal_id`

---

#### GET /animals/animals/{id}

**Description**: Retrieve a specific animal by ID

**Parameters**:

- `id` (path) - Animal ID

**Response**: `200 OK` or `404 NOT FOUND`

```json
{
  "animal_id": 2,
  "name": "Luna",
  "specie": "Pantera",
  "habitat": "Floresta",
  "description": "Animal dócil e calmo.",
  "country_of_origin": "Brasil",
  "date_of_birth": "2019-07-22"
}
```

---

#### POST /animals/add

**Description**: Create a new animal

**Request Body**:

```json
{
  "name": "Max",
  "specie": "Tiger",
  "habitat": "Jungle",
  "description": "Strong and agile",
  "country_of_origin": "India",
  "date_of_birth": "2020-05-15"
}
```

**Required Fields**:

- `name` (string, non-empty)
- `specie` (string, non-empty)

**Optional Fields**:

- `habitat` (string or null)
- `description` (string or null)
- `country_of_origin` (string or null)
- `date_of_birth` (string: "yyyy-MM-dd" or "dd/MM/yyyy", or null)

**Response**: `201 CREATED` or `400 BAD REQUEST`

```json
{
  "animal_id": 105,
  "name": "Max",
  "specie": "Tiger",
  "habitat": "Jungle",
  "description": "Strong and agile",
  "country_of_origin": "India",
  "date_of_birth": "2020-05-15"
}
```

**Validation Errors**:

- `"Name is required and cannot be empty"` - name is empty or whitespace
- `"Specie is required and cannot be empty"` - specie is empty or whitespace

---

#### PUT /animals/update/{id}

**Description**: Update an existing active animal

**Parameters**:

- `id` (path) - Animal ID

**Request Body** (all fields optional):

```json
{
  "name": "Max Updated",
  "specie": "Bengal Tiger",
  "habitat": "Tropical Jungle",
  "description": "Updated description",
  "country_of_origin": "Bangladesh",
  "date_of_birth": "2020-05-15"
}
```

**Response**: `200 OK`, `400 BAD REQUEST`, or `404 NOT FOUND`

**Notes**:

- Only updates fields provided in request body
- Uses COALESCE to preserve existing values for omitted fields
- Only affects animals where `is_active = 1`
- Returns updated animal object

**Validation Errors**:

- `"Name cannot be empty"` - name provided but empty
- `"Specie cannot be empty"` - specie provided but empty

---

#### POST /animals/deactivate/{id}

**Description**: Soft-delete an animal (set `is_active = 0`)

**Parameters**:

- `id` (path) - Animal ID

**Response**: `204 NO CONTENT` or `404 NOT FOUND`

**Notes**:

- Data remains in database (not deleted)
- Animal disappears from all GET requests
- Can be manually reactivated in database if needed
- Idempotent (safe to call multiple times)

---

### Cares Endpoints

#### GET /cares/list

**Description**: Retrieve all care types

**Response**: `200 OK`

```json
[
  {
    "cares_id": 1,
    "type_of_care": "Alimentacao",
    "frequency": "Diaria",
    "description": "Fornecimento de comida adequada"
  }
]
```

---

#### GET /cares/by-id/{id}

**Description**: Retrieve a specific care type by ID

**Parameters**:

- `id` (path) - Care ID

**Response**: `200 OK` or `404 NOT FOUND`

---

#### POST /cares/add

**Description**: Create a new care type

**Request Body**:

```json
{
  "type_of_care": "Grooming",
  "frequency": "Weekly",
  "description": "Brush and clean fur"
}
```

**Required Fields**:

- `type_of_care` (string, non-empty)
- `frequency` (string, non-empty)

**Optional Fields**:

- `description` (string or null)

**Response**: `201 CREATED` or `400 BAD REQUEST`

```json
{
  "cares_id": 12,
  "type_of_care": "Grooming",
  "frequency": "Weekly",
  "description": "Brush and clean fur"
}
```

**Validation Errors**:

- `"Type of care is required and cannot be empty"`
- `"Frequency is required and cannot be empty"`

---

### Animal-Care Relationship Endpoints

#### GET /animal-cares/list

**Description**: Retrieve all care assignments

**Response**: `200 OK`

```json
[
  {
    "animal_care_id": 1,
    "date_of_care": "2025-11-22",
    "fk_cares_cares_id": 1,
    "fk_animal_animal_id": 2
  }
]
```

---

#### GET /animal-cares/by-id/{id}

**Description**: Get a specific care assignment by ID

**Parameters**:

- `id` (path) - Animal care ID

**Response**: `200 OK` or `404 NOT FOUND`

---

#### GET /animal-cares/by-animal/by-id/{id}

**Description**: Get care assignments for a specific animal

**Parameters**:

- `id` (path) - Animal ID

**Response**: `200 OK` or `404 NOT FOUND`

---

#### POST /animal-cares/add

**Description**: Assign a care to an animal

**Request Body**:

```json
{
  "fk_animal_animal_id": 2,
  "fk_cares_cares_id": 1,
  "date_of_care": "2025-11-22"
}
```

**Required Fields**:

- `fk_animal_animal_id` (integer)
- `fk_cares_cares_id` (integer)

**Optional Fields**:

- `date_of_care` (string: "yyyy-MM-dd" or "dd/MM/yyyy", or null)

**Response**: `201 CREATED` or `500 INTERNAL SERVER ERROR`

```json
{
  "animal_care_id": 107,
  "date_of_care": "2025-11-22",
  "fk_cares_cares_id": 1,
  "fk_animal_animal_id": 2
}
```

---

## Frontend Components

### Layout Improvements

1. **Centered Design**

   - All pages wrapped in flex containers with centered alignment
   - Consistent max-width of 1200px across pages
   - Responsive and visually balanced

2. **Scrollable Lists**

   - Animal lists limited to 600px height with `overflow: auto`
   - Sticky headers remain visible during scroll
   - Headers styled with background color and z-index for proper layering

3. **Popup Positioning**
   - PopupCare component supports two modes:
     - `inline={true}`: Static positioning under parent element
     - `inline={false}`: Fixed positioning at cursor location
   - ListPage uses inline mode to display popup under "Create Animal" section

### CRUD Operations

#### Create (Add Animal)

- Form with 6 fields: name, specie, habitat, description, country_of_origin, date_of_birth
- Date picker with pt-BR locale support
- Backend auto-generates animal_id using `ISNULL(MAX(animal_id),0)+1`
- All new animals created with `is_active = 1`

#### Read (List Animals)

- Displays all active animals (`is_active = 1`)
- Search functionality by name or species
- Hover to view care information in popup

#### Update (Modify Animal)

**ModifyPage.jsx** - Sequential modify workflow:

1. Select multiple animals using checkboxes
2. Click "Modificar" button to start workflow
3. For each selected animal, a popup appears showing:
   - Full animal form (pre-filled with current data)
   - Checkbox to enable care management
   - Radio buttons to select:
     - Existing care (dropdown)
     - Create new care (3-field form)
   - Date picker for care assignment date
4. Sequential counter shows progress (e.g., "1 de 3")
5. Save updates the animal and creates care relationship
6. Cancel skips to next animal

**ModifyAnimalPopup.jsx** component:

- 335 lines
- Complete animal modification form
- Care management with checkbox enablement
- Supports both existing and new care assignment
- Backend integration for all operations

#### Delete (Soft Delete)

- Select animals using checkboxes
- Click "Excluir" button (shows count of selected)
- Backend sets `is_active = 0` instead of deleting rows
- Deactivated animals immediately removed from UI
- Data preserved in database for audit/recovery

## Backend Endpoints

### Animals (`/animals`)

| Method | Endpoint           | Handler             | Description                            |
| ------ | ------------------ | ------------------- | -------------------------------------- |
| GET    | `/list`            | `get_animals`       | Get all active animals (is_active = 1) |
| GET    | `/animals/{id}`    | `get_animal_by_id`  | Get single animal by ID                |
| POST   | `/add`             | `add_animal`        | Create new animal                      |
| PUT    | `/update/{id}`     | `update_animal`     | Update existing animal                 |
| POST   | `/deactivate/{id}` | `deactivate_animal` | Soft-delete animal (set is_active = 0) |

### Cares (`/cares`)

| Method | Endpoint      | Handler          | Description           |
| ------ | ------------- | ---------------- | --------------------- |
| GET    | `/list`       | `get_cares`      | Get all care types    |
| GET    | `/by-id/{id}` | `get_care_by_id` | Get single care by ID |
| POST   | `/add`        | `add_care`       | Create new care type  |

### Animal-Care Relationships (`/animal-cares`)

| Method | Endpoint                | Handler                        | Description                  |
| ------ | ----------------------- | ------------------------------ | ---------------------------- |
| GET    | `/list`                 | `get_animal_cares`             | Get all care assignments     |
| GET    | `/by-id/{id}`           | `get_animal_care_by_id`        | Get assignment by ID         |
| GET    | `/by-animal/by-id/{id}` | `get_animal_care_by_animal_id` | Get care for specific animal |
| POST   | `/add`                  | `add_animal_care`              | Assign care to animal        |

### Handler Details

#### `add_animal`

- Accepts `CreateAnimal` DTO with optional date string
- Parses date in two formats: `dd/MM/yyyy` or `yyyy-MM-dd`
- Auto-generates `animal_id` using SQL: `SELECT ISNULL(MAX(animal_id),0)+1`
- Returns `StatusCode::CREATED` with created Animal object

#### `deactivate_animal`

- Updates `is_active = 0` for specified animal_id
- Only affects animals where `is_active = 1` (idempotent)
- Returns `StatusCode::NO_CONTENT` on success
- Returns `StatusCode::NOT_FOUND` if animal doesn't exist or already inactive

#### `update_animal`

- Accepts `UpdateAnimal` DTO with all optional fields
- Uses COALESCE to preserve existing values for NULL fields
- Only updates animals where `is_active = 1`
- Fetches and returns updated animal after successful update

#### `add_care`

- Accepts `CreateCare` DTO
- Auto-generates `cares_id` using `ISNULL(MAX(cares_id),0)+1`
- Returns `StatusCode::CREATED` with created Care object

#### `add_animal_care`

- Accepts `CreateAnimalCare` DTO
- Parses optional date_of_care string
- Auto-generates `animal_care_id`
- Creates relationship in `Animal_Care_have` table
- Returns `StatusCode::CREATED` with created AnimalCare object

## Care Management

### Workflow

1. **Enable Care Management**: Checkbox in modify popup
2. **Select Care Type**:
   - **Existing Care**: Dropdown populated from `/cares/list`
   - **New Care**: Form with type_of_care, description, frequency
3. **Set Care Date**: Optional date picker for when care was/will be performed
4. **Save**:
   - Updates animal data
   - Creates new care if selected (calls `/cares/add`)
   - Creates relationship in Animal_Care_have (calls `/animal-cares/add`)

### Database Relations

- `Animal_Care_have.fk_Animal_animal_id` → `Animal.animal_id`
- `Animal_Care_have.fk_Cares_cares_id` → `Cares.cares_id`
- Multiple cares can be assigned to same animal over time
- Each assignment tracked with optional date

## Technical Implementation

### Date Handling

**Backend (Rust)**:

- Uses `chrono::NaiveDate` for date storage
- Parses two formats: `dd/MM/yyyy` and `yyyy-MM-dd`
- Stores dates in database as DATE type

**Frontend (React)**:

- Uses `react-datepicker` with pt-BR locale
- Date objects converted to ISO format (`yyyy-MM-dd`) for backend
- Display format: `dd/MM/yyyy` (Brazilian standard)

### ID Generation

All tables use manual ID generation (no IDENTITY columns):

```sql
SELECT ISNULL(MAX(table_id), 0) + 1 AS next_id FROM TableName
```

This approach ensures consistent ID generation across manual inserts and API operations.

### Error Handling

**Backend**:

- Database connection errors: `StatusCode::INTERNAL_SERVER_ERROR`
- Not found: `StatusCode::NOT_FOUND`
- Invalid data: Parse errors returned with appropriate status
- All errors logged to console with `eprintln!`

**Frontend**:

- Error state displayed in red above forms/lists
- Network errors caught and displayed to user
- Optimistic UI updates (add to list before backend confirms)

### CORS Configuration

- Backend uses `tower_http::cors::CorsLayer::permissive()`
- Allows all origins, methods, headers (development configuration)
- Production should restrict to specific frontend domain

## Testing Steps

### 1. Backend Startup

```bash
cd backend
cargo run
```

Server should start on `http://localhost:3000`

### 2. Frontend Startup

```bash
cd frontend
npm run dev
```

Frontend should start on `http://localhost:5173`

### 3. Test CRUD Operations

#### Create

1. Navigate to ListPage (`/`)
2. Fill in "Create Animal" form
3. Click "Adicionar Animal"
4. Verify animal appears in list below

#### Read

1. View list of animals on main page
2. Use search to filter by name/species
3. Hover over animal to see care popup

#### Update

1. Navigate to ModifyPage (`/modify`)
2. Select one or more animals with checkboxes
3. Click "Modificar (N)" button
4. Modify animal data in popup
5. Enable care management checkbox
6. Select existing care OR create new care
7. Click "Salvar"
8. Verify next popup appears (if multiple selected)
9. Verify changes reflected in list after completion

#### Delete

1. Navigate to ModifyPage
2. Select animals with checkboxes
3. Click "Excluir (N)" button
4. Verify animals removed from list
5. Check database to confirm `is_active = 0` (not deleted)

### 4. Database Verification

Check active animals:

```sql
SELECT COUNT(*) FROM Animal WHERE is_active = 1;
```

Check deactivated animals:

```sql
SELECT * FROM Animal WHERE is_active = 0;
```

Check care assignments:

```sql
SELECT a.name, c.type_of_care, ac.date_of_care
FROM Animal a
JOIN Animal_Care_have ac ON a.animal_id = ac.fk_Animal_animal_id
JOIN Cares c ON ac.fk_Cares_cares_id = c.cares_id
WHERE a.is_active = 1;
```

## Future Enhancements

### Priority 1 - Production Readiness

- [ ] Restrict CORS to production frontend domain
- [ ] Add authentication and authorization
- [ ] Implement proper error boundaries in React
- [ ] Add loading states for all async operations
- [ ] Implement optimistic UI rollback on failure

### Priority 2 - UX Improvements

- [ ] Confirmation dialogs for delete operations
- [ ] Undo functionality for soft-deleted animals
- [ ] Bulk edit functionality (apply same changes to multiple animals)
- [ ] Advanced search filters (by habitat, country, date range)
- [ ] Pagination for large animal lists

### Priority 3 - Features

- [ ] Care schedule/calendar view
- [ ] Photo upload for animals
- [ ] Export data to CSV/Excel
- [ ] Print-friendly animal profiles
- [ ] Care history timeline for each animal
- [ ] Animal statistics dashboard (species distribution, care frequency, etc.)

### Priority 4 - Technical Debt

- [ ] Add comprehensive unit tests (backend handlers)
- [ ] Add integration tests (API endpoints)
- [ ] Add React component tests
- [ ] Implement proper logging framework (replace eprintln!)
- [ ] Add database migrations framework
- [ ] Containerize application (Docker)
- [ ] Set up CI/CD pipeline

## Files Modified/Created

### Backend

- **Modified**:
  - `src/handlers/animals.rs` - Added `update_animal`, `deactivate_animal`
  - `src/handlers/cares.rs` - Added `add_care`
  - `src/handlers/animal_cares.rs` - Added `add_animal_care`
  - `src/models/animal_care.rs` - Updated `CreateAnimalCare` to use `Option<String>` for date
  - `src/main.rs` - Registered new routes for update, add_care, add_animal_care

### Frontend

- **Modified**:

  - `src/pages/ListPage.jsx` - Centered layout, scrollable list, inline popup
  - `src/pages/ModifyPage.jsx` - Added modify workflow, sequential popups, delete functionality
  - `src/pages/Dashboard.jsx` - Centered layout
  - `src/pages/components/PopupCare.jsx` - Added inline positioning mode

- **Created**:
  - `src/pages/components/ModifyAnimalPopup.jsx` - Complete modify popup with care management

### Documentation

- **Created**:
  - `docs/implementation-summary.md` - This document

---

## Validation Rules

### Frontend Validation (React)

#### Animal Creation/Update (ListPage.jsx & ModifyAnimalPopup.jsx)

**Required Fields**:

- ✅ `name` - Cannot be empty or contain only whitespace
- ✅ `specie` - Cannot be empty or contain only whitespace

**Optional Fields**:

- `habitat` - Trimmed before submission, null if empty
- `description` - Trimmed before submission, null if empty
- `country_of_origin` - Trimmed before submission, null if empty
- `date_of_birth` - Must be valid date or null

**Validation Messages**:

```javascript
"O campo Nome é obrigatório";
"O campo Espécie é obrigatório";
```

**Behavior**:

- Validation runs when form is submitted
- Error displayed in red above form
- Submission blocked until validation passes
- Text fields automatically trimmed

---

#### Care Creation (ModifyAnimalPopup.jsx)

**Required Fields**:

- ✅ `type_of_care` - Cannot be empty or whitespace only
- ✅ `frequency` - Cannot be empty or whitespace only

**Optional Fields**:

- `description` - Trimmed, null if empty

**Validation Messages**:

```javascript
"O campo Tipo de Cuidado é obrigatório";
"O campo Frequência é obrigatório";
"Por favor, selecione um cuidado existente ou crie um novo";
```

**Behavior**:

- Only validates when "Enable Care Management" is checked
- Must select existing care OR create new care (not both)
- If creating new care, both required fields must be filled

---

### Backend Validation (Rust/Axum)

#### Animal Endpoints

**POST /animals/add**:

```rust
// Validates before database operations
if payload.name.trim().is_empty() {
    return Err(StatusCode::BAD_REQUEST, "Name is required and cannot be empty");
}
if payload.specie.trim().is_empty() {
    return Err(StatusCode::BAD_REQUEST, "Specie is required and cannot be empty");
}
```

**PUT /animals/update/{id}**:

```rust
// Validates only if field is provided
if let Some(ref name) = payload.name {
    if name.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST, "Name cannot be empty");
    }
}
if let Some(ref specie) = payload.specie {
    if specie.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST, "Specie cannot be empty");
    }
}
```

---

#### Care Endpoints

**POST /cares/add**:

```rust
if payload.type_of_care.trim().is_empty() {
    return Err(StatusCode::BAD_REQUEST, "Type of care is required and cannot be empty");
}
if payload.frequency.trim().is_empty() {
    return Err(StatusCode::BAD_REQUEST, "Frequency is required and cannot be empty");
}
```

---

### Validation Testing

**Test Empty Name (Backend)**:

```bash
curl -X POST http://localhost:3000/animals/add \
  -H "Content-Type: application/json" \
  -d '{"name":"","specie":"Test"}'
# Response: 400 BAD REQUEST
# "Name is required and cannot be empty"
```

**Test Whitespace Only (Backend)**:

```bash
curl -X POST http://localhost:3000/animals/add \
  -H "Content-Type: application/json" \
  -d '{"name":"   ","specie":"Test"}'
# Response: 400 BAD REQUEST
# "Name is required and cannot be empty"
```

**Test Valid Data**:

```bash
curl -X POST http://localhost:3000/animals/add \
  -H "Content-Type: application/json" \
  -d '{"name":"Valid Name","specie":"Valid Species"}'
# Response: 201 CREATED
# Returns animal object with generated ID
```

---

## Testing Guide

### Prerequisites

- Docker and Docker Compose installed
- Ports 3000, 5173, and 1433 available

### Starting the Application

**1. Start All Services**:

```bash
cd /path/to/Desafio-CIEE
sudo docker compose -f docker-compose.dev.yml up
```

**2. Verify Services Running**:

```bash
sudo docker ps
```

Should show 3 containers:

- `rust-backend-dev` (port 3000)
- `react-frontend-dev` (port 5173)
- `sqlserver` (port 1433)

**3. Check Service Health**:

```bash
# Backend
curl http://localhost:3000/message
# Expected: "Hello from backend!"

# Frontend
curl http://localhost:5173
# Expected: HTML with Vite

# Database
sudo docker exec -it sqlserver /opt/mssql-tools18/bin/sqlcmd \
  -S localhost -U SA -P 'Password123' \
  -Q "SELECT COUNT(*) FROM Animal WHERE is_active = 1" -C
```

---

### Manual Testing Scenarios

#### Scenario 1: Create Animal with Validation

1. Open http://localhost:5173
2. Try to submit form with empty name
   - ✅ Should show "O campo Nome é obrigatório"
3. Fill name, leave specie empty
   - ✅ Should show "O campo Espécie é obrigatório"
4. Fill both required fields, submit
   - ✅ Should create animal and appear in list

---

#### Scenario 2: Modify Single Animal

1. Navigate to `/modify`
2. Select one animal with checkbox
3. Click "Modificar (1)"
4. Popup should show with animal data
5. Change name to empty, try to save
   - ✅ Should show "O campo Nome é obrigatório"
6. Fill valid data, click Save
   - ✅ Popup should close
   - ✅ Changes reflected in list

---

#### Scenario 3: Modify Multiple Animals with Care

1. Select 3 animals
2. Click "Modificar (3)"
3. First popup shows "1 de 3"
4. Enable care management
5. Try to save without selecting care
   - ✅ Should show "Por favor, selecione um cuidado..."
6. Select "Create New Care"
7. Leave type_of_care empty, try to save
   - ✅ Should show validation error
8. Fill care fields, save
   - ✅ Should move to "2 de 3"
9. Repeat for remaining animals
10. After last save, popup closes
    - ✅ Selection cleared
    - ✅ All changes reflected

---

#### Scenario 4: Delete Animals

1. Select 2-3 animals
2. Click "Excluir (N)"
3. ✅ Animals immediately removed from list
4. Check database:

```sql
SELECT name, is_active FROM Animal WHERE animal_id IN (2,3,4);
```

5. ✅ Should show `is_active = 0`

---

#### Scenario 5: Backend API Validation

```bash
# Test empty fields
curl -X POST http://localhost:3000/animals/add \
  -H "Content-Type: application/json" \
  -d '{"name":"","specie":"Test"}' -w "\nHTTP %{http_code}\n"
# ✅ Should return 400 with error message

# Test valid creation
curl -X POST http://localhost:3000/animals/add \
  -H "Content-Type: application/json" \
  -d '{"name":"API Test","specie":"Test Species"}' -w "\nHTTP %{http_code}\n"
# ✅ Should return 201 with animal object

# Test update validation
curl -X PUT http://localhost:3000/animals/update/2 \
  -H "Content-Type: application/json" \
  -d '{"name":""}' -w "\nHTTP %{http_code}\n"
# ✅ Should return 400 with error message

# Test care validation
curl -X POST http://localhost:3000/cares/add \
  -H "Content-Type: application/json" \
  -d '{"type_of_care":"","frequency":"Daily"}' -w "\nHTTP %{http_code}\n"
# ✅ Should return 400 with error message
```

---

## Known Issues

None currently identified.

## Support

For questions or issues, please refer to the project README or contact the development team.

---

**Last Updated**: November 22, 2025
**Version**: 1.1.0
**Changes**: Added comprehensive validation (frontend + backend), updated documentation with API reference, validation rules, and testing scenarios
