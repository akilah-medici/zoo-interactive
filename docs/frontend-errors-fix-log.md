# Frontend Error Resolution Log

**Date:** 21-11-2025  
**Status:** Completed

---

## Issues Addressed

### 1. CORS / Network Error When Adding Animal

**Error Message:**

```
Cross-Origin Request Blocked: The Same Origin Policy disallows reading the remote resource at https://0.0.0.0/animal. (Reason: CORS request did not succeed).
TypeError: NetworkError when attempting to fetch resource.
```

**Root Causes:**

- Wrong protocol (`https` instead of `http`).
- Wrong host syntax (`https:0.0.0.0/animal` missing `//`).
- Wrong endpoint path (`/animal` instead of `/animals`).
- Backend route expects `POST /animals` (configured in `main.rs`).

**Fix Implemented:**

- Updated `addAnimal` in `frontend/src/pages/ListPage.jsx` to:
  - Use endpoint: `http://localhost:3000/animals`.
  - Proper JSON body matching backend DTO (`name`, `specie`, etc.).
  - Convert selected date to ISO `YYYY-MM-DD` using `.toISOString().slice(0,10)`.
  - Added error handling and optimistic UI update.

**Why This Works:**

- Correct host and protocol allow successful request.
- Backend `CorsLayer::permissive()` already accepts cross-origin requests.
- Matching field names ensure deserialization by backend `CreateAnimal` struct.

### 2. React Key Warning in Dashboard

**Warning:**

```
Each child in a list should have a unique "key" prop.
```

**Root Cause:**

- Mapped JSX used `key={animal.id}` but backend returns `animal_id`.
- `animal.id` was `undefined`, causing all elements to have an invalid key.

**Fix Implemented:**

- In `frontend/src/pages/Dashboard.jsx` changed `key={animal.id}` to `key={animal.animal_id}`.
- Guarded date rendering: `animal.date_of_birth ? new Date(...).toLocaleDateString('pt-BR') : 'Unknown'`.

**Why This Works:**

- React requires stable unique keys to reconcile list items efficiently.
- Using `animal_id` prevents duplicate or missing key warnings.

### 3. Form State Binding Issues (Silent)

**Observation:** Some inputs lacked `name` attributes earlier; would prevent generic `handleChange` from updating state.
**Fix Implemented:** Ensured each input has a proper `name` attribute matching `newAnimal` object keys.

---

## Files Modified

| File                               | Change Summary                                                                                                 |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `frontend/src/pages/Dashboard.jsx` | Fixed key prop, guarded date formatting.                                                                       |
| `frontend/src/pages/ListPage.jsx`  | Implemented `addAnimal` with correct endpoint, payload, error handling, optimistic update, date serialization. |

---

## Code Snippets

**Corrected Add Function (`ListPage.jsx`):**

```jsx
async function addAnimal() {
  setError(null);
  const payload = {
    name: newAnimal.name,
    specie: newAnimal.specie,
    habitat: newAnimal.habitat || null,
    description: newAnimal.description || null,
    country_of_origin: newAnimal.country_of_origin || null,
    date_of_birth: newAnimal.date_of_birth
      ? newAnimal.date_of_birth.toISOString().slice(0, 10)
      : null,
  };
  try {
    const response = await fetch("http://localhost:3000/animals", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });
    if (!response.ok)
      throw new Error(`Failed to create animal: ${response.status}`);
    const created = await response.json();
    setAnimals((prev) => [...prev, created]);
    setFiltered((prev) => [...prev, created]);
    setNewAnimal({
      name: "",
      specie: "",
      habitat: "",
      description: "",
      country_of_origin: "",
      date_of_birth: null,
    });
  } catch (e) {
    setError(e.message);
  }
}
```

**Corrected Key Usage (`Dashboard.jsx`):**

```jsx
{
  animals.map((animal) => (
    <div key={animal.animal_id} style={styles.animalCard}>
      ...
    </div>
  ));
}
```

---

## Testing & Verification

### Manual Browser Test

1. Loaded `/list` page → Existing animals displayed without key warnings.
2. Filled form and clicked "Adicionar" → New animal appended immediately.
3. Opened DevTools console → No CORS/network errors.
4. Opened `/` (Dashboard) → No key warnings, new animal present.

### Backend Confirmation (Earlier Curl)

```bash
curl -X POST http://localhost:3000/animals \
 -H 'Content-Type: application/json' \
 -d '{"name":"Leão","specie":"Panthera leo","habitat":"Savana","description":"Rei da selva","country_of_origin":"África","date_of_birth":"2020-11-21"}'
```

Response contained persisted record including `animal_id`.

---

## Potential Future Improvements

1. Add user feedback on success (toast or success message).
2. Disable submit button while request is in-flight.
3. Form validation (required name/specie, date bounds).
4. Error boundary or retry logic UI.
5. Migrate manual ID allocation to database IDENTITY column to avoid race conditions.
6. Replace optimistic update with re-fetch for consistent ordering.

---

## Lessons Learned

- Ensure correct endpoint path and protocol before debugging CORS—often it’s a URL typo.
- React key warnings usually indicate wrong field names from backend response shape.
- Use consistent DTO field names across frontend/backend to prevent silent failures.
- Serialize dates explicitly to avoid locale-dependent parsing issues.

---

## Conclusion

All reported frontend issues (CORS/network failure and React key warning) were resolved by correcting endpoint URL, payload structure, and key prop usage. The application now successfully creates and lists animals without warnings or network errors.

---

**Author:** AI Assistant Automated Implementation
