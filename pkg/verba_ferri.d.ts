/* tslint:disable */
/* eslint-disable */

export class GameUI {
    free(): void;
    [Symbol.dispose](): void;
    constructor();
    new_game(): void;
    share(): void;
    submit_guess(guess: string): any;
    toggle_answer(): void;
}

export function dictionary_size(): number;

export function init_ui(): void;

export function submit_guess(guess: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_gameui_free: (a: number, b: number) => void;
    readonly gameui_new: () => [number, number, number];
    readonly gameui_new_game: (a: number) => void;
    readonly gameui_share: (a: number) => [number, number];
    readonly gameui_submit_guess: (a: number, b: number, c: number) => [number, number, number];
    readonly gameui_toggle_answer: (a: number) => void;
    readonly init_ui: () => void;
    readonly dictionary_size: () => number;
    readonly submit_guess: (a: number, b: number) => [number, number, number];
    readonly wasm_bindgen__convert__closures_____invoke__hb7ccabe4a4a3591c: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h8ec708de229d7649: (a: number, b: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
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
