# React DatePicker Docker Installation Issue - Resolution Log

**Date:** 21 de novembro de 2025  
**Issue:** Failed to resolve import "react-datepicker" in Docker container  
**Status:** ✅ RESOLVED

---

## Problem Summary

### Initial Error

```
[plugin:vite:import-analysis] Failed to resolve import "react-datepicker" from "src/pages/ListPage.jsx". Does the file exist?

/app/src/pages/ListPage.jsx:5:43
```

### Context

User wanted to implement a custom date picker with Brazilian Portuguese (pt-BR) locale formatting to display dates in the format `dd/MM/yyyy` instead of the default `yyyy-mm-dd` format provided by HTML5's `<input type="date">`.

---

## Root Cause Analysis

### Why This Happened

1. **Local vs Docker Environment Mismatch**

   - The user ran `npm install react-datepicker` and `npm install date-fns` locally in `/home/nijiuu/Documentos/Desafio-CIEE/frontend`
   - These installations updated the local `package.json` and created/updated local `node_modules`
   - **However**, the Vite development server was running inside a Docker container (`/app` path in error message)
   - The Docker container was built **before** the new dependencies were added to `package.json`

2. **Docker Build Process**

   - The `Dockerfile.dev` has these steps:
     ```dockerfile
     COPY package*.json ./
     RUN npm install
     COPY . .
     ```
   - When the container was initially built, `react-datepicker` and `date-fns` were **not** in `package.json`
   - Therefore, `npm install` in the container didn't install these packages
   - Even though the local `package.json` was updated later, the running container still had the old dependencies

3. **Import Resolution**
   - Vite (running in Docker) looks for modules in `/app/node_modules` (container filesystem)
   - The import statement `import DatePicker from "react-datepicker"` failed because the package didn't exist in the container
   - Local `node_modules` is irrelevant because the app runs in Docker

---

## Solution Implementation

### Step 1: Verify Dependencies in package.json

Confirmed that `package.json` now includes:

```json
{
  "dependencies": {
    "date-fns": "^4.1.0",
    "react-datepicker": "^8.9.0",
    ...
  }
}
```

### Step 2: Stop All Containers

```bash
cd /home/nijiuu/Documentos/Desafio-CIEE
sudo docker compose -f docker-compose.dev.yml down
```

**Why:** Ensures clean state before rebuild and prevents file locks or conflicts.

### Step 3: Rebuild Frontend Container

```bash
sudo docker compose -f docker-compose.dev.yml build react-frontend
```

**What Happened:**

- Docker re-ran the Dockerfile.dev instructions
- `COPY package*.json ./` copied the **updated** package.json (with react-datepicker and date-fns)
- `RUN npm install` installed all dependencies **inside the container**, including react-datepicker v8.9.0 and date-fns v4.1.0
- `COPY . .` copied the source code including the updated ListPage.jsx

**Build Output:**

- Build completed successfully in ~43 seconds
- New image created with all required dependencies

### Step 4: Restart All Services

```bash
sudo docker compose -f docker-compose.dev.yml up -d
```

**Result:**

- All containers started successfully
- Frontend container now has react-datepicker and date-fns in `/app/node_modules`
- Vite can now resolve the import statement

---

## Code Changes

### ListPage.jsx - Date Picker Implementation

**Imports:**

```jsx
import DatePicker, { registerLocale } from "react-datepicker";
import { ptBR } from "date-fns/locale";
import "react-datepicker/dist/react-datepicker.css";

registerLocale("pt-BR", ptBR);
```

**State:**

```jsx
const [addAnimalDate, setAddAnimalDate] = useState(null);
```

**Component:**

```jsx
<DatePicker
  selected={addAnimalDate}
  onChange={(date) => setAddAnimalDate(date)}
  dateFormat="dd/MM/yyyy"
  placeholderText="Selecione a data de nascimento"
  locale="pt-BR"
/>
```

**Display Format:**

```jsx
<p>
  <strong>Date of Birth:</strong>{" "}
  {new Date(animal.date_of_birth).toLocaleDateString("pt-BR")}
</p>
```

