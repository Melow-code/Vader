# ORGA — Agentic Team Design

> Version 3 — claims verified 2026-11-25; maintenance rules added.
> Harness-agnostic. Model-agnostic. Orchestration-tool-agnostic.
> The only substrate required: **files** and **git**.
> Size rule: this file stays under ~200 lines (§11). Evidence, justifications and examples live
> in SOURCES.md and EXAMPLES.md.

## Why this exists

Multi-agent coding setups fail from **coordination**, not model quality. The DeepMind scaling study
categorizes the observed errors as specification failures, inter-agent misalignment, and
verification failures (arXiv:2512.08296). This document defines the coordination protocol. It
names no tool, harness, or model.

## 1. The communication substrate

All coordination happens through **files and git**. No agent-to-agent messaging, no shared mutable
memory, no message bus.

```
<coord-dir>/            project-configured; default: repo root
  BOARD.md              task table, rewritten every cycle
  activity-<cycle>.md   gate results and lead decisions, current cycle only
  decisions.md          architecture decisions, append-only
  <task-id>.md          brief + record for one task
  archive/<task-id>.md  merged or closed tasks
  ORGA.md               this file
  TEAM.md               project-specific overrides
```

Why files: shared files cut output tokens by ~42% vs. 1-to-1 messaging at 8-agent scale
(arXiv:2608.16801), they survive crashes and rate limits, and git provides the audit trail and the
merge engine. Files are not an event bus: the lead reads when it checks, not on notification.
Batch coordination is a feature — it prevents ping-pong loops.

## 2. Roles

| Role | Reads | Writes | Does NOT |
|------|-------|--------|----------|
| **Lead** | everything | briefs, BOARD.md, activity.md, public git actions | write code; self-review; run more than 3 writers at once |
| **Writer** | its own brief, the files it owns | code, tests, its record in the brief | talk to other writers; touch out-of-scope files; decide outside brief scope |
| **Reviewer** | brief, diff, gate command | verdict + rationale appended to the brief | trust the writer's numbers; reuse the writer's context |
| **Gate** | the worktree | pass/fail + raw output | interpret; decide; approve |
| **Merger** | gate result + reviewer verdict | the merge commit + activity.md entry | merge without gate passing or reviewer approval |

The Lead and Merger can be the same entity.

## 3. Task lifecycle

```
Lead writes brief (task-id.md)
  → Writer starts in its worktree
  → Writer appends record, pushes branch / opens PR
  → Gate runs (tests, CI, linter)
  ├─ FAIL → writer gets record + raw gate output, one more attempt;
  │         after 3 failures: lead decides (abort / escalate model / change scope)
  └─ PASS → Reviewer starts (fresh context, different model), runs the gate independently
            ├─ REJECT → findings appended to brief; lead decides; one review round, no ping-pong
            └─ APPROVE → Merger merges; remaining branches rebase onto main;
                         lead updates BOARD.md and activity.md
```

Brief (written by the lead before the writer starts): 1. **Goal**, one sentence. 2. **Read
first** — full PR/issue threads, all comments, linked. 3. **File Ownership Matrix** — owned files,
forbidden files. 4. **Done line** — exact command and expected output. 5. **Gate command** —
runnable from a fresh checkout. 6. **Stop rule** — scope change, design decision, or file outside
the matrix: note and stop. 7. **Budget cap** — max tokens/attempts per escalation tier; exceeding
it is a circuit breaker (§6).

Record (appended by the writer): files touched (paths, not summaries); decisions and why; **dead
ends** (mandatory — what was tried and why it failed); commands run with raw one-line output; open
points.

## 4. File Ownership Matrix

Declared in the brief before the writer starts. **Human-validated before any writer starts** —
this document forbids delegating decomposition (§10), so the entity that wrote the matrix does not
approve it alone. On small teams this is a glance. A writer that finds a bug outside its matrix
appends a note and stops; the lead decides.

Why non-negotiable: naive file-based parallelism cost +60% for only +3.2% pass rate versus
sequential; cohesion-aware partitioning gives 2.10× (arXiv:2606.00953, details in SOURCES.md).
Two writers touching one file is a correctness problem, not a workflow problem.

## 5. Parallelism rules

**R1 — Structure, not team size.** Map dependencies first. Shared files or interfaces run
sequentially; disjoint ownership runs in parallel. The optimal count is the maximum independent
set of the dependency graph.

**R2 — Cap: 3 concurrent writers.** Claude Code docs say start with 3–5 teammates; Cognition's
post-mortem says writes stay single-threaded; practitioners converge on 2–4 with 3 as the modal
hard cap. The bottleneck is review bandwidth, not generation.

**R3 — Read-only tasks do not count.** Reviewers, baseline runners, CI fetches, research.

**R4 — Shared infrastructure counts.** At most 2 heavy builds at a time (dev.to 2025-10-29
post-mortem: 9–18 GB, server froze). The brief specifies hardware constraints.

