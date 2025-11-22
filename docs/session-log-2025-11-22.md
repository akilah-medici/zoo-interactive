# Development Session Log - November 22, 2025

## Overview

This session focused on implementing animal care management features, improving the frontend listing functionality, resolving database queries, and managing version control.

---

## Features Implemented

### 1. Animal Care Management System

**Files Created:**

- `backend/src/models/animal_care.rs` - Model for Animal_Care_have table
- `backend/src/models/cares.rs` - Model for Cares table
- `backend/src/handlers/animal_cares.rs` - Handler for animal care operations
- `backend/src/handlers/cares.rs` - Handler for care operations

**Key Changes:**

- Created Rust models matching the database schema for `Animal_Care_have` and `Cares` tables
- Implemented GET handler for retrieving animal care records
- Implemented GET handler for retrieving care records
- Fixed SQL query to include all required columns (`animal_care_id` was initially missing)

**Database Schema:**

```sql
Animal_Care_have:
- animal_care_id (INT, PRIMARY KEY)
- date_of_care (DATE)
- fk_Cares_cares_id (INT, FOREIGN KEY)
- fk_Animal_animal_id (INT, FOREIGN KEY)
```

### 2. Frontend Improvements

**Files Created/Modified:**

- `frontend/src/pages/ListPage.jsx` - Animal listing with search functionality
- `frontend/src/pages/components/PopupCare.jsx` - Care information popup component
- `frontend/public/zoo-icon.svg` - Custom zoo icon
- `frontend/.gitignore` - Added to exclude node_modules from version control

**Key Features:**

- Search bar with filtering capability (name and species)
- Side-by-side layout for animal list and add animal form
- Date picker with Brazilian Portuguese locale (pt-BR)
- Hover popup functionality for showing additional animal information
- React key prop fixes to eliminate warnings

---

## Issues Resolved

### 1. SQL Server Column Name Typo

**Problem:** Column `frequency` was misspelled as `fesquency` in the database
**Solution:**

```sql
EXEC sp_rename 'dbo.Cares.fesquency', 'frequency', 'COLUMN';
```

**Learning:** Always use schema.table.column format in sp_rename to avoid ambiguity

### 2. SQL Command Line Access

**Problem:** `sqlcmd` not found at `/opt/mssql-tools/bin/sqlcmd`
**Solution:** Updated path to `/opt/mssql-tools18/bin/sqlcmd` for SQL Server 2022
**Command:**

```bash
docker exec -it sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P 'Password123' -C
```

**Note:** The `-C` flag is required to bypass SSL certificate verification for self-signed certificates

### 3. Animal Care Handler Query Error

**Problem:** SQL query didn't select `animal_care_id` but code tried to map it
**Original Query:**

```sql
SELECT date_of_care, fk_Cares_cares_id, fk_Animal_animal_id
FROM Animal_Care_have
ORDER BY animal_care_id
```

**Fixed Query:**

```sql
SELECT date_of_care, fk_Cares_cares_id, fk_Animal_animal_id, animal_care_id
FROM Animal_Care_have
ORDER BY animal_care_id
```

### 4. React Key Prop Warnings

**Problem:** List items rendered without unique keys
**Solution:** Added `key={animal.animal_id}` to mapped elements
**Learning:** React requires unique keys for efficient list rendering and updates

### 5. Git Merge Conflicts

**Problem:** node_modules files causing merge conflicts during rebase
**Solution:**

- Removed node_modules from version control
- Added frontend/.gitignore
- Resolved conflicts and continued rebase
  **Commands:**

```bash
git rm -rf frontend/node_modules/.vite frontend/node_modules/.package-lock.json
GIT_EDITOR=true git rebase --continue
```

---

## Best Practices Discussed

### 1. Data Filtering Strategy

**Current Approach:** Fetching entire tables to frontend and filtering client-side
**Issue:** Not scalable for large datasets (performance, bandwidth, security)
**Recommended Approach:**

- Filter and paginate on the backend
- Only send required data to frontend
- Use query parameters for search/filter operations
- Example: `/animals?search=lion` instead of fetching all and filtering in JS

### 2. Search Bar Implementation

**For Small Datasets:**

- Fetch once, filter in frontend on each keystroke
- Good for < 1000 records

**For Large Datasets:**

- Implement debounced backend search
- Send search query to backend after user stops typing
- Return filtered results only

