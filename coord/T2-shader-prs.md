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

---

## Record (Claude Sonnet 5.5, second writer, Melo1, 2026-11-26)

### Result

Two of the four fixes are delivered. Both `neg` fixes are left out.

| Fix | State | Commits |
|---|---|---|
| `v_mad_i64_i32` literal and lowering arm | delivered | df7458d |
| `ds_permute_b32` scatter | delivered | 0ff5205, e32346c |
| 16-bit `neg` (VIctory pull request 2374) | left out | none |
| packed `neg` (victory-fork pull request 890) | left out | none |
| tests moved out of `lower.rs` | needed by the policy | dff1506 |

The lead ruled on 2026-11-26: leave out a fix that the evidence does not
support, record why, and deliver the others.

The first writer's work is kept: branch `import/t2-writer` (a092871) and
the stash entry "T2: earlier writer uncommitted ds_permute work".

### Why the two `neg` fixes are left out

Both patches read `neg` on an integer source as a two's complement. We
found no evidence for that meaning. We ran no measurement of our own.

Packed integer operations (VOP3P), measured evidence against the patch:

- Pull request 4268 of the AnyPS5 project (owner boykopovar, merged
  2026-10-12) applies `neg_lo` and `neg_hi` as a flip of bit 15.
- Its author reports that the PS5 compiler emits
  `v_pk_max_i16 neg(literal 0x7fff)` to mean `max(-1, x)`. Only a bit
  flip turns 0x7fff into 0xffff.
- A comment by simonbbby reports an RDNA 2 card (gfx1036). The bit flip
  matched 512 of 512 results. A two's complement matched 47 of 512.
- A comment by thuanquang reports a second RDNA 2 card (gfx1031). The bit
  flip matched every row. A two's complement matched 0 to 46 percent.

16-bit VOP3 integer operations, no direct measurement:

- The machine-readable RDNA 2 ISA from AMD (release 2026-02-20) says that
  `NEG` takes the floating-point negation of the operand.
- Pull request 19592 of the same project reports a flip of bit 15 on an
  RDNA 2 card for 16-bit integer sources in DPP16 and SDWA forms. A comment
  there reports the same on an RDNA 4 card, also for the VOP3
  `v_ldexp_f16` exponent.
- That pull request states that its VOP3 16-bit integer ALU operations
  still refuse `neg`. We found no measurement for `v_add_nc_u16` and the
  other opcodes of the patch.
- The triage review on VIctory pull request 2374 asked for evidence and
  preferred the refusal. A comment by upipibi raised the bit-flip rule.

The brief names these two patches. A bit flip is a different fix, so the
writer did not choose it.

Decision for the lead:

1. Packed: port `neg` as a flip of bit 15. The measurements above support
   it. The change in `packed_src` is an XOR with 0x8000.
2. 16-bit VOP3: keep the refusal, or use the same flip by inference.

The GPU tests of the first writer expect two's complement values. A port
with a bit flip needs new expected values.

### Files touched

- crates/victory-shader/src/emit/lower.rs
- crates/victory-shader/src/emit/valu_tests.rs
- crates/victory-shader/src/emit/wave64_tests.rs
- coord/T2-shader-prs.md
- coord/BOARD.md

### Decisions and why

- `v_mad_i64_i32` had no lowering arm in our tree. The brief asks for it in
  the done line and also asks to decide with the lead. The first writer
  added it. It is kept, because the literal fix has no effect without it.
- The ISA data gives FMT_NUM_I64 for src2 of `v_mad_i64_i32`, also in the
  literal forms.
- The carry-out of `v_mad_i64_i32` is not lowered. A test shows that a read
  of it stops the program.
- `ds_permute_b32`: the first writer's scatter is kept with its scratch and
  barriers. The writer added a rule for collisions (e32346c). With the
  first design, lane 3 read the data of lane 0 on the RTX 2080 Ti. The
  expected value is the data of lane 31.
- Evidence for the collision rule: pull request 334 of the AnyPS5 project.
  Its author adriadam10 reports an RDNA 2 measurement (wave32): the highest
  sender wins, an unnamed active lane reads zero, inactive lanes neither
  send nor receive.
- The new lowering made `lower.rs` 367 bytes larger than the policy limit.
  The four new lowering tests moved to the two test files. `lower.rs` now
  has 2279 bytes of room (main had 9448).
- The cache key needs no change. `translation_fingerprint` includes a hash
  of the crate's sources and the host's widest wave.

### Dead ends and findings

- The first `v_mad_i64_i32` lowering test passed with the whole lowering
  arm removed. Nothing read its result, so nothing was lowered.
- The first writer's trailer claims both `neg` GPU tests. The 16-bit one
  matches the test that welisonzika14-jpg posted on pull request 2374.
- A plain atomic exchange also passes the collision test on the 2080 Ti.
  The test cannot tell it from the atomic maximum on this card. The maximum
  does not depend on the write order.

### Commands run

Windows 11, RTX 2080 Ti (driver 621.40), rustc 1.98.1,
`CARGO_TARGET_DIR` set to the T2 target folder.

- `cargo test -p victory-shader --locked` on main (2313bbd): 215 passed,
  0 failed, 3 ignored.
- The same command on dff1506: 222 passed, 0 failed, 3 ignored.
- `cargo clippy -p victory-shader --all-targets --locked -- -D warnings`
  on dff1506: no warning. The same with `--all-features`: no warning.
- `cargo test -p xtask --locked` on dff1506: 4 passed, 0 failed. On
  e32346c, `repository_passes_policy` failed on the size of `lower.rs`.
- Not run: the workspace test and Clippy steps of CI.

Each of the 7 new tests failed with the lowering of main, and passed with
the fixes. The scatter readback also failed with each of these removed:
the zeroing of the slot, the EXEC guard, the offset.

### Open points

- No title ran with this change. Nothing here is proven in game.
- No console measurement exists for any of the three delivered behaviors.
- 32-lane halves for a 64-lane console wave: we have no measurement for
  `ds_permute_b32`. Mesa states the limit for `ds_bpermute_b32` on GFX10.
  Our `ds_bpermute_b32` on a 64-lane host does not apply that limit.
- Group barriers: in a group of several console waves, a scalar branch
  around `ds_permute_b32` can leave a wave waiting. No test covers it.
- The wave read uses a lane index that differs between lanes, as the
  existing `ds_bpermute_b32` lowering does. It ran on the 2080 Ti only.
- `ds_permute_b32` GPU readbacks run on a hardware adapter only. CI has
  none, so CI runs only the lowering and DXIL validation steps.
- A possible later design: loop over the 32 source lanes with wave reads.
  It needs no scratch and no barrier. The AnyPS5 project uses it.
- On main, 16-bit integer compares and `v_cvt_f16_u16` / `v_cvt_f16_i16`
  read their source and ignore `neg` without a refusal.
- `lower.rs` is close to the size limit. The next fix there will need a
  split of the file or a new limit.
- The local branch `trial/7900xtx` also changes `lower.rs`. A test merge
  of this branch into it gave conflicts in `lower.rs` and
  `valu_tests.rs`. That branch raises the file limit to 640 KiB.
