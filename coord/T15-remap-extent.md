# T15 — Pass the remap extent from xmem

Brief, cycle 1. Writer: to be assigned (DeepSeek V4 Pro). Reviewer: a
different model, fresh session.

## 1. Goal

Make `XMemMapPhysicalPages` and the unmap path tell the GPU side which
range of title memory actually changed, so a remap waits only for the
readers that overlap it. Without this, T8's range pins still stall on
every remap because xmem announces the whole address space.

## 2. Read first

- The T8 record (same folder, after it merges): the pins exist; only the
  extent is missing.
- `crates/victory-hle/src/modules/xmem.rs`: `XMemMapPhysicalPages`
  (`unmap` + `map`), `map_alias`; they all take
  `victory_gpu::title_memory_remapping()` today.
- `crates/victory-gpu/src/watch.rs`: the pin API T8 added (the writer's
  record names the exact new function, e.g.
  `title_memory_remapping_range`), and `lib.rs` exports.
- DexNom's equivalent: `C:\Users\Tugakit\Desktop\Nouveau dossier\DexNom\crates\victory-gpu\src\watch.rs`
  lines 734-929 (range-aware remap announcements).

## 3. Files in scope

- `crates/victory-hle/src/modules/xmem.rs`
- `crates/victory-gpu/src/lib.rs` (exports only)
- `crates/victory-gpu/src/watch.rs` (the new range API only)

Forbidden: everything else.

## 4. Ownership matrix (to validate)

| Files | Writer |
|---|---|
| the three files above | T15 writer (sole owner) |
| crates/victory-shader/** | T2 writer (do not touch, still running) |

## 5. Done line

- `title_memory_remapping()` gains an extent-aware form used by xmem:
  the unmap path passes `[base, base + len)` of what it unmaps, and the
  map path passes what it maps.
- No caller uses the whole-space form anymore (or it is removed).
- `cargo test --workspace` passes and
  `cargo clippy --workspace --all-targets -- -D warnings` is clean.

## 6. Gate command

```
cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings
```

## 7. Stop rule

If the pin API in watch.rs differs from what the T8 record describes,
stop and report. Do not choose an answer yourself.
