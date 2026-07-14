```javascript
// =============================================================================
//                     NITC WORDLE: SYSTEM PSEUDOCODE
// =============================================================================

// Global Configurations & Safety Nets
const FALLBACK_WORDS = ["ragam", "btech", "mtech", "links", "hosts", "clubs", "event", "batch", "board", "smart"];
const STATIC_BLACKLIST = ["image", "cargo", "thumb", "width", "align", "files", "photo", "notes", "admin", "thats"];
const ADMIN_PASSWORD = "opensesame";


// =============================================================================
// PHASE 1: SINGLE-QUERY MULTI-PAGE WORD GENERATION & FILTERING PIPELINE
// =============================================================================
async function fetchSingleCampusWord() {
    // Step 1: Initialize local client-side and static blacklists
    const local_blacklist = load_from_local_storage("nitc_wordle_blacklist");
    const active_blacklist = [...STATIC_BLACKLIST, ...local_blacklist];
    
    try {
        // Step 2: Fetch 5 random pages in 1 network request (grnlimit=5) to minimize server traffic
        const api_url = "[https://wiki.fosscell.org/api.php?action=query&generator=random&grnnamespace=0&grnlimit=5&prop=revisions&rvprop=content&format=json&origin=](https://wiki.fosscell.org/api.php?action=query&generator=random&grnnamespace=0&grnlimit=5&prop=revisions&rvprop=content&format=json&origin=)*";
        const api_response = await fetch(api_url);
        const pages = api_response.query.pages;
        const master_word_pool = [];
        
        for (const page of pages) {
            // Guard A: Namespace verification (Must be main article namespace)
            if (page.namespace !== 0) {
                continue; // skip page
            }
                
            // Guard B: Title structural verification (No subpages, users, categories, or templates)
            const title_lower = page.title.toLowerCase();
            if (
                title_lower.startsWith("user:") || 
                title_lower.startsWith("category:") || 
                title_lower.startsWith("template:") || 
                title_lower.includes("user") || 
                title_lower.includes("member") || 
                title_lower.includes("/")
            ) {
                continue; // skip page
            }
                
            // Guard C: Multi-word name verification (e.g., skips "First Middle Last" bios)
            if (count_words(page.title) >= 3) {
                continue; // skip page
            }
                
            // Guard D: Source code metadata verification (No personal profiles or rosters)
            const raw_content = page.revisions[0].content;
            const content_lower = raw_content.toLowerCase();
            if (
                content_lower.includes("category:members") || 
                content_lower.includes("category:people") || 
                content_lower.includes("{{member") || 
                content_lower.includes("{{profile")
            ) {
                continue; // skip page
            }
                
            // Step 3: Tokenize, sanitize, and extract valid 5-letter candidate words
            const page_tokens = split_text_into_individual_words(page.title + " " + raw_content);
            for (const token of page_tokens) {
                const clean_word = remove_special_characters(token).toLowerCase();
                
                if (clean_word.length === 5 && !active_blacklist.includes(clean_word)) {
                    master_word_pool.push(clean_word);
                }
            }
        }
                    
        // Step 4: Random selection from the filtered multi-page pool
        if (master_word_pool.length > 0) {
            return select_random_element(master_word_pool);
        } else {
            throw new Error("No clean words available on these pages");
        }
            
    } catch (error) {
        // Safety Fallback (Guarantees immediate game loading)
        return select_random_element(FALLBACK_WORDS);
    }
}


// =============================================================================
// PHASE 2: CORE GAMEPLAY LOOP & ADMIN CONTROL INTERACTION
// =============================================================================
function runWordleGame() {
    // Step 1: Initialize game and boot WebAssembly game state engine
    const secret_word = fetchSingleCampusWord();
    const wasm_engine = Initialize_WordleGame_WASM(secret_word);
    let current_row = 0;
    const max_rows = 6;
    
    // Step 2: Establish Admin-exclusive bypass controls
    if (URL_parameters.includes("admin=" + ADMIN_PASSWORD)) {
        show_element("Block & Skip Word button");
        
        on_click("Block & Skip Word button", () => {
            // Persist the banned word locally to the admin's browser
            save_to_local_storage("nitc_wordle_blacklist", secret_word);
            
            // Format and export the updated blocklist array for global hardcoding in index.html
            const local_blacklist = load_from_local_storage("nitc_wordle_blacklist");
            const merged_list = unique_elements([...STATIC_BLACKLIST, ...local_blacklist]);
            print_to_screen_for_copy(merged_list);
            
            // Reload the page instantly to fetch a fresh target word
            reload_current_webpage();
        });
    }
            
    // Step 3: Main Player Interaction Loop
    on_submit_guess(() => {
        const user_guess = read_input_field_value().toLowerCase();
        
        if (user_guess.length !== 5) {
            show_ui_message("Word must be exactly 5 letters");
            return;
        }
            
        try {
            // Run matching algorithm inside compiled WebAssembly
            const letter_evaluations = wasm_engine.submit_guess(user_guess);
            
            // Render results to screen
            render_grid_row_tiles(current_row, letter_evaluations);
            clear_input_field();
            
            // Evaluate game termination conditions
            if (wasm_engine.is_won()) {
                show_ui_message("🎉 You Won!");
                disable_input_field();
                hide_element("Block & Skip Word button");
            } else if (wasm_engine.is_game_over() || current_row === (max_rows - 1)) {
                show_ui_message("Game Over! The word was: " + secret_word.toUpperCase());
                disable_input_field();
                hide_element("Block & Skip Word button");
            } else {
                current_row++;
            }
                
        } catch (error) {
            show_ui_message(error.message);
        }
    });
}
