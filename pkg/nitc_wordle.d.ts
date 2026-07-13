/* tslint:disable */
/* eslint-disable */

export class GuessResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    letter: string;
    state: LetterState;
}

export enum LetterState {
    Correct = 0,
    Present = 1,
    Absent = 2,
}

export class WordleGame {
    free(): void;
    [Symbol.dispose](): void;
    is_game_over(): boolean;
    is_won(): boolean;
    constructor(secret_word: string);
    submit_guess(guess: string): any;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_get_guessresult_letter: (a: number) => number;
    readonly __wbg_get_guessresult_state: (a: number) => number;
    readonly __wbg_guessresult_free: (a: number, b: number) => void;
    readonly __wbg_set_guessresult_letter: (a: number, b: number) => void;
    readonly __wbg_set_guessresult_state: (a: number, b: number) => void;
    readonly __wbg_wordlegame_free: (a: number, b: number) => void;
    readonly wordlegame_is_game_over: (a: number) => number;
    readonly wordlegame_is_won: (a: number) => number;
    readonly wordlegame_new: (a: number, b: number) => number;
    readonly wordlegame_submit_guess: (a: number, b: number, c: number) => [number, number, number];
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
