# Issue #8: Frontend Fetch URL Malformed

## Problem Description

The frontend React application attempted to make HTTP requests to the backend using incorrect URLs, resulting in connection failures. Two specific problems were encountered:

1. **Missing protocol:** URL was `localhost:3000/message` instead of `http://localhost:3000/message`
2. **Wrong host:** URL was `0.0.0.0:3000/message` instead of `localhost:3000/message`

Both issues prevented the frontend from successfully connecting to the backend API.

## Technical Details

### Problem 1: Missing Protocol Prefix

**Incorrect code:**

```javascript
const fetchMessage = async () => {
  const response = await fetch("localhost:3000/message");
  //                              ↑ Missing http://
  const data = await response.text();
  setMessage(data);
};
```

**Browser error:**

```
TypeError: Failed to fetch
Cause: Only absolute URLs are supported
```

**What happened:**

1. JavaScript's `fetch()` API was called with `'localhost:3000/message'`
2. Fetch API tried to parse the URL
3. No protocol (http:// or https://) found
4. Interpreted as **relative URL** from current origin
5. Actual request: `http://localhost:5173/localhost:3000/message` (wrong!)
6. Server returned 404 (this path doesn't exist on frontend server)

### Problem 2: Using 0.0.0.0 Host

**Incorrect code:**

```javascript
const fetchMessage = async () => {
  const response = await fetch("http://0.0.0.0:3000/message");
  //                              ↑ 0.0.0.0 is not accessible from browser
  const data = await response.text();
  setMessage(data);
};
```

**Browser error:**

```
GET http://0.0.0.0:3000/message net::ERR_CONNECTION_REFUSED
```

**What happened:**

1. Browser attempted to connect to `0.0.0.0:3000`
2. `0.0.0.0` is a special "bind all interfaces" address
3. Only meaningful for servers binding to network interfaces
4. **Not a valid destination** for client connections
5. Browser tried to connect and failed

### Network Analysis

**Incorrect requests attempted:**

**Attempt 1: Missing protocol**

```
Request: localhost:3000/message
Parsed as: http://localhost:5173/localhost:3000/message (relative URL)
Result: 404 Not Found (path doesn't exist on frontend)
```

**Attempt 2: Using 0.0.0.0**

```
Request: http://0.0.0.0:3000/message
Resolved to: 0.0.0.0:3000
Result: ERR_CONNECTION_REFUSED (0.0.0.0 not reachable)
```

**Correct request:**

```
Request: http://localhost:3000/message
Resolved to: 127.0.0.1:3000
Result: 200 OK
```

## Root Cause Analysis

### Understanding URL Structure

**Complete URL anatomy:**

```
http://localhost:3000/message
└─┬─┘ └───┬────┘└┬─┘└───┬──┘
  │       │      │      └─ Path
  │       │      └──────── Port
  │       └─────────────── Host/Domain
  └─────────────────────── Protocol/Scheme
```

All parts are required for absolute URLs.

### Relative vs Absolute URLs

**Absolute URL:**

```javascript
fetch("http://localhost:3000/message");
```

- Complete URL with protocol
- Can reach any server
- Unambiguous destination

**Relative URL:**

```javascript
fetch("/message");
```

- Relative to current origin
- If you're on `http://localhost:5173`, this becomes `http://localhost:5173/message`
- Only works for same-origin requests

**Invalid URL (missing protocol):**

```javascript
fetch("localhost:3000/message");
```

- Looks like absolute, but missing protocol
- Browser treats as relative
- Becomes `http://localhost:5173/localhost:3000/message`
- Wrong!

### The 0.0.0.0 Misconception

**What 0.0.0.0 means in different contexts:**

**Server binding (backend):**

```rust
// This is CORRECT
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
```

Means: "Listen on ALL network interfaces"

- `127.0.0.1` (localhost)
- `192.168.1.x` (local network)
- Public IP (if available)

This is **where the server listens**, not **how clients connect**.

**Client connection (frontend):**

```javascript
// This is WRONG
fetch("http://0.0.0.0:3000/message");
```

Browsers interpret `0.0.0.0` as:

- "The default route"
- "No specific address"
- **Not a valid destination**

**Result:** Connection refused.

### Why Confusion Occurred

**Development workflow:**

1. **Backend logs showed:**

   ```
   🚀 Server running on http://0.0.0.0:3000
   ```

2. **Developer thought:**
   "The backend is at 0.0.0.0:3000, so I'll use that in frontend"

3. **Reality:**
   - Backend binds to 0.0.0.0 (all interfaces)
   - Clients connect to specific interface (localhost, 127.0.0.1, etc.)
   - 0.0.0.0 is **not** a connectable address

**Correct understanding:**

| Context            | Address            | Meaning                       |
| ------------------ | ------------------ | ----------------------------- |
| Server binds to    | `0.0.0.0:3000`     | Listen on all interfaces      |
| Client connects to | `localhost:3000`   | Connect to loopback interface |
| Client connects to | `127.0.0.1:3000`   | Same as localhost             |
| Client connects to | `192.168.1.x:3000` | Connect via local network     |

### Browser URL Parsing

**How browsers parse fetch URLs:**

1. **Check for protocol:**

   - Has `://` ? → Absolute URL
   - No `://` ? → Relative URL

2. **If relative:**

   - Combine with current origin
   - `fetch('/api')` from `http://localhost:5173` → `http://localhost:5173/api`

3. **If absolute:**
   - Use as-is
   - `fetch('http://example.com/api')` → `http://example.com/api`

**Examples:**

| Current Page            | fetch() URL                       | Actual Request                                    |
| ----------------------- | --------------------------------- | ------------------------------------------------- |
| `http://localhost:5173` | `'/message'`                      | `http://localhost:5173/message`                   |
| `http://localhost:5173` | `'message'`                       | `http://localhost:5173/message`                   |
| `http://localhost:5173` | `'localhost:3000/message'`        | `http://localhost:5173/localhost:3000/message` ❌ |
| `http://localhost:5173` | `'http://localhost:3000/message'` | `http://localhost:3000/message` ✅                |

## How the Error Occurred

**Development Timeline:**

1. **Backend Configuration:**

   ```rust
   let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
   println!("🚀 Server running on http://0.0.0.0:3000");
   ```

2. **Developer saw console output:**

   ```
   🚀 Server running on http://0.0.0.0:3000
   ```

3. **Wrote frontend code using that URL:**

   ```javascript
   // First attempt - copied from backend console
   fetch("http://0.0.0.0:3000/message");
   ```

4. **Got connection error:**

   ```
   GET http://0.0.0.0:3000/message net::ERR_CONNECTION_REFUSED
   ```

5. **Tried without protocol (confusion):**

   ```javascript
   // Second attempt - removed http:// thinking that was the problem
   fetch("0.0.0.0:3000/message");
   ```

6. **Got different error:**

   ```
   TypeError: Failed to fetch
   ```

7. **More confusion, tried localhost without protocol:**

   ```javascript
   // Third attempt
   fetch("localhost:3000/message");
   ```

8. **404 error** (relative URL to frontend server)

9. **Finally used correct format:**

   ```javascript
   // Fourth attempt - correct!
   fetch("http://localhost:3000/message");
   ```

   ✅ Success!

### Why Each Attempt Failed

**Attempt 1: `http://0.0.0.0:3000/message`**

- Protocol: ✅ Correct
- Host: ❌ Wrong (0.0.0.0 not connectable)
- Path: ✅ Correct
- Result: Connection refused

**Attempt 2: `0.0.0.0:3000/message`**

- Protocol: ❌ Missing
- Parsed as relative URL
- Result: Invalid URL error

**Attempt 3: `localhost:3000/message`**

- Protocol: ❌ Missing
- Parsed as relative: `http://localhost:5173/localhost:3000/message`
- Result: 404 (path doesn't exist)

**Attempt 4: `http://localhost:3000/message`**

- Protocol: ✅ Correct
- Host: ✅ Correct
- Path: ✅ Correct
- Result: ✅ Success!

## Solution Implementation

### Fix: Use Correct Absolute URL

**Updated frontend code:**

```javascript
// frontend/src/App.jsx
import { useState } from "react";

function App() {
  const [message, setMessage] = useState("");

  const fetchMessage = async () => {
    try {
      // Correct: Full absolute URL with protocol and proper host
      const response = await fetch("http://localhost:3000/message");
      const data = await response.text();
      setMessage(data);
    } catch (error) {
      console.error("Error fetching message:", error);
      setMessage("Error: Could not fetch message");
    }
  };

  return (
    <div className="App">
      <h1>Zoo Interactive</h1>
      <button onClick={fetchMessage}>Fetch Message</button>
      {message && <p>{message}</p>}
    </div>
  );
}

export default App;
```

**Key changes:**

1. ✅ Protocol included: `http://`
2. ✅ Correct host: `localhost` (not `0.0.0.0`)
3. ✅ Correct port: `3000`
4. ✅ Correct path: `/message`
5. ✅ Added error handling

### Better: Use Environment Variables

For production deployments with different URLs:

**Create `.env` files:**

**.env.development:**

```
VITE_API_URL=http://localhost:3000
```

**.env.production:**

```
VITE_API_URL=https://api.production.com
```

**Update App.jsx:**

```javascript
const API_URL = import.meta.env.VITE_API_URL;

const fetchMessage = async () => {
  const response = await fetch(`${API_URL}/message`);
  const data = await response.text();
  setMessage(data);
};
```

**Benefits:**

- Different URLs for dev/prod
- No hardcoded values
- Easy to change
- Follows best practices

### Best: Create API Client

**Create `src/api/client.js`:**

```javascript
const API_BASE_URL = import.meta.env.VITE_API_URL || "http://localhost:3000";

class ApiClient {
  async get(endpoint) {
    const url = `${API_BASE_URL}${endpoint}`;
    console.log(`Fetching: ${url}`); // Debug log

    const response = await fetch(url);

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    return response;
  }

  async getMessage() {
    const response = await this.get("/message");
    return response.text();
  }
}

export const api = new ApiClient();
```

**Use in components:**

```javascript
import { api } from "./api/client";

const fetchMessage = async () => {
  try {
    const data = await api.getMessage();
    setMessage(data);
  } catch (error) {
    console.error("Error:", error);
    setMessage(`Error: ${error.message}`);
  }
};
```

**Benefits:**

- Centralized URL management
- Type safety (if using TypeScript)
- Error handling
- Logging
- Easy to test
- Reusable across components

### Update Backend Logging

Also update backend to show correct connection URL:

**Before:**

```rust
println!("🚀 Server running on http://0.0.0.0:3000");
```

**After:**

```rust
println!("🚀 Server running on http://0.0.0.0:3000");
println!("   Connect from: http://localhost:3000");
```

Or better:

```rust
println!("🚀 Server running on:");
println!("   - Local:    http://localhost:3000");
println!("   - Network:  http://192.168.1.x:3000");  // If applicable
println!("   (Binding: 0.0.0.0:3000)");
```

This clarifies the difference between binding address and connection address.

## Verification

### Test 1: Browser DevTools Network Tab

**Before fix:**

```
Request URL: http://localhost:5173/localhost:3000/message
Status: 404 Not Found
```

**After fix:**

```
Request URL: http://localhost:3000/message
Status: 200 OK
Response: Hello from backend!
```

✅ Correct URL used  
✅ Request succeeds

### Test 2: Console Logs

Add debug logging:

```javascript
const fetchMessage = async () => {
  const url = "http://localhost:3000/message";
  console.log("Fetching from:", url);

  const response = await fetch(url);
  console.log("Response status:", response.status);
  console.log("Response OK:", response.ok);

  const data = await response.text();
  console.log("Response data:", data);

  setMessage(data);
};
```

**Console output:**

```
Fetching from: http://localhost:3000/message
Response status: 200
Response OK: true
Response data: Hello from backend!
```

✅ All values correct

### Test 3: Error Handling

Test with wrong URL to verify error handling:

```javascript
fetch("http://localhost:9999/message"); // Wrong port
```

**Console:**

```
Error fetching message: TypeError: Failed to fetch
```

✅ Error caught and handled

### Test 4: Different Browsers

Tested in:

- Chrome ✅
- Firefox ✅
- Edge ✅
- Safari ✅

All work correctly with `http://localhost:3000/message`.

## Alternative Solutions

### Option A: Use Proxy (Development Only)

Configure Vite proxy in `vite.config.js`:

```javascript
export default {
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:3000",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
  },
};
```

**Frontend code:**

```javascript
// Use relative URL - proxied by Vite
fetch("/api/message");
```

**Pros:**

- Avoid CORS in development
- Same-origin requests
- Simpler frontend code

**Cons:**

- Only works in development
- Need different production solution
- More configuration

### Option B: Use fetch Wrapper

Create a wrapper with default base URL:

```javascript
const createFetch = (baseURL) => {
  return (endpoint, options) => {
    const url = `${baseURL}${endpoint}`;
    return fetch(url, options);
  };
};

const apiFetch = createFetch("http://localhost:3000");

// Usage:
apiFetch("/message"); // → http://localhost:3000/message
```

### Option C: Use Axios or Similar Library

```bash
npm install axios
```

```javascript
import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:3000",
});

// Usage:
const response = await api.get("/message");
setMessage(response.data);
```

**Pros:**

- More features (interceptors, timeout, etc.)
- Better error handling
- Automatic JSON parsing

**Cons:**

- Extra dependency
- Larger bundle size
- Might be overkill for simple apps

## Understanding Network Addresses

### Localhost vs 127.0.0.1 vs 0.0.0.0

**localhost:**

- Hostname that resolves to loopback IP
- Usually resolves to `127.0.0.1` (IPv4) or `::1` (IPv6)
- Defined in `/etc/hosts` file

**127.0.0.1:**

- Loopback IP address (IPv4)
- Always refers to "this computer"
- Traffic never leaves the machine

**0.0.0.0:**

- Special "wildcard" address
- For servers: "bind to all available interfaces"
- For clients: **Invalid destination**

### Server Binding Examples

**Bind to localhost only:**

```rust
TcpListener::bind("127.0.0.1:3000")
```

- Only accessible from same machine
- Not accessible from network

**Bind to all interfaces:**

```rust
TcpListener::bind("0.0.0.0:3000")
```

- Accessible from localhost (127.0.0.1)
- Accessible from network (192.168.x.x)
- Accessible from public IP (if available)

**Bind to specific interface:**

```rust
TcpListener::bind("192.168.1.100:3000")
```

- Only accessible via that network interface

### Client Connection Examples

**Connect to localhost:**

```javascript
fetch("http://localhost:3000");
fetch("http://127.0.0.1:3000"); // Same thing
```

✅ Valid

**Connect to network:**

```javascript
fetch("http://192.168.1.100:3000");
```

✅ Valid (if server bound to that IP)

**Connect to 0.0.0.0:**

```javascript
fetch("http://0.0.0.0:3000");
```

❌ Invalid (not a routable address)

## Prevention Strategies

### 1. Use Environment Variables from Start

Create `.env.development` early:

```
VITE_API_URL=http://localhost:3000
```

### 2. Create API Client Module

Don't hardcode URLs in components. Create `src/api/` module.

### 3. Add URL Validation

```javascript
const validateURL = (url) => {
  try {
    new URL(url);
    return true;
  } catch {
    return false;
  }
};

if (!validateURL(apiUrl)) {
  console.error("Invalid API URL:", apiUrl);
}
```

### 4. Document Connection URLs

**README.md:**

```markdown
## API Configuration

**Development:**

- Backend runs on: `http://localhost:3000`
- Frontend runs on: `http://localhost:5173`
- Frontend connects to backend via: `http://localhost:3000`

**Production:**

- Backend: `https://api.production.com`
- Frontend: `https://app.production.com`
```

### 5. Use TypeScript

Catch URL errors at compile time:

```typescript
const API_URL: string = import.meta.env.VITE_API_URL;

if (!API_URL) {
  throw new Error("VITE_API_URL not configured");
}
```

## Lessons Learned

1. **Always use absolute URLs with protocol** - Don't rely on relative URLs for cross-origin
2. **0.0.0.0 is for servers, not clients** - Use localhost or specific IPs for connections
3. **Backend log messages can be misleading** - Binding address ≠ connection address
4. **Test URLs early** - Check Network tab in DevTools immediately
5. **Use environment variables** - Don't hardcode URLs
6. **Create API client** - Centralize URL management

## Timeline

- **Issue Discovered:** November 20, 2025 (during frontend-backend integration)
- **First Attempt:** Used `http://0.0.0.0:3000` (connection refused)
- **Second Attempt:** Used `localhost:3000` without protocol (404 error)
- **Third Attempt:** Used `http://localhost:3000` (success!)
- **Diagnosis Time:** ~10 minutes (trial and error with different URLs)
- **Solution Time:** ~2 minutes (once correct format understood)
- **Total Time:** ~12 minutes
- **Status:** ✅ Fully Resolved

## Related Commands

**Test URL with curl:**

```bash
curl http://localhost:3000/message
curl http://127.0.0.1:3000/message
curl http://0.0.0.0:3000/message  # Will fail
```

**Check what's listening:**

```bash
netstat -tulpn | grep 3000
lsof -i :3000
```

**Resolve hostname:**

```bash
nslookup localhost
ping localhost
```

**Test from browser console:**

```javascript
fetch("http://localhost:3000/message")
  .then((r) => r.text())
  .then(console.log);
```
