# Plan

## Objective

Run the Xbox Series build of GTA VI on a mid-range PC at a stable frame
rate, with correct rendering and no crashes in long sessions.

Target hardware: RTX 2060 (6 GB VRAM), 16 GB RAM.
Development hardware: RTX 2080 Ti (11 GB VRAM), 32 GB RAM.

## Reality check

The best public builds render the open world close to the retail game at
3 to 5 fps on an RX 9070 XT or an RTX 4070 (PortWatch VI, 2026-11-25).
This tree runs the prologue at 1 to 3 fps on an RTX 3050.

30 fps on the target hardware is about 10 times the current state of the
art. The plan does not promise it. It sets measurable steps toward it and
decides each step by measurement, not by opinion.

## KPIs

| KPI | Meaning | Gate |
|---|---|---|
| Frame time p95 | 95 % of frames are this fast or faster | below 33 ms = 30 fps |
| Frame time p50 | median frame time | below 16 ms = 60 fps |
| Card busy time | timestamp pairs around each host list | at least 90 % of frame time |
| Submit thread time | time in the layer's submit path | tracked per frame |
| JIT translation rate | shaders translated during play vs from cache | zero on the play path |
| VRAM high-water mark | process video-memory budget used | below 6 GB on target |
| Lost texture notifications | ranges dropped by the 32,768-entry queue | zero per frame |

## Method

Port nothing blind. Every work stream follows this loop:

1. Capture a baseline on the development machine.
2. Change one thing.
3. Measure the same scene again.
4. Keep the change only if it passes the gate.

### Baseline protocol

Use a fixed scene (the first room after the prologue) and a diagnostics
build with:

```
VICTORY_FRAME_LOG=1
VICTORY_CARD_METER=1
VICTORY_CAPTURE_WORLD_EVERY=64
```

Record: frame time p50/p95/p99, card busy time, submit thread time,
copies, copy-back, texture work, faults, disk reads. One file per run, in
`bench/`.

### Exit criteria

Each work stream below has an exit criterion. A stream is not "done"
until its criterion is met on the development machine. Streams without a
criterion are not started.

## Work streams

### S0. Baseline and harness

Measure the merged tree before any new port. This is the reference every
later number compares against.

Exit: `bench/baseline-01.md` with the numbers above.

### S1. Correctness

Port the fixes that make the picture and the run correct.

- [x] Bink video planes: read GPU-view textures through their CPU view
      (`crates/victory-gpu/src/mirrors.rs`). Done, pending an in-game check.
- [ ] Shader lowering fixes from Melow-code's pull requests:
      v_mad_i64_i32 sign-extended literal, 16-bit and packed neg
      modifiers, ds_permute_b32 scatter, shader cache keyed on the host
      max wave.
- [ ] Descriptor sync: mirror raw heap writes with a cheap fingerprint
      (upstream victory-fork).
- [ ] Integer render-target blending, depth bounds, mesh PSO topology
      (upstream and DexNom).

Exit: zero shader-related device losses in 90 minutes; the intro video
plays with the picture; the prologue renders without dropped draws that
have a translated shader.

### S2. Frame rate

Take the merged upstream frame-rate work and verify each switch against
the baseline, one at a time. Candidates: `VICTORY_RELAY=2`,
`VICTORY_LATE_WAIT=3`, `VICTORY_WIDE_BACK=1`, `VICTORY_INDIRECT_GATHER=1`,
`VICTORY_LEVELS_ON_DEMAND=1`, `VICTORY_ASYNC_SUBMIT=1`, `VICTORY_QUEUE2=1`.

Exit: frame time p95 of the fixed scene reduced by at least 20 % against
the baseline, with no picture regression.

### S3. Lighting

The ray-traced lighting does not run yet; shadows stay black. Options,
in order of cost:

1. Belobug's ambient stand-in (constant tint after the temporal resolve)
   behind a switch. This is a picture hack, not correctness.
2. DexNom's ray traversal and builder work (software ray tests in DXIL).

Exit: the fixed scene renders without fully black shadowed areas, with
the change off by default.

### S4. VRAM envelope for 6 GB

The development card has 11 GB. The target has 6 GB. Every memory change
must be checked against the target budget:

- card buffers capped below the 1 GiB upstream default,
- `VICTORY_LEVELS_ON_DEMAND=1` tested on NVIDIA,
- streamed-mip shedding and idle eviction kept on,
- the pagefile guidance in the README followed on 16 GB machines.

Exit: VRAM high-water mark below 6 GB in a full prologue run on the
development machine, with at least 20 % headroom.

## Risks

- `VICTORY_WAVE_WIDE=1` lost the device on NVIDIA. Never enable it.
- `VICTORY_INDIRECT_WINDOW=1` has a history of device losses. Test only
  with `VICTORY_DRIVER_WATCH=1`.
- Several upstream switches are off by default because they measured
  neutral. Do not flip defaults without a baseline comparison.
- The light stand-in changes the picture. Keep it behind a switch and
  label it as a stand-in, not a fix.
- This repository hosts no game files, no executables, and no keys.
  Keep it that way; it is what keeps the project within its license and
  outside takedown scope.

## Definition of playable

The game is playable when, on the target hardware:

- frame time p95 is below 33 ms at the chosen render scale,
- no shader is translated during play (cache warm),
- the VRAM budget holds for a full session,
- a 90-minute session completes without a crash.

## Open questions

- No measurements exist for the RTX 2060. Find a test machine or add a
  telemetry run on one before S4 gates.
- Upscaling (render scale plus NVIDIA Image Scaling) is not evaluated.
  It may be the cheapest lever for the target card.
