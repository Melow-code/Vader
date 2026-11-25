# SOURCES — evidence and justifications for ORGA.md

Annex of ORGA.md. The protocol file stays under ~200 lines; every claim carries its source here.
Verification wave: 2026-11-25, four opencode workers, one per group. Report:
/tmp/claude-1000/verify/results.md.

## Verified papers

- **arXiv:2512.08296** — Google DeepMind, "Towards a Science of Scaling Agent Systems"
  (Dec 2025: 180 configurations, 5 architectures, 4 benchmarks; v3 Apr 2026: 260 configurations,
  6 benchmarks). Error taxonomy (v1): specification, inter-agent misalignment, verification
  failures. Sequential/constraint-satisfaction tasks: multi-agent degrades −39% to −70%
  (PlanCraft). Error amplification ×17.2 for independent agents vs ×4.4 centralized.
  VERIFIED 2026-11-25.
- **arXiv:2608.16801** — Destefanis & Aste, "When Agents Coordinate". 1,902 runs; shared files cut
  output tokens by ~42% vs. direct messaging at 8 agents (36–49% across sessions).
  VERIFIED 2026-11-25.
- **arXiv:2606.00953** — Yang et al., "When Parallelism Pays Off: Cohesion-Aware Task Partitioning
  for Multi-Agent Coding". Naive file-based parallelism: +60% cost for +3.2% pass rate vs
  sequential ("can offset" the gains, not "always offsets"). Static-analysis dependency graphs +
  community detection: −52% latency, −35% cost vs sequential, 2.10× speedup (CodeProjectEval).
  VERIFIED 2026-11-25. (Not "Co-Coder" — that name is not the paper's.)
- **arXiv:2610.30074** — Li, Demidovich & Richtárik, "Parallel AI Agent Collaboration as
  Graph-Structured Distributed Block Optimization". "Parallel AI agents can produce useful local
  edits yet degrade a shared solution when their edits are combined." Optimal fractional coloring
  yields a merger preserving 1/χ_f(G) of verified local improvement. VERIFIED 2026-11-25.
- **arXiv:2606.15376** — "CoAgent: Concurrency Control for Multi-Agent Systems". MTPO (Monotonic
  Trajectory Pre-Order) protocol; stays within 5% of serial correctness at 1.4× speedup.
  VERIFIED 2026-11-25.

## Industry / vendor

- **Cognition, 2026-04-22, "Multi-agents working"** (cognition.com/blog/multi-agents-working).
  Devin Review catches ~2 bugs/PR, ~58% severe; "multi-agent systems work best today when writes
  stay single-threaded and the additional agents contribute intelligence rather than actions";
  swarms "mostly a distraction". The 127→53 step figure is NOT in this post — it is in the
  **SWE-2 launch post, 2026-09-10** (cognition.com/blog/swe-2): SWE-1.7 127 → SWE-2 medium 53
  (high 80, max 98) on FrontierCode 1.1 Main. VERIFIED 2026-11-25.
- **Claude Code agent-teams docs** (code.claude.com/docs/en/agent-teams): "Start with 3-5
  teammates for most workflows." VERIFIED 2026-11-25.
- **Claude Code memory docs** (code.claude.com/docs/en/memory): "target under 200 lines per
  CLAUDE.md file." The "ignores half" phrasing comes from the best-practices doc (over-specified
  CLAUDE.md). VERIFIED 2026-11-25.
- **Microsoft AutoGen**, 2025-10-02: maintenance mode announced ("no significant new features"),
  merged into Microsoft Agent Framework
  (github.com/microsoft/autogen/discussions/7066; venturebeat.com). "DRACO" has no relation —
  removed from earlier draft. VERIFIED 2026-11-25.
- **Anthropic multi-agent docs** (claude.com/resources/articles/building-multi-agent-systems-when-and-how-to-use-them;
  multi-agent-coordination-patterns): context pollution across subtasks; evaluator loops must be
  bounded; reactive loops over shared state "cycle indefinitely or stop arbitrarily when one
  agent's context fills". No "unbounded mesh" framing — reworded in §9. VERIFIED 2026-11-25.

