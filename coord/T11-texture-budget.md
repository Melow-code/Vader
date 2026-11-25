# T11 — Texture upload budget (victory-perf port)

Brief, cycle 1. Writer: DeepSeek V4 Pro. Reviewer: hungerg-free, APPROVE.

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

## 4. Ownership matrix

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

---

## Record

Status: done. `crates/victory-gpu/src/draw.rs` only. Gate green,
reviewer APPROVE.

### Files touched

- `crates/victory-gpu/src/draw.rs` (only file):
  - `VICTORY_TEXTURE_STAGE_MB` default 192 -> 48.
  - `VICTORY_TEXTURE_STAGE_MS` default 250 -> 10.
  - Catch-up: `due.len() >= 1024` with `(per_frame * 3, span * 3)` replaced
    by victory-perf's flow: `census[0] > 2_000` (long queue), caps
    `64 << 20` bytes and 24 ms via `max`, gated by a new `catching_up`
    helper (`VICTORY_TEXTURE_GATE_S`, 60 s).

### Decisions and why

- Copied the catch-up shape from victory-perf's draw.rs: the x3 budget
  kept freezing frames as long as a backlog existed, which defeats the
  point of a budget. The gate caps the total catch-up time at 60 s and
  each pass at 64 MB / 24 ms.
- `max` instead of x3: an env override above the cap still wins.
- Kept `VICTORY_TEXTURE_CATCHUP` and added `VICTORY_TEXTURE_GATE_S`
  (0 disables the gate). Kept our newer logic untouched: staging pool,
  full-card handling, the recheck/tiles paths.
- The commits 4cb867b / a7b2fb8 are not in the shallow victory-perf clone;
  worked from the files as they are.
- Kept the historical run notes that mention 192 MB: they document why
  the budgets exist.

### Dead ends

- First attempt committed the whole diff as one commit before splitting;
  reset (mixed) and redid as two logical commits.

### Commands run

- `cargo test -p victory-gpu` -> 185 passed, 0 failed, 5 ignored.
- `cargo clippy -p victory-gpu --all-targets -- -D warnings` -> clean.
- Commits: `61279ce` (defaults 48 MB / 10 ms), `a310329` (bounded
  catch-up 64 MB / 24 ms), each with the Assisted-by trailer.

### Open points

- Runtime behavior not measured (no game run allowed here): the 48 MB /
  10 ms values are victory-perf's, ours is a port, not a new measurement.