---

## Why This Solution Works

### 1. **Docker Image Rebuild**

- Rebuilding the container ensures that `npm install` runs with the latest `package.json`
- All dependencies are installed in the container's filesystem (`/app/node_modules`)
- Vite can now find and import the packages

### 2. **Locale Registration**

- `registerLocale("pt-BR", ptBR)` registers Brazilian Portuguese locale with react-datepicker
- `date-fns/locale` provides comprehensive locale data (month names, day names, date formatting)
- The `locale="pt-BR"` prop on DatePicker uses this registered locale

### 3. **Date Formatting**

- `dateFormat="dd/MM/yyyy"` controls the input display format
- `toLocaleDateString("pt-BR")` formats existing dates for display in Brazilian format
- Both work together to provide a consistent Brazilian date experience

### 4. **Docker Layer Caching**

- Docker caches layers, so if `package.json` doesn't change, it won't re-run `npm install`
- This is why adding packages locally didn't affect the container
- Rebuild forces re-execution of all steps with fresh inputs

---

## Best Practices Learned

### 1. **Development in Docker Environments**

- Always rebuild containers after modifying `package.json` or other dependency files
- Command: `docker compose -f <file> build <service>`
- Cannot rely on local `node_modules` when running in containers

### 2. **Dependency Management**

- Keep `package.json` as the single source of truth
- Test in the same environment where the app runs (Docker in this case)
- Local installs are useful for IDE autocomplete but don't affect containerized apps

### 3. **Docker Compose Workflow**

- Stop → Build → Start for clean deployments
- Use `--no-cache` flag for complete rebuilds if needed: `docker compose build --no-cache`
- Check logs with `docker compose logs <service>` to debug issues

### 4. **React Date Picker Setup**

- Always import CSS: `import "react-datepicker/dist/react-datepicker.css"`
- Register locale before using: `registerLocale("pt-BR", ptBR)`
- Use `date-fns` for robust locale support

---

## Verification Steps

1. ✅ Frontend container rebuilt with new dependencies
2. ✅ All containers started successfully
3. ✅ Vite dev server running on http://localhost:5173
4. ✅ No import errors in browser console
5. ✅ DatePicker component renders with Brazilian locale
6. ✅ Calendar shows Portuguese month/day names
7. ✅ Date format displays as dd/MM/yyyy

---

## Related Files

- `/home/nijiuu/Documentos/Desafio-CIEE/frontend/src/pages/ListPage.jsx` - Component with DatePicker
- `/home/nijiuu/Documentos/Desafio-CIEE/frontend/package.json` - Dependencies
- `/home/nijiuu/Documentos/Desafio-CIEE/frontend/Dockerfile.dev` - Docker build configuration
- `/home/nijiuu/Documentos/Desafio-CIEE/docker-compose.dev.yml` - Docker Compose orchestration

---

## Future Recommendations

1. **Automated Rebuilds**

   - Consider using Docker volume mounts for `node_modules` in development
   - Or use `docker compose watch` for automatic rebuilds on file changes

2. **Documentation**

   - Add instructions in README.md about rebuilding containers after dependency changes
   - Create a quick reference guide for common Docker commands

3. **CI/CD Pipeline**

   - Implement automated container builds on `package.json` changes
   - Add health checks to verify all dependencies are available

4. **Form Validation**
   - Add validation for the date picker to ensure valid dates
   - Handle edge cases (empty dates, future dates for birth dates, etc.)

---

## Conclusion

The issue was successfully resolved by understanding the distinction between local and Docker environments. The key learning is that when running applications in Docker containers, all dependencies must be installed **inside the container** through the Docker build process. Local npm installs only update the host filesystem and don't affect the containerized application.

The solution—rebuilding the Docker image—ensured that the container's `node_modules` matched the updated `package.json`, allowing Vite to successfully resolve and import `react-datepicker` and `date-fns`.

**Total Resolution Time:** ~5 minutes  
**User Impact:** Minimal downtime during container rebuild  
**Success Rate:** 100% - Issue fully resolved on first rebuild