## Frameworks

- **CrewAI**: `max_iter` default 25 in source (base_agent.py); docs table inconsistently says 20.
  Manager-LLM routing in hierarchical mode (docs.crewai.com, processes). Token explosion:
  agilesoftlabs.com "CrewAI in production" (2026). SQLite "database is locked" failures: crewAI
  GitHub issues (e.g. crewAI-examples#191) — NOT in agilesoftlabs/towardsai. VERIFIED 2026-11-25.
- **Gas Town** (yegge.ai/essays/welcome-to-gas-town): Mayor (main agent), Refinery (merge
  engineer), Witness (un-stuck watcher); Beads = git-backed persistence, "all state lives in
  Beads" (cloudnativenow.com, dolthub.com). VERIFIED 2026-11-25.
- **OpenAI Swarm README** (github.com/openai/swarm): states non-goals in prose (educational, not
  for production, no state between calls) — no dedicated section. VERIFIED 2026-11-25.
- **LangGraph** (docs.langchain.com): checkpointers persist state per step (human-in-the-loop,
  fault tolerance); per-key reducer functions control state updates. VERIFIED 2026-11-25.

## Practitioner terrain

- **Writer counts**: r/ClaudeAI 1wju273 ("hard cap of 3", "reviewing the output is the actual
  bottleneck"), 1wnanuq ("the limit is 3-4"), 1rubrca ("2-3 genuinely isolated tasks"); worktrees:
  1qzduim, 1w8i74q. Convergence: 2–4 writers, 3 modal. VERIFIED 2026-11-25.
- **Parallel heavy builds froze a server**: dev.to, alex_iolan, 2025-10-29 — six agents × 1.5–3 GB
  builds = 9–18 GB, swap thrash, hard reboot. (Earlier draft cited "15 GB+ (r/ClaudeCode 2025)" —
  wrong venue.) VERIFIED 2026-11-25.
- **Shared dev database across worktrees**: joxo.ai "one database per git worktree" (2026),
  maverickdotdev — migrations from one branch break the other's tests. (Earlier draft cited
  r/ClaudeCode — not found there.) VERIFIED 2026-11-25.
- **Cost trap**: r/LocalLLaMA 1vmrbju (2026) — "It only pays when something other than the big
  model can check the small one. Tests passing, a schema validating, a diff applying."
  stackness.dev (2026-09-25): routed work cut Astra usage 98%, cost per 1k lines $11.32 → ~$0.30;
  "a cheap model that needs three attempts is not cheap". VERIFIED 2026-11-25.
- **Handoff / dead ends**: vilix.ai (2026) — dead-ends list with reasons ("tried X, failed
  because Y"). alexlavaee.me (2026) — delegation contract with a verification field; "if you can't
  fill in the verification field, the subtask isn't granular enough yet". mindstudio.ai (2026,
  WHISK) — task brief before each session, HANDOFF.md, compact after a milestone. The dev.to
  handoff-card article (2025) is deleted/404 — dropped. VERIFIED 2026-11-25.
- **Creator/verifier**: hackersandwizards.dev (2026, updated 2026-11-07) — second agent with its
  own context judges; "independence has to reach the real-world signal". VERIFIED 2026-11-25.
- **One head, many hands**: evozon.com (2026-10-21) — single deterministic verify command closes
  every agent loop; the "failed within a week / two people cannot hold one theory of a system"
  lines are an explicitly hypothetical scenario, not a case study — cited with caution.
  VERIFIED 2026-11-25.

## Dropped from earlier drafts

- "hallucinated coordination" (Cognition): term does not appear in the source — removed.
- santageai.com synthesis: numbers correct but secondary (×17.2 = Independent topology) — replaced
  by the primary paper.
- towardsai.net CrewAI article: does not exist — replaced.
- DRACO: no relation to AutoGen — removed.
- "~100 lines" instruction-file limit: Anthropic's number is 200 — corrected.
