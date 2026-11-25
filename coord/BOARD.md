# BOARD

Rewritten each cycle. Task files: `coord/Tn.md`; done or closed ones move
to `coord/archive/`. Statuses: queued, in progress, blocked, done, void.

| Task | File | Goal (one line) | Status |
|------|------|-----------------|--------|
| T1 | T1-baseline.md | Capture the world-scene baseline (diag build, frame log + card meter) | done (bench/baseline-02.md: CPU-bound on remap/texture sync, card 86 % idle) |
| T2 | T2-shader-prs.md | Port the shader lowering fixes (mad_i64 literal, neg 16-bit/packed, ds_permute scatter; cache max-wave already upstream) | in progress |
| T3 | T3-bink-check.md | Confirm the Bink video plane fix in game | done (video plays; credits planes partially black, see record) |
| T4 | T4-switches.md | Validate the upstream frame-rate switches one by one against the baseline | queued |
| T5 | T5-ambient-light.md | Port the ambient-light stand-in (source: VItoriaBr, fork-based NVIDIA) | queued |
| T6 | T6-vram-envelope.md | Hold the 6 GB VRAM envelope (budgets, levels-on-demand test) | queued |
| T7 | T7-scarlettport-diff.md | Extract portable fixes from ScarlettPort (stand-in lighting, tile-grid fix) | done (record in archive/; port candidates ranked) |
| T8 | T8-dexnom-textures.md | Port DexNom sparse-texture and remap-notification fixes | top priority: range-scoped remap pins (watch.rs) kill the 1,397 ms texture-refresh wait |
| T9 | T9-remapped-cap.md | Raise the lost-texture-notification cap 1<<15 → 1<<18 (ScarlettPort #1) | queued |
| T10 | archive/T10-victory-perf-diff.md | Diff vitutu1/victory-perf against our tree | done (record archived; ranked port list) |
| T11 | T11-texture-budget.md | Texture upload budget 192 MB/250 ms → 48 MB/10 ms (victory-perf #1) | in progress (writer) |
| T12 | T12-heap-buckets.md | heap_buffers 64-bucket smoothing + submit-path O(N²) removals | queued |
| T13 | T13-standins.md | Lighting stand-ins (ambient/shadowmask/exposure) behind switches | queued |
| T14 | T14-precompile.md | Ahead-of-time compute PSO precompile | queued |

Backlog (from the research agents, not yet briefed): integer-blend PSO
fixes, depth bounds, static sampler across uniform branch, GPU dirty-page
compare, own-view copy-back, wave64 split hardening.
