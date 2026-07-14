# 🎯 NITC Wordle (FOSS Cell Edition)

Welcome to the ultimate campus time-waster, engineering edition. This is a high-performance Wordle clone built using **Rust (WebAssembly)** for core game evaluations and real-time frontend interaction. 

The twist? The secret words are dynamically pulled directly from the live **FOSS Cell Wiki** using the MediaWiki Action API! You might guess a generic word, only to discover the secret answer is a campus event, engineering tech stack, or FOSS keyword.

---

## 🚀 Technical Highlights
* **WebAssembly Engine:** Core game logic (letter state calculations, budget allocations for duplicate characters) runs at raw assembly speeds compiled straight from Rust.
* **Live Scraper API Integration:** No static dictionary files. The game dynamically batches and queries random wiki pages on runtime, filtering out metadata/bios to build a clean vocabulary pool.
* **Instant Deployments:** Hosted entirely on Cloudflare Pages for rapid global delivery (and zero cost).

---

## 🤖 The Core Engine (Behind the Scenes)

Curious how the matching engine handles duplicate letter guesses (like guessing `PAPAP` against `APPLE`)? It implements a strict **Two-Pass Budget Allocation Algorithm** inside Rust to prevent visual rendering bugs:

```text
Function Submit_Guess(User_Guess):
    Initialize arrays for match evaluation states (set all to ABSENT)
    Map and count total frequency occurrences of letters in Target_Word (Letter Budget)
    
    // Pass 1: Green Check (Exact Matches)
    For index i from 0 to Word_Length:
        If User_Guess[i] == Target_Word[i]:
            Mark state at index i as CORRECT (Green)
            Decrement Letter Budget for User_Guess[i]
            
    // Pass 2: Yellow/Grey Check (Partial Matches)
    For index i from 0 to Word_Length:
        If state at index i is already CORRECT:
            Continue to next iteration
            
        Current_Char = User_Guess[i]
        If Current_Char exists in Letter Budget AND Budget[Current_Char] > 0:
            Mark state at index i as PRESENT (Yellow)
            Decrement Letter Budget for Current_Char
        Else:
            Mark state at index i as ABSENT (Grey)
            
    Return collection of graded states to Frontend JavaScript UI
