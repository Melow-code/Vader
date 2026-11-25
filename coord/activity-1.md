# Activity — cycle 1

2026-11-25. Lead state, appended after each merge or read-pass. A
restarted lead reads this + BOARD.md, not its memory.

- Setup: ztrx clone fixed (incomplete extraction → correct repo), build
  green, launcher path bug fixed.
- Merged fork upstream main (D2). Clippy + tests green after fixing
  upstream's dead-code and a stale GPU test (budget charge order).
- Ported the Bink GPU-view mirrors (D3). Verified in game: the intro
  video plays. Credits planes partially black (texture format/detile
  issue, tracked, not yet tasked).
- Repo renamed to Vader, protection set (D4), Linear synced (D5),
  Dependabot on. Issues recreated after the sync was enabled: #7-#12.
- Research done: Belobug (ambient stand-in, triangle-line fix, clamp
  hoisting), DexNom (RT traversal, sparse textures, remap coalescing),
  VItoriaBr (NVIDIA fork with ambient pass; 70-file delta, port
  selectively), upstream fork commits (merged), xcl mirrors (ported),
  ScarlettPort (diff running).
- Baselines: release run ~3.3 fps in the world (282-304 ms/frame), GPU
  mostly idle by the runtime's own counter. Diag runs so far only reached
  the menus (33 ms frames, title's own pacing) because of the input
  issue; pad input confirmed working now (XInput device 0, keyboard
  fallback requires no pad connected).
- Input: pad polls fine (device 0, sequence advances). The keyboard
  stand-in is gated on "no pad connected" — Enter does nothing while the
  pad is plugged. Known behavior, not a bug.
- Waiting on: the ScarlettPort diff (T7), a diag run that reaches the
  world (T1), then T2/T4/T5.
- Next atomic actions: read the T7 diff when it lands; brief T2 (neg16
  via half_isrc, packed neg, mad_i64 arm + literal guard, ds_permute
  scatter); rebuild diag; run the world baseline with the pad working.
