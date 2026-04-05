/* tslint:disable */
/* eslint-disable */

export function assign_province_to_state(province_id: string, state_id: string): void;

export function create_region(id: string, name: string): void;

export function create_state(id: string, name: string, region_id: string): void;

export function load_map_image(data: Uint8Array, width: number, height: number): void;

export function run_app(): void;

export function select_province_by_id(id: string): void;

export function set_active_region(region_id?: string | null): void;

export function trigger_export_map(): void;

export function update_brush_settings(active: boolean, state_id?: string | null): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly assign_province_to_state: (a: number, b: number, c: number, d: number) => void;
    readonly create_region: (a: number, b: number, c: number, d: number) => void;
    readonly create_state: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly load_map_image: (a: number, b: number, c: number, d: number) => void;
    readonly select_province_by_id: (a: number, b: number) => void;
    readonly set_active_region: (a: number, b: number) => void;
    readonly trigger_export_map: () => void;
    readonly update_brush_settings: (a: number, b: number, c: number) => void;
    readonly run_app: () => void;
    readonly wgpu_render_pass_draw: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wgpu_render_pass_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly wgpu_render_pass_set_pipeline: (a: number, b: bigint) => void;
    readonly wgpu_render_pass_set_viewport: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly wgpu_compute_pass_set_pipeline: (a: number, b: bigint) => void;
    readonly wgpu_render_pass_draw_indirect: (a: number, b: bigint, c: bigint) => void;
    readonly wgpu_render_bundle_draw: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wgpu_render_pass_set_bind_group: (a: number, b: number, c: bigint, d: number, e: number) => void;
    readonly wgpu_compute_pass_set_bind_group: (a: number, b: number, c: bigint, d: number, e: number) => void;
    readonly wgpu_render_pass_execute_bundles: (a: number, b: number, c: number) => void;
    readonly wgpu_render_pass_pop_debug_group: (a: number) => void;
    readonly wgpu_render_pass_write_timestamp: (a: number, b: bigint, c: number) => void;
    readonly wgpu_compute_pass_pop_debug_group: (a: number) => void;
    readonly wgpu_compute_pass_write_timestamp: (a: number, b: bigint, c: number) => void;
    readonly wgpu_render_pass_push_debug_group: (a: number, b: number, c: number) => void;
    readonly wgpu_render_pass_set_scissor_rect: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wgpu_compute_pass_push_debug_group: (a: number, b: number, c: number) => void;
    readonly wgpu_render_pass_set_vertex_buffer: (a: number, b: number, c: bigint, d: bigint, e: bigint) => void;
    readonly wgpu_render_pass_set_blend_constant: (a: number, b: number) => void;
    readonly wgpu_render_pass_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wgpu_compute_pass_set_push_constant: (a: number, b: number, c: number, d: number) => void;
    readonly wgpu_render_pass_end_occlusion_query: (a: number) => void;
    readonly wgpu_render_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
    readonly wgpu_render_pass_multi_draw_indirect: (a: number, b: bigint, c: bigint, d: number) => void;
    readonly wgpu_compute_pass_dispatch_workgroups: (a: number, b: number, c: number, d: number) => void;
    readonly wgpu_compute_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
    readonly wgpu_render_pass_begin_occlusion_query: (a: number, b: number) => void;
    readonly wgpu_render_pass_draw_indexed_indirect: (a: number, b: bigint, c: bigint) => void;
    readonly wgpu_render_pass_set_stencil_reference: (a: number, b: number) => void;
    readonly wgpu_render_bundle_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly wgpu_render_bundle_set_pipeline: (a: number, b: bigint) => void;
    readonly wgpu_render_bundle_draw_indirect: (a: number, b: bigint, c: bigint) => void;
    readonly wgpu_render_bundle_set_bind_group: (a: number, b: number, c: bigint, d: number, e: number) => void;
    readonly wgpu_render_pass_multi_draw_indirect_count: (a: number, b: bigint, c: bigint, d: bigint, e: bigint, f: number) => void;
    readonly wgpu_render_bundle_set_vertex_buffer: (a: number, b: number, c: bigint, d: bigint, e: bigint) => void;
    readonly wgpu_render_pass_multi_draw_indexed_indirect: (a: number, b: bigint, c: bigint, d: number) => void;
    readonly wgpu_render_bundle_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wgpu_compute_pass_dispatch_workgroups_indirect: (a: number, b: bigint, c: bigint) => void;
    readonly wgpu_render_pass_end_pipeline_statistics_query: (a: number) => void;
    readonly wgpu_compute_pass_end_pipeline_statistics_query: (a: number) => void;
    readonly wgpu_render_bundle_draw_indexed_indirect: (a: number, b: bigint, c: bigint) => void;
    readonly wgpu_render_pass_begin_pipeline_statistics_query: (a: number, b: bigint, c: number) => void;
    readonly wgpu_compute_pass_begin_pipeline_statistics_query: (a: number, b: bigint, c: number) => void;
    readonly wgpu_render_pass_multi_draw_indexed_indirect_count: (a: number, b: bigint, c: bigint, d: bigint, e: bigint, f: number) => void;
    readonly wgpu_render_bundle_insert_debug_marker: (a: number, b: number) => void;
    readonly wgpu_render_bundle_pop_debug_group: (a: number) => void;
    readonly wgpu_render_bundle_push_debug_group: (a: number, b: number) => void;
    readonly wgpu_render_bundle_set_index_buffer: (a: number, b: bigint, c: number, d: bigint, e: bigint) => void;
    readonly wgpu_render_pass_set_index_buffer: (a: number, b: bigint, c: number, d: bigint, e: bigint) => void;
    readonly wasm_bindgen__closure__destroy__h9b7edc1d0574efe9: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h4673408f593360d7: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__he0a5ea3872f01a50: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__ha291330413db2d3d: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__hacf5ed35587918bc: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h3f468ded062f9606: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h29f4e26104c0725a: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h95cac94e45cb856a: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__hc96061f7194344a9: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h44bfc77ed56929ed: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h3df0230212ae6e1b: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h29941624535d4e20: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h796347e32483f801: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h814adad3f1bf97e8: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h525114b7ea77b507: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hf14ca8a978f155a2: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h18950b7f0a02d22b: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__he19ffc1f47f3bd6b: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0a69b7016f72225f: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h15187aba60f36be4: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__he74128d0911aa9cd: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__he669d3e8d46be7dd: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h9ee8b4610bf128e2: (a: number, b: number) => number;
    readonly wasm_bindgen__convert__closures_____invoke__h50f1762cd5d1170d: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hbb39871592f15d58: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
