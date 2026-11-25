# T10 — victory-perf diff (record)

Status: done. Read-only diff of vitutu1/victory-perf (HEAD 52a6094,
commit c3faf2a = the 44-file perf pack) against our tree. 98 differing
paths. Their 10-14 fps is a Series S + 50% render-scale profile on a
4070 SUPER/64 GB; not comparable to our Series X baseline directly.

## Ranked port candidates

1. Texture staging budget: ours 192 MB / 250 ms (catch-up 576 MB /
   750 ms); theirs 48 MB / 10 ms (64 MB / 24 ms). draw.rs. Very high for
   the CPU-bound world.
2. heap_buffers 64-bucket smoothing + per-bucket spans (draw.rs 1560-1814,
   descriptor/host.rs 1416-1437). Kills the recurring 83,000-slot spike.
3. Submit-path O(N²) removal and lock fixes: hash-map aggregation of
   shader-written ranges, Batch refresh index, known-destination copy
   records, no-op setter skipping, atomic descriptor `made` state,
   shadow read-fast paths, NtReadVirtualMemory. host.rs/draw.rs/list.rs/
   descriptor/host.rs/shadow.rs.
4. Overlapped texture refresh + fast non-crypto hash (memcopy.rs,
   host.rs 6045-6225). Their pool: 1-3 threads.
5. Coalesced CARD_COMPARE + 128 MiB inline L0 readback ring (host.rs
   1565-1733, 3620-4313): 64 lanes coalesced, LDS reduction; our compare
   hashes each 64 KiB piece 64 times. System RAM, no ReBAR.
6. REMAPPED cap 1<<15 → 1<<18 (descriptor/host.rs 256-313). Exact match
   for our 32,768 losses.
7. DirectStorage late page-array wait (dstorage.rs 1043-1143): up to
   100 ms instead of reading immediately.
8. Ahead-of-time compute PSO precompile (apps precompile.rs,
   gpu/precompile.rs, pipeline/host.rs 463-510) + raw DXBC slicing fix.
   Bounded driver compiles.
9. Pipeline loading waits: shared 12 s window per kind (make.rs). Only
   if drops are "being made"; tune after precompile.
10. Shader coverage: ACCC/FSPX upscaler containers, split dispatches
    over 65,535 groups, quad/helper reads, SCC as data, packed half FMA,
    9-bit mesh index mask. Correctness, not steady fps.
11. Scoped clamp barriers + per-list UAV dedup (indirect.rs 258-353,
    host.rs 5149-5181) behind switches. GPU-side; our GPU is idle now.
12. Merged ES-GS lowering (merged.rs). Medium.
13. Partial-mip staging (formats/texture.rs 589-608, gpu/texture.rs).
    Overlaps our tiles.rs levels-on-demand: evaluate, not blind merge.
14. EVICT_IDLE_FRAMES=120 + relaxed eviction 4→30 frames (resource.rs).
    Careful on 6 GB.
15. Lighting stand-ins (stand_in.rs, exposure.rs; switches
    VICTORY_AMBIENT_STUB / VICTORY_SHADOWMASK_STUB / VICTORY_EXPOSURE_STUB)
    + Series S overlay + render-scale hooks (hash-locked to title build
    b99f876b...) + KBM. Visual fixes, not fps fixes.
16. ExitProcess HLE export fix (kernel.rs 250-286) — CRT exit-handler
    deadlock fix, worth extracting alone.

## Keep ours

Instrumentation (cardmeter, drvwatch, fpsbox, framelog, profile), Bink
mirrors, queue2, plaincopy, tiles, descriptor fingerprinting, page-write
notifications, own-view copy-back, staging pool, newer shader-cache
fingerprint, efficiency-mode opt-out, clzero fast path.

## Risky / defer

- ReBAR (VICTORY_REBAR) and COPY_BACK_ONLY_READBACK (RVA 0x1d40f90):
  off in their profile; defer for 2060/16 GB.
- Indirect windows + hoist mode 7 on by default there; we keep them off
  (device-loss history). Port scoped barriers separately.
- Their defaults (2 GB card buffers, 1.5 GB card windows) are tuned for
  12 GB. Reduce for 11 GB, much more for 6 GB.
- 128 MiB ring is system RAM; their census: ~18 GB private + 6.2 GB
  mapped. 16 GB target needs the VRAM envelope work.
- VICTORY_LIBRARY_TRIM=0 in their profile: bad default for 16 GB.
