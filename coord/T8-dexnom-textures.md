# T8 — Range-scoped remap pins (DexNom port)

Brief, cycle 1. Writer: DeepSeek V4 Pro. Reviewer: hungerg-free, REJECT
(scope note only, see record). Lead decision: merge as partial
infrastructure; T15 completes the xmem side.

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
- Our current machinery: `crates/victory-gpu/src\watch.rs`
  (MAPPING RwLock, `title_memory_remapping`, `title_memory_reading`),
  `texture.rs` `refresh_timed` (waits `title_memory_reading`), and the
  REMAPPED queue in `descriptor/host.rs` (capped at 1<<15, clears on
  overflow).

## 3. Files in scope

- `crates/victory-gpu/src/watch.rs`
- `crates/victory-gpu/src/texture.rs`
- `crates/victory-gpu/src/descriptor/host.rs`
- `crates/victory-gpu/src/host.rs` (call sites only, if the API shape changes)

Forbidden: everything else, especially the shader and hle crates.

## 4. Ownership matrix

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

---

## Record (DeepSeek V4 Pro, writer)

### Files touched

- crates/victory-gpu/src/watch.rs
- crates/victory-gpu/src/texture.rs
- crates/victory-gpu/src/descriptor/host.rs
- crates/victory-gpu/src/host.rs (one frame-line label string only)

### Decisions and why

- MAPPING is now DexNom's shape: a Mutex over { changing: Option<Range>,
  readers: Vec<Range> } plus a WRITER mutex and a Condvar, instead of the
  RwLock<()>. `title_memory_remapping()`/`title_memory_reading()`/
  `title_memory_reading_now()` keep their signatures and delegate to
  pub(crate) _range versions with ALL_MEMORY = 0..usize::MAX, so xmem,
  draw.rs and inplace.rs compile untouched (all out of scope).
- The _range entry points are pub(crate), not re-exported in lib.rs
  (lib.rs is out of scope). The lead's xmem patch needs a one-line
  export + `title_memory_remapping_range(unmap_extent(...))` call there.
- Remapping no longer records DexNom's physical-span diff: our xmem calls
  `title_memory_remapped` itself after the change, so the Drop only clears
  changing and notifies. The framelog counters (Remaps, RemapWaitUs,
  RemapUs) kept their places.
- refresh_timed pins [start, start + size) with saturating_add (checked_add
  would refuse huge title sizes that writes() already tolerated); the
  source recompute (source_address + self.src.swap) and the
  mirrors::remapped call in title_memory_remapped are untouched.
- REMAPPED queue: ported DexNom's record_remap (adjacent/overlapping
  coalesce, full queue -> one (0, u64::MAX) range) and merge_remaps in
  texture_slots_remapped. STALE[2] now counts fallbacks (was: ranges
  dropped); cap kept at 1 << 15 (DexNom raised it to 1 << 18; not asked,
  left to T9).
- watch.rs mapping tests serialized with a module-level ALONE mutex: they
  share global state and would time out each other in parallel.

### Dead ends and findings

- None blocking. The wall is not cut by the gpu-side alone: xmem still
  calls the un-ranged `title_memory_remapping()`, which announces
  ALL_MEMORY, so every refresh still waits for every remap. The win needs
  xmem to pass its extent (DexNom's physical.unmap_extent(base, end)),
  which is out of scope here; per the stop rule I did not touch xmem.

### Commands run

- cargo check -p victory-gpu: clean.
- cargo clippy -p victory-gpu --all-targets -- -D warnings: exit 0.
- cargo test -p victory-gpu: 188 passed, 0 failed, 5 ignored; exit 0
  (WARP tests included; 12 extra runs of watch::mapping_tests, 0 fails).

### Open points

- The measured wall (1,397.7 ms, baseline-02) only drops when xmem passes
  its remap extent to title_memory_remapping_range; needs a change in
  crates/victory-hle/src/modules/xmem.rs + a lib.rs export, both out of
  T8's scope. T15 does this.
- REMAPPED cap still 1 << 15; DexNom uses 1 << 18 with the same fallback.
  Raising it is T9's call.
- "A remap waits only for readers that overlap the ranges it changes" is
  true for ranged callers; the un-ranged public API (xmem today) still
  waits for all readers.

## Review (hungerg-free, REJECT)

- The pins themselves are correct: no deadlock, missed wakeup or race
  found; the fallback queue is correct; Bink mirrors preserved; API for
  HLE unchanged; gate re-run green (188 passed, clippy clean).
- REJECT because the done line is not met in production: xmem still calls
  the whole-space form, so the ranged API is only exercised by tests.
- Lead decision: merge as infrastructure; T15 completes the xmem side.
