# T7 — ScarlettPort diff (record)

Status: done. Read-only diff of ScarlettPort (`runtime/victory`) against our
tree, recursive SHA-256, 138 differing files (53 only there, 11 only
ours, 74 changed in both).

## Verdict

Do not copy wholesale. Keep our newer instrumentation (cardmeter,
drvwatch, fpsbox, framelog, mirrors, plaincopy, queue2, spread, tiles)
and our zlib-rs. Port candidates, ranked:

1. `REMAPPED` cap 1<<15 → 1<<18 (descriptor/host.rs). Exact match for our
   32,768 lost notifications; ~4 MiB extra queue. Lowest risk, port first.
2. DirectStorage IOCP pipeline + page-array wait (dstorage/pipeline.rs,
   dstorage.rs 1034-1065) and the RegionMap avoiding VirtualQuery walks
   (core/memmap.rs, gpu/regions.rs). Attacks host stalls during streaming.
3. Read-wait / copy-back reductions (readfix.rs, readback.rs, gpuown.rs):
   156→26 ms per frame on their 3060. High risk if partially ported.
4. Partial-mip uploads + banded detiling (formats/texture.rs 585-665,
   gpu/texture.rs 1421-1458, 1078-1135). Directly targets streaming
   freezes and black textures.
5. Pipeline validity: integer-target blending, dual-source to target 0,
   mesh-shader output linking under 32 KiB (emit/link.rs), merged ES-GS,
   wider lowering (ldexp/frexp, f64, POPS, float3 views), VICTORY_RT_MISS.
   Targets our 872 dropped draws.
6. `victory warm` + graphics PSO precompile (pipeline/precompiled.rs,
   make.rs 170-200). Helps if drops are "being made".
7. NVIDIA view-heap remapping (descriptor/slot_map.rs + lower.rs
   7361-7412): identity window + pool for slots past ~737,855.
8. Mirror placed R32_UINT UAV writes over R9G9B9E5 lighting views
   (descriptor/host.rs 3228-3291). Off by default.
9. Stand-ins: ambient GI floor, shadow-mask stub, constant exposure
   (emit/stand_in.rs, emit/exposure.rs; switches VICTORY_AMBIENT_STUB,
   VICTORY_SHADOWMASK_STUB, VICTORY_EXPOSURE_STUB). Hard-coded hashes.
10. Split dispatches over 65,535 groups (indirect.rs 375-397, lower.rs
    2457): the title asks for 115,200 groups in one gather.
11. VICTORY_SHADER_OPT off by default (walls/roofs removed on 3080/4080).
    A/B candidate for our missing geometry.
12. Series S plane composition, FidelityFX fallback, title hooks
    (render-scale lever), build-locked watchdog. Hooks are build-specific.
13. Launcher: not portable as-is (sets VICTORY_PIPELINE_WAIT_MS=0 and
    switches we do not implement).
14. Experimental memory work (ReBAR, mem2, pagemap, batch): defer; no
    measured gain; risky on 6 GB/16 GB.

## Risks

- Their runtime uses ~24 GB RAM (STATUS.md:25): a 2060/16 GB target
  needs our VRAM envelope work first.
- Restoring dropped draws and heap remapping can increase GPU work:
  correctness up, fps possibly down.
- The read-wait work needs the relay/precise-sync machinery; partial
  ports broke (black picture with COPY_BACK_ONLY_READBACK).

## Dead ends (none yet; this was read-only)

## Commands run

- Recursive SHA-256 comparison of the two crates trees; file inventories
  above.
