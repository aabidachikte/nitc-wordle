# Technical Specification: Wiki-Driven NITC Wordle Micro-App

## 1. Document Overview
This document defines the architectural specification, data contract, and implementation blueprint for integrating the NITC Wordle game with the FOSS Cell MediaWiki API. 

### 1.1 Objective (Why)
The goal is to migrate the game from a static, hardcoded word array to a dynamic, community-driven system. By sourcing content dynamically via the Wiki's native random page generation mechanism, the application achieves zero-maintenance scalability, allowing anyone in the community to implicitly introduce new campus terms by expanding the wiki.

---

## 2. System Architecture & Boundaries (Where)
The architecture consists of three completely decoupled layers:
[ FOSS Cell Wiki API ]
│  (HTTPS GET / JSON Payload)
▼
[ Client Browser (JS Fetch Glue) ] ──(Data Pipeline & Filter)──► [ Rust WASM Game Engine ]
▲
│  (Static Asset Deployment)
[ Cloudflare Pages Edge ]


*   **Data Tier:** MediaWiki Web API hosted at `https://wiki.fosscell.org/api.php`.
*   **Application Frontend:** Client-side Rust WebAssembly (`wasm-bindgen`) bundle executed within the user's browser.
*   **Hosting Layer:** Globally distributed static hosting via Cloudflare Pages.

---

## 3. Technical Specification & Data Contracts (What & How)

### 3.1 API Data Acquisition
The engine will fetch candidate titles dynamically using MediaWiki's native randomized query module. 

*   **Target Endpoint:** `https://wiki.fosscell.org/api.php`
*   **Query Parameters:**
    *   `action=query`: Primary data retrieval action.
    *   `list=random`: Utilizes the wiki's native randomizer.
    *   `rnnamespace=0`: Restricts results strictly to main-space articles (ignores Talk, Template, and User profiles).
    *   `rnlimit=50`: Requests a batch of 50 candidates per handshake to optimize data density and minimize network round-trips.
    *   `format=json`: Ensures standard structured parsing capability.
    *   `origin=*`: Mandatory query parameter to explicitly bypass browser Cross-Origin Resource Sharing (CORS) enforcement.

*   **Constructed Production URL:**
    ```text
    [https://wiki.fosscell.org/api.php?action=query&list=random&rnnamespace=0&rnlimit=50&format=json&origin=](https://wiki.fosscell.org/api.php?action=query&list=random&rnnamespace=0&rnlimit=50&format=json&origin=)*
    ```

### 3.2 Client-Side Validation Pipeline (The "Junk Word" Filter)
Because `list=random` returns arbitrary page titles from the entire wiki database, the application wrapper will execute a strict multi-tiered sanitization pipeline before feeding data into the core Rust WASM state machine:

1.  **Length Constraints:** The candidate string must satisfy the boundary conditions $5 \le \text{length} \le 8$.
2.  **Character Invariance:** Must match the strict alphabetical regular expression: `^[a-zA-Z]+$`. Any title containing white spaces, numeric values, hyphens, brackets, or punctuation is instantly discarded.
3.  **Case Normalization:** Valid strings are piped through a lowercasing function to ensure uniform comparison matrices across the keyboard UI.

### 3.3 Resilient Fallback System
To protect user experience against network isolation, API latency spikes, or upstream MediaWiki downtime, a client-side hardcoded array of 20 verified campus terms will remain packaged inside the codebase. If the API response returns a status code $\neq 200$, throws a network exception, or yields zero valid words post-filtering, the game seamlessly falls back to this static array.

### 3.4 Deterministic Word of the Day Selection
To preserve the core Wordle mechanics where all concurrent daily users play against the exact same puzzle, the index of the active word is resolved deterministically using a calendar-based hash seed:

$$\text{Seed} = (\text{Year} \times 10000) + (\text{Month} \times 100) + \text{Day}$$
$$\text{Word Index} = \text{Hash}(\text{Seed}) \pmod{\text{Total Valid Words}}$$

---

## 4. Implementation & Governance Roadmap

*   **Phase 1: Repository Governance:** Creation of standard open-source licensing compliance. The codebase is officially licensed under the permissive **MIT License**.
*   **Phase 2: JavaScript Fetch Bridge:** Implement the asynchronous API handshake and structural extraction of the JSON response payload.
*   **Phase 3: Rust WASM Binding Refactor:** Update the Rust `WasmEngine::init()` entrypoint to accept dynamically allocated vectors rather than compiling with local static definitions.
*   **Phase 4: CI/CD Pipeline Configuration:** Bind the GitHub repository tracking the target branch directly to the Cloudflare Pages automated edge compilation script (`wasm-pack build --target web`).
