# Baseline 02 — world, diagnostics build

Date: 2026-11-25, evening. Build: diag (diagnostics), tree main at
`951c0be`. Machine: Ryzen 7 5800X3D, 32 GB RAM, RTX 2080 Ti 11 GB,
driver 621.40. Scene: the world after the intro, ~3,186 draws/frame.

## Frame breakdown (frame log + card meter)

Worst frame, 1,690 ms:

| Where | Time |
|---|---|
| Render thread in submit calls | 1,687.7 ms (41 submissions) |
| Submit thread preparing | 1,581.3 ms |
| Recording, all threads | 674.1 ms in 48,773 calls |
| Textures refresh | 1,407.4 ms, of which **1,397.7 waiting for memory to hold still** |
| Copies | 131.2 ms, 265.1 MB |
| Copy-back | 93.6 ms, 63.39 MB |
| Card | idle 1,454.8 ms (1,445.0 behind us); own lists 51.7 + 8.1 + 133.6 ms |

Usual frame, 352 ms: copies 97.9 ms, copy-back 198.6 ms, textures
refresh 81.3 ms (57.2 waiting), card idle 133.9 ms.

## Verdict

CPU-bound on memory synchronization. The texture refresh waits for the
title's memory remaps (1,397.7 ms of one frame); the card sits 86 % idle
behind the CPU. Copies + copy-back are the second wall.

## Consequence for the port order

1. DexNom's range-scoped remap pins (watch.rs): pin the exact ranges a
   texture reads, wait only overlapping readers (T8, top priority).
2. Remap coalescing: full-rescan fallback instead of silent loss at the
   32,768 cap (T9 + DexNom watch.rs).
3. Upstream switches on the measured walls: VICTORY_ASYNC_SUBMIT=1
   (render thread off the submit path), VICTORY_QUEUE2=1 (copy-back on a
   copy queue), VICTORY_RELAY=2 (card waits) — A/B one at a time.
4. victory-perf's heap_buffers incremental pruning (O(N²) spikes) and
   RegionMap (avoid VirtualQuery walks) once their diff lands.
