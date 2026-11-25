# T11 — Texture upload budget (victory-perf port)

Brief, cycle 1. Writer: to be assigned (DeepSeek V4 Pro). Reviewer: a
different model, fresh session.

## 1. Goal

Lower the heap-texture upload budget from 192 MB / 250 ms (catch-up
576 MB / 750 ms) to 48 MB / 10 ms (catch-up 64 MB / 24 ms), as
victory-perf measured, so a streaming frame no longer spends most of its
time uploading textures while the card waits.

## 2. Read first

- `bench/baseline-02.md` (the world frame breakdown).
- `coord/archive/T10-victory-perf-diff.md` (rank 1).
- victory-perf's implementation:
  `C:\Users\Tugakit\Desktop\Nouveau dossier\victory-perf\crates\victory-gpu\src\draw.rs`
  lines 1979-2065, and its revisions in commits 4cb867b, a7b2fb8.
- Our current budget code:
  `D:\ztrx-repo\ztrx\crates\victory-gpu\src\draw.rs` lines 1852-1858 and
  1929-1942.

## 3. Files in scope

- `crates/victory-gpu/src/draw.rs` only.

Forbidden: everything else. T8 owns watch.rs/texture.rs/descriptor/host.rs;
do not touch them.

## 4. Ownership matrix (to validate)

| Files | Writer |
|---|---|
| crates/victory-gpu/src/draw.rs | T11 writer (sole owner) |
| watch.rs, texture.rs, descriptor/host.rs | T8 writer (do not touch) |
| crates/victory-shader/** | T2 writer (do not touch) |

## 5. Done line

- The budget constants match victory-perf's final values (48 MB / 10 ms,
  catch-up 64 MB / 24 ms), behind the same kind of env override if they
  have one.
- `cargo test -p victory-gpu` passes and
  `cargo clippy -p victory-gpu --all-targets -- -D warnings` is clean.

## 6. Gate command

```
cargo test -p victory-gpu && cargo clippy -p victory-gpu --all-targets -- -D warnings
```

## 7. Stop rule

If the budget interacts with heap_buffers or tiles.rs in a way that
needs their files, stop and report. Do not choose an answer yourself.
