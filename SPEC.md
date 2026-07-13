Overview
NITC Wordle is a custom word-guessing game built specifically for the NITC community. Instead of using a standard dictionary, it grabs its guessing words in real-time from the FOSS Cell Wiki. Every time you refresh the page, you get a fresh puzzle based on actual campus vocabulary, locations, and insider shorthand.

Technical Architecture
Core Pipeline Matrix (The 4-Step Lifecycle) When a player loads the game, four quick things happen behind the scenes:

Initialization: The web browser wakes up the super-fast game code targets via init().
The Grab: The browser reaches out across the web to pull text directly from the FOSS Cell Wiki database.
The Clean Up: The engine filters out messy code, cuts up sentences into individual words, and removes common filler language.
The Selection: A digital dice roll picks one perfect campus word and instantly sets up the game board loop object (new WordleGame()).

Cross-Origin Resource Isolation (CORS) Mitigation (Bypassing Browser Blocks) Web browsers have strict security rules that normally block a game hosted on one website from grabbing database info from a completely different website. To fix this without paying for a slow middleman server, we use a trick called JSONP. It disguises our data requests as standard, harmless script files (&callback=mediawiki_jsonp_XXXXX), allowing the campus words to stream into the game safely and instantly.


Text Sanitization & Stop-Word Filtering (Cleaning the Data) Raw text pulled randomly from wiki pages is messy and full of links, timestamps, and punctuation. The game runs all text through an aggressive cleaning filter:

Splitting: It breaks long sentences and page titles into single words using a whitespace filter: title.split(/[\s_]+/).
Cleaning: It forces all letters to lowercase and completely strips out numbers and punctuation symbols: replace(/[^a-zA-Z]/g, "").
Length Check: It throws away any word that is shorter than 5 letters or longer than 8 letters so it fits perfectly on a standard Wordle grid.
Campus Focus: It deletes boring filler words (like "which", "their", "about") to ensure only cool, relevant campus terms remain.

Fault-Tolerant Resilience (The Safety Net) If the campus Wi-Fi drops, firewalls block the connection, or the wiki goes completely offline, the game board cannot freeze. The game runs a built-in 2.0-second background stopwatch. If the wiki fails to reply before the timer hits zero, the game instantly cuts the line and loads a backup list of twenty pre-saved campus words (FALLBACK_WORDS). The player gets to keep playing seamlessly without seeing any error screens.

Resource Intensity & Performance
Server Stress (Zero Cost): 100% of the game logic, word cleaning, and randomizing happens inside the player's own browser. The FOSS Cell servers do zero background processing work.
Client Stress (Fast and Light): Because the game uses Rust and WebAssembly, it loads instantly, runs smoothly, and won't lag or drain a user's phone battery.
