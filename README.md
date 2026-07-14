
    ================================================================================
                     NITC WORDLE: SYSTEM PSEUDOCODE
    ================================================================================

     // Global Configurations & Safety Nets
    GLOBAL CONSTANT FALLBACK_WORDS = ["ragam", "btech", "mtech", "links", "hosts", "clubs", "event", "batch", "board", "smart"]
    GLOBAL CONSTANT STATIC_BLACKLIST = ["image", "cargo", "thumb", "width", "align", "files", "photo", "notes", "admin", "thats"]
    GLOBAL CONSTANT ADMIN_PASSWORD = "opensesame"


    // =============================================================================
    // PHASE 1: SINGLE-QUERY MULTI-PAGE WORD GENERATION & FILTERING PIPELINE
    // =============================================================================
      FUNCTION fetchSingleCampusWord():
    // Step 1: Initialize local client-side and static blacklists
    local_blacklist = load_from_local_storage("nitc_wordle_blacklist")
    active_blacklist = STATIC_BLACKLIST + local_blacklist
    
    TRY:
        // Step 2: Fetch 5 random pages in 1 network request (grnlimit=5) to minimize server traffic
        api_response = fetch("https://wiki.fosscell.org/api.php?action=query&generator=random&grnnamespace=0&grnlimit=5&prop=revisions&rvprop=content&format=json&origin=*")
        pages = api_response.query.pages
        master_word_pool = []
        
        FOR EACH page IN pages:
            // Guard A: Namespace verification (Must be main article namespace)
            IF page.namespace != 0:
                CONTINUE (skip page)
                
            // Guard B: Title structural verification (No subpages, users, categories, or templates)
            title_lower = page.title.to_lowercase()
            IF title_lower starts with "user:" OR "category:" OR "template:" 
               OR title_lower contains "user" OR "member" OR title_lower contains "/":
                CONTINUE (skip page)
                
            // Guard C: Multi-word name verification (e.g., skips "First Middle Last" bios)
            IF count_words(page.title) >= 3:
                CONTINUE (skip page)
                
            // Guard D: Source code metadata verification (No personal profiles or rosters)
            raw_content = page.revisions[0].content
            content_lower = raw_content.to_lowercase()
            IF content_lower contains "category:members" OR "category:people" OR "{{member" OR "{{profile":
                CONTINUE (skip page)
                
            // Step 3: Tokenize, sanitize, and extract valid 5-letter candidate words
            page_tokens = split_text_into_individual_words(page.title + " " + raw_content)
            FOR EACH token IN page_tokens:
                clean_word = remove_special_characters(token).to_lowercase()
                
                IF length_of(clean_word) == 5 AND clean_word NOT IN active_blacklist:
                    master_word_pool.add(clean_word)
                    
        // Step 4: Random selection from the filtered multi-page pool
        IF size_of(master_word_pool) > 0:
            RETURN select_random_element(master_word_pool)
        ELSE:
            THROW ERROR ("No clean words available on these pages")
            
    CATCH ERROR (network timeout, api error, or empty master pool):
        // Safety Fallback (Guarantees immediate game loading)
        RETURN select_random_element(FALLBACK_WORDS)


    // =============================================================================
    // PHASE 2: CORE GAMEPLAY LOOP & ADMIN CONTROL INTERACTION
    // =============================================================================
    FUNCTION runWordleGame():
    // Step 1: Initialize game and boot WebAssembly game state engine
    secret_word = fetchSingleCampusWord()
    wasm_engine = Initialize_WordleGame_WASM(secret_word)
    current_row = 0
    max_rows = 6
    
    // Step 2: Establish Admin-exclusive bypass controls
    IF URL_parameters contain "admin=" + ADMIN_PASSWORD:
        SHOW "Block & Skip Word" button
        
        ON "Block & Skip Word" button click:
            // Persist the banned word locally to the admin's browser
            save_to_local_storage("nitc_wordle_blacklist", secret_word)
            
            // Format and export the updated blocklist array for global hardcoding in index.html
            local_blacklist = load_from_local_storage("nitc_wordle_blacklist")
            merged_list = unique_elements(STATIC_BLACKLIST + local_blacklist)
            PRINT_TO_SCREEN_FOR_COPY(merged_list)
            
            // Reload the page instantly to fetch a fresh target word
            reload_current_webpage()
            
    // Step 3: Main Player Interaction Loop
    ON user_presses_enter_key_in_input_field:
        user_guess = read_input_field_value().to_lowercase()
        
        IF length_of(user_guess) != 5:
            SHOW_UI_MESSAGE("Word must be exactly 5 letters")
            RETURN
            
        TRY:
            // Run matching algorithm inside compiled WebAssembly
            letter_evaluations = wasm_engine.submit_guess(user_guess)
            
            // Render results to screen
            render_grid_row_tiles(current_row, letter_evaluations)
            clear_input_field()
            
            // Evaluate game termination conditions
            IF wasm_engine.is_won():
                SHOW_UI_MESSAGE("🎉 You Won!")
                disable_input_field()
                HIDE "Block & Skip Word" button
                
            ELSE IF wasm_engine.is_game_over() (current_row == max_rows - 1):
                SHOW_UI_MESSAGE("Game Over! The word was: " + secret_word.to_uppercase())
                disable_input_field()
                HIDE "Block & Skip Word" button
                
            ELSE:
                current_row = current_row + 1
                
        CATCH ERROR (invalid guess dictionary validation error):
            SHOW_UI_MESSAGE(error_message_from_engine)
