# Lesson 3: Cave train

**Skill:** Taking away within five. **Prerequisites:** Lessons 1–2: counting and combining within five.
Prepare **5 recognizable toys**. Story props are optional; real food is not needed.

You drive the train. Toy passengers get off at their caves; find how many still travel.

Example question: “There are five travelers. Three get off at the cave. How many are still traveling?”

## Acyclic teaching and correction paths

A child may repeat the whole correct answer. For two toys, `1, 2, 1, 2`
(or “один, два, один, два”) is one successful response. Each repetition must be
a complete `1..n` count, in order, ending at the actual toy total. Repetitions
never earn extra task completions or replace a later independent question.
Skipped numbers, incomplete restarts, and repeating one number inside a count
remain correction cases. Long valid repetitions are checked in full, then
compacted for bounded saved diagnostics; a late error is never discarded.

After an incorrect spoken answer, the next teaching question begins with
**“А ты уверен?”** in Russian (**“Are you sure?”** in English). The prefix is
spoken once, after any setup wait; skips and silence do not trigger it.

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
    Choose["How many travelers are still going?"] -->|"Independent success"| Done["Keep progress; continue to next question or finish"]
    Choose -->|"Counts the original group"| E0["Some travelers stopped; exclude them"]
    E0 --> S0
    Choose -->|"Counts the travelers who left"| E1["Separate stopped travelers from those still going"]
    E1 --> S0
    Choose -->|"Other wrong answer"| E2["Use real travelers and count only those still here"]
    E2 --> S0
    S0["Prepare and count the original travelers"] -->|"Guided success; no mastery credit"| S1
    S0 -->|"Mistake at this step"| H0["Model the current settled count; one number per toy"]
    H0 --> R0["New guided attempt at this SAME step"]
    R0 -->|"Guided success"| S1
    R0 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    S1["Move the specified travelers off; count those remaining, including zero"] -->|"Guided success; no mastery credit"| Follow
    S1 -->|"Mistake at this step"| H1["Model the current settled count; one number per toy"]
    H1 --> R1["New guided attempt at this SAME step"]
    R1 -->|"Guided success"| Follow
    R1 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    Follow["Look only at the remaining travelers: how many are going on?"] -->|"Independent success"| Done
    Follow -->|"Still needs help"| Focus["Revisit only the final remaining group; do not rebuild learned starting counts"]
    Focus --> Continue
    Choose -->|"Break or budget reached"| Rest["Rest; save earned progress"]
    Continue -->|"Child requests rest, task limit, or three consecutive misses"| Rest
```

## Six missions and acceptance

1. 3 - 1 = 2
2. 4 - 2 = 2
3. 5 - 1 = 4
4. 2 - 2 = 0
5. 5 - 3 = 2
6. 5 - 0 = 5

Independently find every remaining total in the six missions, including subtracting all and subtracting zero.

## Toy-based help

1. Line up a toy travelers.
2. Remove b travelers from the table.
3. Count who remains, including an empty train.

Each camera-visible stage waits for its required stable total and a complete
spoken count (or zero for an empty group). A disturbed table cancels the pending
answer without scoring an error. A wrong guided count is modeled, then that same physical stage is tried again. Setup itself is unscored. The camera verifies totals, not toy identities,
the meaning of the scene, or the child's pointing.

## Controls, progress, and integration

Strategy ID: `unmath-subtract-to-five`. Use `Lesson::CaveTrain`
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
`unmath.lesson3.adventure.v1`; saved values keep completed missions, the next
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
