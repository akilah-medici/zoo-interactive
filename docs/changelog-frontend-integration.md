# Frontend-Backend Integration Changelog

**Date:** November 20, 2025  
**Type:** Feature Implementation

---

## Overview

Implemented frontend integration to fetch and display animals from the database through the Rust backend API. The Dashboard now shows real-time animal data with statistics and detailed cards.

---

## Changes Made

### Dashboard Component (`frontend/src/pages/Dashboard.jsx`)

#### 1. **State Management Updated**

```jsx
// Before
const [stats, setStats] = useState(null);
const [animal, setAnimal] = useState();

// After
const [animals, setAnimals] = useState([]);
const [error, setError] = useState(null);
```

**Why:**

- `animals` array stores all fetched animals
- `error` state handles API failures gracefully
- Removed unused `stats` and singular `animal` states

---

#### 2. **API Fetch Function**

```jsx
async function fetchAnimals() {
  try {
    setLoading(true);
    setError(null);

    const response = await fetch("http://localhost:3000/animals");

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();
    setAnimals(data);
  } catch (err) {
    console.error("Error fetching animals:", err);
    setError(err.message);
  } finally {
    setLoading(false);
  }
}
```

**Features:**

- ✅ Proper error handling with try-catch
- ✅ HTTP status code validation
- ✅ Loading state management
- ✅ Error state for user feedback

**Fixed Issues:**

- Previous `get_data()` didn't return or await the fetch
- No error handling
- Incorrect state updates

---

#### 3. **Statistics Cards**

Displays calculated statistics from fetched data:

```jsx
<div style={styles.card}>
  <h3>Total Animals</h3>
  <p style={styles.number}>{animals.length}</p>
</div>

<div style={styles.card}>
  <h3>Endangered Species</h3>
  <p style={styles.number}>
    {animals.filter(a => a.endangered).length}
  </p>
</div>

<div style={styles.card}>
  <h3>Unique Habitats</h3>
  <p style={styles.number}>
    {new Set(animals.map(a => a.habitat).filter(Boolean)).size}
  </p>
</div>
```

**Calculations:**

- **Total Animals:** Array length
- **Endangered Count:** Filters animals where `endangered === true`
- **Unique Habitats:** Creates a Set from habitat names (removes duplicates), filters out null values

---

#### 4. **Animals Grid Display**

```jsx
<div style={styles.animalsGrid}>
  {animals.map((animal) => (
    <div key={animal.id} style={styles.animalCard}>
      <h3 style={styles.animalName}>{animal.name}</h3>
      <p>
        <strong>Species:</strong> {animal.species}
      </p>
      <p>
        <strong>Habitat:</strong> {animal.habitat || "Unknown"}
      </p>
      <p>
        <strong>Status:</strong>{" "}
        <span
          style={{
            color: animal.endangered ? "#d32f2f" : "#388e3c",
            fontWeight: "bold",
          }}
        >
          {animal.endangered ? "⚠️ Endangered" : "✓ Safe"}
        </span>
      </p>
      <p style={styles.dateText}>
        Added: {new Date(animal.created_at).toLocaleDateString()}
      </p>
    </div>
  ))}
</div>
```

**Features:**

- Responsive grid layout (auto-fills based on screen width)
- Color-coded endangered status (red for endangered, green for safe)
- Formatted date display using native JavaScript
- Handles null habitat values with fallback

---

#### 5. **Error Handling UI**

```jsx
{error ? (
  <div style={styles.errorBox}>
    <p style={styles.errorText}>❌ Error: {error}</p>
    <button style={styles.button} onClick={fetchAnimals}>
      Retry
    </button>
  </div>
) : ( ... )}
```

**Features:**

- Red error box with clear message
- Retry button to re-fetch data
- User-friendly error display

---

#### 6. **Empty State**

```jsx
{
  animals.length === 0 && (
    <p style={styles.emptyText}>No animals found in the database.</p>
  );
}
```