**R5 — Rebase after every merge.** Parallel agents produce good local edits that fail when
combined (arXiv:2610.30074). Rebase keeps conflicts syntactic. Never fan in branches at once.

## 6. Circuit breakers

| Trigger | Action |
|---------|--------|
| Gate fails 3× on a task | Writer stops. Lead decides: abort, escalate, re-scope. |
| Budget cap exceeded | Writer stops. No automatic retry. Lead decides: abort, reduce scope, escalate with explicit new budget. |
| Reviewer rejects (1 round) | Lead decides. No second review round. |
| Out-of-scope file touched | Task voided. Lead re-scopes and re-assigns. |
| No commit in N minutes | Lead checks; stuck writer appends a blocked note and stops. |
| "Done" without gate passing | Discarded. Gate output is the only completion signal. |

Enforced by the lead. An agent cannot waive its own breaker.

## 7. Handoff

No context is passed between sessions. The outgoing writer writes to the record before stopping:
last commit, **dead ends** (the most expensive omission), file pointers with exact paths and line
ranges, next atomic action. The incoming agent reads the full brief, the full record, and the
actual files — never a summary, never a question to the outgoing agent.

**Lead handoff.** The lead's state lives in activity-<cycle>.md. After each merge or read-pass,
the lead appends what it decided, what it waits on, what it has not read. A restarted lead reads
BOARD.md + activity-<cycle>.md and does not trust its own memory. Conversation is a cache for the
lead too — at 4–10 writers this breaks before the super-lead of §12 is ever needed.

## 8. Model routing

| Task type | Tier |
|-----------|------|
| Mechanical, spec-bound | Cheap (DeepSeek, Haiku, local) — the gate catches errors |
| Judgment: multi-file, unfamiliar code | Mid (Sonnet-class) |
| Hard bugs, previous tier failed | Frontier — pass the failed tier's full record |
| Review | Different model from the writer, at least same tier |

Start cheap; escalate after one failed gate with the full record. The cost trap: a planner/executor
split only pays when a non-frontier verifier checks the cheap output (r/LocalLLaMA 1vmrbju;
stackness.dev 2026-09-25).

## 9. Topology

| Task structure | Topology | Do not use |
|----------------|----------|------------|
| Disjoint file ownership | Hub-and-spoke, parallel writers | Uncoordinated independent agents (error amplification ×17.2 vs ×4.4 centralized) |
| Strictly sequential | Pipeline (DAG) | Multi-agent (−39% to −70% on sequential tasks) |
| Parallelizable sub-trees, large scale | Hierarchical with sub-leads | Flat hub (context overflows) |
| Open-ended exploration | Bounded mesh (max hops + breaker) | Unbounded (context pollution, infinite loops) |

Default: hub-and-spoke. Upgrade only when task structure forces it.

## 10. What cannot be delegated

- **Task decomposition** — defines the matrix; must happen before any writer starts.
- **Architectural boundary decisions** — agent-made ones have no recorded rationale.
- **Done for non-automatable tasks** — if it cannot be a gate, it stays human.
- **Merge authority** — workers never self-approve.
- **Reading PR threads before public action** — someone may have fixed it since the last read.

## 11. Anti-patterns (confirmed in production)

| Anti-pattern | What breaks |
|--------------|-------------|
| Agents share a working directory | File clobbers; race conditions |
| Direct agent-to-agent messaging | No merge owner; ping-pong loops |
| Self-review, same model and context | Misses ~2 bugs/PR, 58% severe |
| Concurrent branch fan-in | Semantic conflicts |
| More than 3 parallel writers | Lead loses track; merge queue jams |
| Instruction file over ~200 lines | Attention dilution |
| LLM-driven routing | Non-deterministic; undebuggable |
| Trusting "done" without gate output | Plausible-but-brittle output |
| Unbounded retry loops | Token budget drain |
| Passing conversation history | Telephone-game degradation |
| Heavy builds in parallel | RAM spike, machine freeze |
| Shared dev database across worktrees | Phantom test failures |

Sources for each row: SOURCES.md.

## 12. Scaling

| Writers | Configuration |
|---------|--------------|
| 1 | Lead + writer + gate. Reviewer optional if the gate is comprehensive. |
| 2–3 | Full setup. Lead reads in gate-completion order. |
| 4–10 | Dedicated Merger; sequential merge queue; re-dispatch failures. |
| 10+ | Sub-teams with own leads; super-lead reads BOARD.md summaries only. |

## 13. Maintenance (the protocol protects itself)

- **Cycle rotation.** activity-<cycle>.md holds the active cycle only; git is the journal.
- **Archive merged tasks.** merged <task-id>.md → archive/.
- **BOARD.md is rewritten each cycle.**
- **Size discipline.** This file stays under ~200 lines (§11). Evidence, justifications and
  example matrices live in SOURCES.md and EXAMPLES.md — never here.
