# BOARD

Rewritten each cycle. Task files: `coord/Tn.md`; done or closed ones move
to `coord/archive/`. Statuses: queued, in progress, blocked, done, void.

| Task | File | Goal (one line) | Status |
|------|------|-----------------|--------|
| T1 | T1-baseline.md | Capture the world-scene baseline (diag build, frame log + card meter) | in progress, blocked on a diag run that reaches the world |
| T2 | T2-shader-prs.md | Port the shader lowering fixes (mad_i64 literal, neg 16-bit/packed, ds_permute scatter; cache max-wave already upstream) | in progress |
| T3 | T3-bink-check.md | Confirm the Bink video plane fix in game | done (video plays; credits planes partially black, see record) |
| T4 | T4-switches.md | Validate the upstream frame-rate switches one by one against the baseline | queued |
| T5 | T5-ambient-light.md | Port the ambient-light stand-in (source: VItoriaBr, fork-based NVIDIA) | queued |
| T6 | T6-vram-envelope.md | Hold the 6 GB VRAM envelope (budgets, levels-on-demand test) | queued |
| T7 | T7-scarlettport-diff.md | Extract portable fixes from ScarlettPort (stand-in lighting, tile-grid fix) | done (record in archive/; port candidates ranked) |
| T8 | T8-dexnom-textures.md | Port DexNom sparse-texture and remap-notification fixes | queued |
| T9 | T9-remapped-cap.md | Raise the lost-texture-notification cap 1<<15 → 1<<18 (ScarlettPort #1) | queued |

Backlog (from the research agents, not yet briefed): integer-blend PSO
fixes, depth bounds, static sampler across uniform branch, GPU dirty-page
compare, own-view copy-back, wave64 split hardening.
