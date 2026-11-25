# T2 — Port the shader lowering fixes from the pull requests

Brief, cycle 1. Writer: to be assigned (DeepSeek V4 Pro). Reviewer: a
different model, fresh session.

## 1. Goal

Port five shader fixes into `victory-shader`, each in its own commit, with
tests, so more console programs translate and the cache key is correct.

## 2. Read first

- The patch files in `C:\Users\Tugakit\Desktop\Nouveau dossier\vader\patches\`:
  vi-2380-mad_i64.patch, vi-2374-neg16.patch, fork-890-packed-neg.patch,
  fork-454-ds-permute.patch. (vi-2369-cache-wave and fork-379 are already
  covered upstream: the translation fingerprint includes the host max
  wave.)
- The full PR threads on GitHub, including all comments:
  Marvisak/VIctory #2380, #2374; fxceless/victory-fork #890, #454.
- The wave64-on-wave32 notes in
  `C:\Users\Tugakit\Desktop\Nouveau dossier\Belobug\docs\wave64-on-wave32.md`
  (ds_permute context).

## 3. Files in scope

- `crates/victory-shader/src/emit/lower.rs`
- `crates/victory-shader/src/emit/valu_tests.rs` (tests only)
- `crates/victory-shader/src/emit/wave64_tests.rs` (tests only, if needed)

Forbidden: everything else, especially the gpu/hle crates and Cargo.toml.

## 4. Ownership matrix (to validate)

| Files | Writer |
|---|---|
| lower.rs + the two test files above | T2 writer (sole owner this cycle) |
| everything else | untouched |

## 5. Done line

- `cargo test -p victory-shader` passes, including the new tests.
- `cargo clippy -p victory-shader --all-targets -- -D warnings` clean.
- Five commits, one per fix: mad_i64 literal sign-extension + lowering arm
  if missing, 16-bit neg modifiers via half_isrc (the VIctory #2374
  version, not fork #508), packed neg modifiers (#890), ds_permute_b32
  scatter (#454).

## 6. Gate command

```
cargo test -p victory-shader && cargo clippy -p victory-shader --all-targets -- -D warnings
```

## 7. Stop rule

If a fix needs a change outside the listed files, or a design decision
(mad_i64 lowering does not exist in our tree: check first, then decide
with the lead), stop and report. Do not choose an answer yourself.
