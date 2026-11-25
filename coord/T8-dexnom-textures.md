# T8 — Range-scoped remap pins (DexNom port)

Brief, cycle 1. Writer: to be assigned (DeepSeek V4 Pro). Reviewer: a
different model, fresh session.

## 1. Goal

Port DexNom's range-scoped memory-remap synchronization so a texture
refresh pins only the ranges it reads instead of stalling on every
title-memory remap. Our measured wall: 1,397.7 ms of one 1,690 ms frame
in "textures refresh waiting for memory to hold still"
(bench/baseline-02.md).

## 2. Read first

- `bench/baseline-02.md` (the measurement that motivates this).
- DexNom sources (the same fix, already ported there):
  `C:\Users\Tugakit\Desktop\Nouveau dossier\DexNom\crates\victory-gpu\src\watch.rs`
  lines 734-929 (range pins, writer/reader coordination, coalescing),
  `texture.rs` 294-315 and 762-775 (reads under the pin),
  `descriptor/host.rs` 273-310 and 1215-1247 (coalesced notifications,
  full-rescan fallback).
- Our current machinery: `D:\ztrx-repo\ztrx\crates\victory-gpu\src\watch.rs`
  (MAPPING RwLock, `title_memory_remapping`, `title_memory_reading`),
  `texture.rs` `refresh_timed` (waits `title_memory_reading`), and the
  REMAPPED queue in `descriptor/host.rs` (capped at 1<<15, clears on
  overflow).

## 3. Files in scope

- `crates/victory-gpu/src/watch.rs`
- `crates/victory-gpu/src/texture.rs`
- `crates/victory-gpu/src/descriptor/host.rs`
- `crates/victory-gpu/src/host.rs` (call sites only, if the API shape changes)

Forbidden: everything else, especially the shader and hle crates. T2 owns
`crates/victory-shader/**` right now; do not touch it.

## 4. Ownership matrix (to validate)

| Files | Writer |
|---|---|
| the four files above | T8 writer (sole owner) |
| crates/victory-shader/** | T2 writer (do not touch) |

## 5. Done line

- A texture refresh pins its own `[start, start + size)` range, and a
  remap waits only for readers that overlap the ranges it changes.
- The REMAPPED notification queue no longer silently drops: on overflow
  it falls back to a full rescan (keep the counter for the log).
- Our Bink mirrors integration still works: `crate::mirrors::remapped`
  is still called, and the source recompute in `refresh_timed` survives.
- `cargo test -p victory-gpu` passes (WARP tests included) and
  `cargo clippy -p victory-gpu --all-targets -- -D warnings` is clean.

## 6. Gate command

```
cargo test -p victory-gpu && cargo clippy -p victory-gpu --all-targets -- -D warnings
```

## 7. Stop rule

If the pin design needs a change in the remapping call sites in
`crates/victory-hle/src/modules/xmem.rs`, or a design decision about the
coalescing, stop and report. Do not choose an answer yourself.
