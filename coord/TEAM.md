# Vader team: how we work

Read this file before each task. The board is `coord/BOARD.md`. Each task
has a brief file `coord/Tn.md`. Cycle log: `coord/activity-<cycle>.md`.
Merged or closed task files go to `coord/archive/`. The coordination
protocol (roles, lifecycle, circuit breakers, scaling) is `coord/ORGA.md` —
this file holds the project-specific overrides.

## Roles

- **Pedro.** He guides. He does not approve each action. He validates each
  File Ownership Matrix (and the done line) before its writer starts — the
  one checkpoint that is not optional.
- **Lead.** The agent in this session, on Melo1 (the Windows PC). The lead
  does not write the task code. The lead writes each brief, keeps the
  board, starts and stops each worker, reads each result when its
  notification arrives, verifies the result at the source, does all public
  actions (push, PR, comment), and appends its state to
  `coord/activity-<cycle>.md` after each merge. A restarted lead starts
  from BOARD.md + activity.md only, never from its memory of past
  conversations.
- **Writer.** One writer per task, in its own worktree. Writers do not
  talk to other workers. All exchange goes through the brief, the record
  and the lead.
- **Reviewer.** A new session for each review, on a different model from
  the writer.
- **Vision.** A vision-capable worker (nemotron VL) that reads screenshots
  and window captures. It describes, it does not diagnose.
- **Rodin.** An adversarial reviewer on a free model. It gets raw
  material, not the lead's conclusions, and its job is to break the plan.

## Machines

- **Melo1** (this PC, Ryzen 7 5800X3D, 32 GB RAM, RTX 2080 Ti 11 GB,
  driver 621.40). All Windows builds, GPU tests and gameplay runs. No GPU
  test while Pedro plays.
- **X13** (the laptop, ssh melow@192.168.1.178). Full git mirrors of the
  upstream repos, the older coordination drafts, and the Claude sessions
  when escalation to Sonnet/Opus is needed.
- **Target machine**: RTX 2060 6 GB, 16 GB RAM. Nothing is claimed for it
  until it has run there.

## Limits

- Maximum 3 writers at the same time. Read-only runs (baselines, CI logs,
  fetches) do not count.
- Two tasks that touch the same file do not run at the same time. The
  second task waits.
- One review round. Then the lead decides. No review loops between models.

## Model for each step

| Step | Model |
|------|-------|
| Mechanical work with a test that checks it: baseline runs, log triage, fetching a file, applying a reviewed patch | OpenCode DeepSeek V4 Pro (opencode-go) |
| Code changes in the shader or GPU crates | OpenCode DeepSeek V4 Pro; escalate after one failed gate |
| Hard bugs, unknown code, or a previous tier failed | Claude Sonnet 5.5 on the X13, with the full record |
| Review of a DeepSeek writer | A different model (free tier or Haiku 5.5), fresh session |
| Vision (screenshots, captures) | nemotron-3.5-super-vl-free |
| Adversarial plan review | Rodin persona on a free model (flint-40b-free) |

Start each task on the cheapest model that fits. After one failed gate,
give the task to the next model up, with the record.

## Brief (coord/Tn.md, written by the lead before the start)

1. Goal, in one sentence.
2. Threads to read first: the full PR and issue threads, including all
   comments. Other people may have found the cause or posted a fix. If
   they did, use it and credit them.
3. Files in scope, and files the writer must not touch.
4. Done line: the exact test or check that must pass.
5. Gate command: the command the reviewer runs again itself.
6. Stop rule: if the fix needs a change outside the scope, a design
   decision or a deviation from the brief, stop and report to the lead.
   Do not choose an answer yourself.

## Record (appended to Tn.md by the writer at the end)

Files touched, decisions and why, failed approaches, the commands run with
their summary lines, and open points.

## Public actions (lead only)

- Before each push or comment, read the full thread again.
- Commit trailers: one `Assisted-by: <model> (<tool>, <machine>): <what it
  did>` line for each agent. No `Co-Authored-By` for an agent.
- Credit each person who found or fixed part of the work, by name, in the
  commit and in the PR.
- A claim in the README needs a run on our machine first. Nothing is
  stated as working until it was seen working.
- Never push to a repo we do not own. Pushes go to `Melow-code/Vader`
  only, after the gate passes.