Handles case when database has no animals.

---

## Updated Styles

### New Style Properties:

```jsx
const styles = {
  // Container now centered with max-width
  container: {
    maxWidth: "1200px",
    margin: "0 auto",
  },

  // Stats cards with gradient background
  card: {
    background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
    color: "white",
    boxShadow: "0 4px 6px rgba(0,0,0,0.1)",
  },

  // Responsive grid for animals
  animalsGrid: {
    display: "grid",
    gridTemplateColumns: "repeat(auto-fill, minmax(280px, 1fr))",
    gap: "20px",
  },

  // Individual animal card styling
  animalCard: {
    background: "#fff",
    border: "2px solid #e0e0e0",
    boxShadow: "0 2px 8px rgba(0,0,0,0.1)",
    transition: "transform 0.2s, box-shadow 0.2s",
  },

  // Error styling
  errorBox: {
    background: "#ffebee",
    border: "2px solid #ef5350",
  },
};
```

---

## How It Works

### Data Flow:

```
1. Component Mounts
   ↓
2. useEffect triggers fetchAnimals()
   ↓
3. Fetch GET http://localhost:3000/animals
   ↓
4. Backend queries MySQL database
   ↓
5. SQLx maps rows to Animal structs
   ↓
6. Axum returns JSON array
   ↓
7. Frontend receives and parses JSON
   ↓
8. setAnimals(data) updates state
   ↓
9. React re-renders with animal data
   ↓
10. UI displays statistics and animal cards
```

### Example API Response:

```json
[
  {
    "id": 1,
    "name": "Leo",
    "species": "Lion",
    "habitat": "Savanna",
    "endangered": true,
    "created_at": "2025-11-20T22:28:59.977"
  },
  {
    "id": 2,
    "name": "Ella",
    "species": "Elephant",
    "habitat": "Savanna",
    "endangered": true,
    "created_at": "2025-11-20T22:28:59.977"
  }
]
```

---

## Testing

### 1. View the Frontend:

```bash
# Frontend should already be running on http://localhost:5173
# Open in browser
```

### 2. Expected Behavior:

**On Success:**

- Shows "Loading animals..." briefly
- Displays 3 statistics cards with correct counts
- Shows grid of animal cards with all data
- Each card shows name, species, habitat, status, and date

**On Error:**

- Shows red error box with message
- Displays "Retry" button
- Console logs detailed error

**Empty Database:**

- Shows "No animals found in the database."
- Statistics show 0s

---

## Problems Resolved

### 1. **Broken Fetch Logic**

**Before:** `get_data().then((res => res)).then(setAnimal(res))`

- Didn't await fetch
- No error handling
- Incorrect promise chaining
- `setAnimal(res)` called immediately instead of as callback

**After:** Proper async/await with error handling

### 2. **No Error Feedback**

**Before:** Errors were silent
**After:** User sees error message and can retry

### 3. **Static Placeholder Data**

**Before:** Hardcoded stats object
**After:** Dynamic calculations from real database data

### 4. **No Loading State Feedback**

**Before:** Generic "Loading data..."
**After:** Specific "Loading animals..." with proper state management

---

## Features Implemented

✅ Real-time database integration  
✅ Dynamic statistics calculation  
✅ Responsive grid layout  
✅ Error handling with retry  
✅ Loading states  
✅ Empty state handling  
✅ Color-coded endangered status  
✅ Date formatting  
✅ Null value handling (habitat)  
✅ Modern gradient UI

---

## Future Enhancements

- 🔄 Add animal filtering (by species, habitat, endangered status)
- 🔄 Add animal search functionality
- 🔄 Implement pagination for large datasets
- 🔄 Add animal detail modal/page
- 🔄 Implement CRUD operations (Create, Update, Delete)
- 🔄 Add sorting options
- 🔄 Real-time updates with WebSocket
- 🔄 Add animal images
- 🔄 Export to CSV/PDF

---

**End of Changelog**
