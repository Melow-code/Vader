# Decisions

Append-only. Date, decision, why.

- **2026-11-25 · D1 · Base = ztrx.** The NVIDIA-tuned line, the most
  stable public build, already runs the prologue on our machine. AMD forks
  look better but are tuned for Radeon; their fixes are portable and get
  ported, the base stays NVIDIA.
- **2026-11-25 · D2 · Merge fxceless/victory-fork upstream main** (13
  commits: descriptor sync fingerprint, card buffers, indirect gather,
  depth bounds, GPU-aware shader cache). Merged in one commit; the tree
  builds, clippy is clean, tests pass.
- **2026-11-25 · D3 · Bink video planes via GPU-view mirrors**, ported
  from xcl's kernelx mirrors (`crates/victory-gpu/src/mirrors.rs`). The
  intro video now plays.
- **2026-11-25 · D4 · Repo = Melow-code/Vader.** Renamed from
  project-americas. Branch protection on main: 2 approving reviews for
  non-admins, no force-push, no deletions, admin bypass on (solo flow).
  Security alerts and automated fixes on.
- **2026-11-25 · D5 · Linear sync.** GitHub Issues ↔ team MEL, two-way.
  The S0-S4 issues live on both sides. Dependabot watches cargo weekly.
- **2026-11-25 · D6 · Model routing.** Writers start on DeepSeek V4 Pro
  (opencode-go), escalate after one failed gate; reviews on a different
  model; vision on nemotron VL; Rodin (adversarial plan review) on a free
  model. See coord/TEAM.md.
- **2026-11-25 · D7 · No unverified claims.** The README separates what
  was seen on our machine from upstream reports. Nothing is marked done
  without a run on Melo1.
- **2026-11-25 · D8 · Measure before porting.** Every frame-rate change is
  validated against the baseline on a fixed scene (docs/PLAN.md), one
  switch at a time.