### 3. React Component Organization

**Pattern Used:**

- Separate components into individual .jsx files
- Import and use with props
- Example: `<AnimalPopup animal={animal} />`
- Benefits: Code reusability, maintainability, testing

### 4. Hover Effects in React

**CSS Approach:** For simple styling

```css
.animal-card:hover {
  background-color: #f0f0f0;
}
```

**JavaScript Approach:** For dynamic behavior

```jsx
const [hovered, setHovered] = useState(false);
<div onMouseEnter={() => setHovered(true)}
     onMouseLeave={() => setHovered(false)}>
```

---

## Database Configuration

### Setting Default Database

**Options Discussed:**

1. Connection string: `Database=zoo_db` in connection string
2. User default: `ALTER LOGIN SA WITH DEFAULT_DATABASE = zoo_db;`
3. Script-based: `USE zoo_db; GO` at script start

**Recommendation:** Use connection string for application connections

---

## Git Operations Summary

### Commits Pushed:

1. `c5ac821` - feat: Add animal care management features and improve animal listing
2. `92b0d9a` - Delete frontend/node_modules directory
3. `06d8d73` - Fix typos in SQL scripts for table definitions and data insertion
4. `a4c1aca` - Refactor code structure for improved readability and maintainability

### Repository Status:

- Branch: main
- Remote: github.com:akilah-medici/zoo-interactive
- Status: Successfully pushed to origin/main

---

## Files Modified/Created Summary

### Backend (Rust):

- `src/models/animal_care.rs` - New model
- `src/models/cares.rs` - New model
- `src/handlers/animal_cares.rs` - New handler
- `src/handlers/cares.rs` - New handler
- `src/handlers/mod.rs` - Updated exports
- `src/models/mod.rs` - Updated exports
- `src/main.rs` - Added new routes

### Frontend (React):

- `src/pages/ListPage.jsx` - New page with search/filter
- `src/pages/components/PopupCare.jsx` - New popup component
- `src/pages/Dashboard.jsx` - Fixed key props
- `public/zoo-icon.svg` - New icon
- `.gitignore` - New file

### SQL:

- `sql/create-database.sql` - Schema verified
- `sql/insert-data.sql` - Sample data updated

### Documentation:

- `docs/animal-create-handler-log.md` - Previous session log
- `docs/frontend-errors-fix-log.md` - Previous session log
- `docs/session-log-2025-11-22.md` - This file

---

## Next Steps / Future Improvements

### Backend:

1. Implement pagination for large datasets
2. Add search/filter query parameters to animal endpoints
3. Add POST/PUT/DELETE handlers for animal care management
4. Implement proper error handling and validation
5. Add transaction support for related operations

### Frontend:

1. Implement debounced search for better performance
2. Add pagination controls
3. Add form validation for add animal
4. Implement edit/delete functionality
5. Add loading states and error boundaries
6. Create care assignment UI for animals

### Database:

1. Consider adding IDENTITY columns for auto-increment
2. Add indexes for frequently queried columns
3. Implement stored procedures for complex operations
4. Add audit tables for tracking changes

### DevOps:

1. Set up CI/CD pipeline
2. Add automated tests
3. Configure production-ready Docker setup
4. Add environment-specific configurations

---

## Technical Learnings

### SQL Server in Docker:

- SQL Server 2022 uses `/opt/mssql-tools18/bin/sqlcmd`
- `-C` flag bypasses SSL certificate verification
- `sp_rename` requires full schema qualification
- Always verify column names before writing queries

### React Best Practices:

- Always use unique keys in mapped lists
- Separate components for better organization
- Use props to pass data between components
- Consider performance implications of data fetching

### Git Workflow:

- Never commit node_modules to version control
- Use .gitignore early in project
- Resolve conflicts carefully during rebase
- Use GIT_EDITOR workaround when vi is not available

---

## Session Statistics

**Duration:** ~8 hours
**Commits:** 4
**Files Created:** 8
**Files Modified:** 5
**Lines Added:** ~700
**Issues Resolved:** 5

---

## Conclusion

This session successfully implemented the animal care management system, including database models, backend handlers, and frontend components. Key challenges around database configuration, SQL queries, and version control were resolved. The foundation is now in place for a scalable animal care tracking system with proper data filtering and user interface components.

**Status:** ✅ All changes committed and pushed to GitHub successfully
