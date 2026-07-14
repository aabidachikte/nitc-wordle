/* tslint:disable */
/* eslint-disable */

export class GradedLetter {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly letter: string;
    readonly state: string;
}

export enum LetterState {
    Correct = 0,
    Present = 1,
    Absent = 2,
}

export class WordleGame {
    free(): void;
    [Symbol.dispose](): void;
    get_secret_word(): string;
    get_word_length(): number;
    is_game_over(): boolean;
    is_won(): boolean;
    constructor(secret_word: string);
    submit_guess(guess: string): GradedLetter[];
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_gradedletter_free: (a: number, b: number) => void;
    readonly __wbg_wordlegame_free: (a: number, b: number) => void;
    readonly gradedletter_letter: (a: number) => number;
    readonly gradedletter_state: (a: number) => [number, number];
    readonly wordlegame_get_secret_word: (a: number) => [number, number];
    readonly wordlegame_get_word_length: (a: number) => number;
    readonly wordlegame_is_game_over: (a: number) => number;
    readonly wordlegame_is_won: (a: number) => number;
    readonly wordlegame_new: (a: number, b: number) => number;
    readonly wordlegame_submit_guess: (a: number, b: number, c: number) => [number, number, number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __externref_drop_slice: (a: number, b: number) => void;
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
