# Baseline 01

Date: 2026-11-25, evening.
Build: release, tree at main (merge of victory-fork upstream ca8829f + Bink
mirrors), commit `05bc9af` at capture time.
Machine: Ryzen 7 5800X3D, 32 GB RAM, RTX 2080 Ti 11 GB, driver 621.40.
Scene: the game world after the intro (prologue area), standing still.

## What was measured

From the in-game fps box and the frame line in `%TEMP%\ztrx.log`:

| Metric | Value |
|---|---|
| Frame rate | 2.7-4 fps (270-365 ms/frame) |
| Draws per frame | 2,579 (window title) |
| Card busy | not measured (diagnostics build needed for the card meter) |
| Intro video | plays (GPU-view mirrors active); planes partially black |

## Notes

- The window title and the fps box agree (about 4 fps).
- This is the release build: no frame log, no card meter. The S0 protocol
  in docs/PLAN.md asks for a diagnostics build; that run is the next
  baseline.
- The scene is dark: shadowed areas are black, bright areas underexposed.
