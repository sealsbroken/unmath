# Lesson 4: Bridge builders

**Skill:** More, fewer, and equal within five. **Prerequisites:** Count sets to five.
Prepare **5 recognizable toys**. Story props are optional; real food is not needed.

Friends need bridges. Choose the team with more supplies, fewer supplies, or the same supply.

Example question: “First team has two blocks. Second team has four. Which has MORE, or are they the same?”

## Acyclic teaching and correction paths

A wrong answer opens the teaching branch below automatically. Each box is a
child action or a smaller question; it is not a replay of the original stem.
The child keeps completed missions. A failed guided response stays at that
physical step. Once the groups are understood, later errors revisit only the
remaining concept. English and Russian use the same transitions.

The graph unfolds a correction episode into distinct forward nodes. `Continue`
is a boundary to the next forward correction episode at the indicated step,
not a jump to the first question. Further scored steps are bounded by the session
task budget; break, skip during setup, or repeated misses can pause. Replaying
a prompt and waiting for stable toys do not advance the teaching graph.
Guided work does not earn mastery: a fresh follow-up is required.

```mermaid
flowchart TD
    Choose["Which team has MORE, FEWER, or an EQUAL supply?"] -->|"Independent success"| Done["Keep progress; continue to next question or finish"]
    Choose -->|"Chooses the wrong side for fewer"| E0["Fewer is the row that runs out first when pairing"]
    E0 --> S0
    Choose -->|"Chooses a side when groups are equal"| E1["If every block has a partner, the amounts are equal"]
    E1 --> S0
    Choose -->|"Other wrong choice"| E2["Count and match the two supplies"]
    E2 --> S0
    S0["Prepare and count the FIRST team blocks"] -->|"Guided success; no mastery credit"| S1
    S0 -->|"Mistake at this step"| H0["Model the current settled count; one number per toy"]
    H0 --> R0["New guided attempt at this SAME step"]
    R0 -->|"Guided success"| S1
    R0 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    S1["Prepare and count the SECOND team blocks"] -->|"Guided success; no mastery credit"| S2
    S1 -->|"Mistake at this step"| H1["Model the current settled count; one number per toy"]
    H1 --> R1["New guided attempt at this SAME step"]
    R1 -->|"Guided success"| S2
    R1 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    S2["Pair the rows: WHICH team has blocks WITHOUT a partner?"] -->|"Guided success; no mastery credit"| S3
    S2 -->|"Mistake at this step"| H2["Make pairs again; look for blocks without partners"]
    H2 --> R2["New guided attempt at this SAME step"]
    R2 -->|"Guided success"| S3
    R2 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    S3["Do the unmatched blocks mean MORE, FEWER, or SAME?"] -->|"Guided success; no mastery credit"| Follow
    S3 -->|"Mistake at this step"| H3["Leftover blocks mean more; no leftovers in either row means equal"]
    H3 --> R3["New guided attempt at this SAME step"]
    R3 -->|"Guided success"| Follow
    R3 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    Follow["This bridge needs MORE or FEWER: which team should we choose?"] -->|"Independent success"| Done
    Follow -->|"Still needs help"| Focus["If the rows are known, revisit only pairing or more/fewer vocabulary"]
    Focus --> Continue
    Choose -->|"Break or budget reached"| Rest["Rest; save earned progress"]
    Continue -->|"Child requests rest, task limit, or three consecutive misses"| Rest
```

## Six missions and acceptance

1. 2 vs 4: more → second
2. 5 vs 3: fewer → second
3. 3 vs 3: more → same
4. 4 vs 1: more → first
5. 2 vs 5: fewer → first
6. 1 vs 1: fewer → same

Correctly choose first, second, or same on all six missions, including both more/fewer questions and equal groups.

## Toy-based help

1. Build and count the first supply.
2. Replace it with and count the second supply.
3. An adult can match groups one-to-one off-camera.

Each camera-visible stage waits for its required stable total and a complete
spoken count (or zero for an empty group). A disturbed table cancels the pending
answer without scoring an error. A wrong guided count is modeled, then that same physical stage is tried again. Setup itself is unscored. The camera verifies totals, not toy identities,
the meaning of the scene, or the child's pointing.

Comparison answers are `first`, `second`, or `same` (`первая`, `вторая`,
`поровну`). The first/second numbers are spoken in every
independent question. Guided supplies are counted one at a time; no app-generated
side-by-side drawing is assumed. Adult-assisted matching can happen off-camera.

## Controls, progress, and integration

Strategy ID: `unmath-compare-to-five`. Use `Lesson::BridgeBuilders`
with `Adventure::new`, or use the ID through `unmath::register` and the registry.
English and Russian content is included.

Choose forest or moon by saying its name. Say `help` / `помоги` during a question
for physical practice; this does not score an error. Say `stop`, `I'm tired`,
`стоп`, or `я устал` to rest, including during setup. Repeat restarts the current
prompt. Skipping a question records a skip and starts help; skipping setup pauses.

Three consecutive unsuccessful responses/timeouts, Stop, or `max_tasks` pauses.
Silence has no task record. Physical setup has no enforced timeout. A successful
uninterrupted adventure takes six tasks, within the default ten-task session.
Guided counts are additional tasks; pause and resume whenever needed.

Persist `SessionSummary.remember` using `apply_remembered` and pass it as the
next `SessionConfig.extra`. The isolated key prefix is
`unmath.lesson4.adventure.v1`; saved values keep completed missions, the next
mission, and forest/moon choice. Guided-stage progress restarts at an independent
question after resuming. Completion clears this lesson's keys.
`final_level` counts independently completed missions (six at completion), while
`correct` also includes guided counts. Insufficient `total_objects` pauses before
any maths task, explaining the required materials.

Success means all six specified challenges were solved independently. It is a
lesson completion check, not a claim of comprehensive fluency or delayed retention.
The story, toys, and reward actions are spoken invitations; no new engine animation
or unimplemented perception feature is assumed.

See the [edge-case audit](Edge-case-audit) for zero-starts, counting mutations,
interruptions, saved-state validation, and engine-boundary regressions.
