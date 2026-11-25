# LEAD-GUIDE — running the team, session by session

The lead does not write code. The lead keeps state in files, dispatches writers, reads results at
the source, enforces circuit breakers, and takes all public actions. Reference: ORGA.md (protocol),
TEAM.md (project specifics), BOARD.md (tasks).

## 1. Session start — always

1. Read `BOARD.md` in full.
2. Read `activity-<cycle>.md` in full.
3. Read `decisions.md` in full.
4. Skim ORGA.md §13 and TEAM.md "Limits" and "Budget" — the numbers change.
5. Update `activity-<cycle>.md` if anything in it is stale. Then act.

Never trust your memory of a previous conversation. The files are the state.

## 2. Dispatch — a new task

1. **Dependency check** (ORGA §5): does the task touch files owned by another open task? Then it
   waits. Disjoint ownership → parallel.
2. **Write the brief** (`<task-id>.md`): goal, threads to read first, File Ownership Matrix,
   done line, gate command, stop rule, budget cap. Pick the model tier from TEAM.md.
3. **Human validation** (ORGA §4): Pedro validates the matrix and the done line before the
   writer starts. The one checkpoint that is not optional.
4. **Spawn the writer** in its own worktree — one per task. Cap: 3 writers; read-only runs
   (research, CI fetches, baselines) do not count.

## 3. Read-pass — when a result arrives

- Read the record, the gate output and the diff at the source. Not the summary.
- Circuit breakers (ORGA §6): gate fail ×3 → you decide; budget exceeded → no auto-retry;
  out-of-scope file touched → task voided; "done" without gate output → discard.
- Gate green → spawn the **reviewer**: fresh context, different model from the writer. One round.
- Reviewer REJECT → you decide: fix (new brief), escalate model (with the full record), or abort.
  Never a writer↔reviewer loop.

## 4. Merge and public actions — lead only

- Read the full PR/issue thread again before every public action. Someone may have fixed it
  since the brief was written.
- Merge only on gate green + reviewer APPROVE. After the merge: remaining open branches rebase
  onto main (ORGA §5 R5), BOARD.md updated, activity file appended.
- Commit trailers: one `Assisted-by: <model> (<tool>, <machine>): <what>` per agent. Credit each
  person who contributed, by name. No `Co-Authored-By` for agents.
- PR "Tested" section: one line of machine specs, then the test summary lines. Short.

## 5. Cycle end

1. Move merged/closed task files to `archive/`.
2. Rewrite `BOARD.md` for the next cycle.
3. Rename the activity file to the next cycle; the old one stays as the journal.
4. Commit atomic, logical commits (README "Repo hygiene").
5. Report the cycle: shipped, blocked, waiting on Pedro.

## 6. Handoff — when the lead session dies

If the lead session restarts mid-cycle: start at §1. The incoming lead reads the files, never the
old conversation. The files are the database (ORGA §7 "Lead handoff").
