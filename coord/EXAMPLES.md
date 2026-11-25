# EXAMPLES — example matrices and briefs for ORGA.md

Annex of ORGA.md. Not rules — worked examples.

## Example File Ownership Matrix (from the gta6 project, cycle 1)

| Writer | Owns | Must not touch |
|--------|------|----------------|
| T12 writer | `src/shader/`, `tests/shader/` | everything else |
| T13 writer | `src/gpu/bvh/`, `tests/bvh/` | everything else |

Note: the lead wrote this matrix, Pedro validated it before the writers started (§4).

## Example brief skeleton (filled)

```markdown
# T12 — 3 upstream-compatible bpermute tests for PR #573

1. Goal: add three bpermute tests that reproduce upstream's semantics without console SDK code.
2. Read first: https://github.com/Marvisak/VIctory/pull/573 (all comments); T1-pr-bvh-tests.md
   for repo conventions.
3. File Ownership Matrix:
   | Owns | Must not touch |
   |------|----------------|
   | tests/bvh/ | everything else |
4. Done line: `cargo test -p victory-shader bpermute` passes on Linux AND Windows, 3 new tests
   green.
5. Gate command: same as done line, from a fresh checkout.
6. Stop rule: any change outside tests/bvh/ or any design choice the brief does not cover —
   note here, stop, wait for the lead.
7. Budget cap: 60k tokens, 2 attempts. Exceeded → stop (circuit breaker).
```

## Example record (appended by the writer to the same file)

```markdown
## Record

- Files touched: tests/bvh/bpermute_halves.rs (+214), tests/bvh/mod.rs (+3)
- Decisions: reuse the ds_bpermute_b32 lowering from T3 for the expected outputs (it matches
  upstream PR #573's semantics); no new helper crate.
- Dead ends: tried writing the tests against the wave64 path first — fails on Melo1 because the
  WARP device reports wave32; moved to the size-agnostic path.
- Commands:
  cargo test -p victory-shader bpermute → 176 passed, 2 known descriptor failures (same as T3)
  cargo clippy -p victory-shader --all-targets -- -D warnings -W clippy::pedantic → 0 warnings
- Open points: none.
```
