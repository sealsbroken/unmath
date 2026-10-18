# Lesson 6: Firefly gates

**Skill:** Ordered counting through ten. **Prerequisites:** Count to five independently.
Prepare **10 recognizable toys**. Story props are optional; real food is not needed.

Fireflies hide pretend gates. Toys are glowing keys; your count opens each gate.

Example question: “The gate needs eight glowing keys! Count from one to eight to open it.”

## Acyclic teaching and correction paths

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
    Choose["Count the firefly keys in order"] -->|"Independent success"| Done["Keep progress; continue to next question or finish"]
    Choose -->|"Starts at zero"| E0["First key gets one"]
    E0 --> S0
    Choose -->|"Skips or repeats a number"| E1["Give each key exactly one number"]
    E1 --> S0
    Choose -->|"Only final number or past the last key"| E2["Count every key once and stop at the last"]
    E2 --> S0
    S0["Prepare the requested real keys and count each one"] -->|"Guided success; no mastery credit"| Follow
    S0 -->|"Mistake at this step"| H0["Model the current settled count; one number per toy"]
    H0 --> R0["New guided attempt at this SAME step"]
    R0 -->|"Guided success"| Follow
    R0 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    Follow["Fresh full count of the prepared keys, starting at one"] -->|"Independent success"| Done
    Follow -->|"Still needs help"| Focus["Practice only this gate group, not the earlier gates"]
    Focus --> Continue
    Choose -->|"Break or budget reached"| Rest["Rest; save earned progress"]
    Continue -->|"Child requests rest, task limit, or three consecutive misses"| Rest
```

## Six missions and acceptance

1. Full sequence 1–6
2. Full sequence 1–7
3. Full sequence 1–8
4. Full sequence 1–9
5. Full sequence 1–10
6. A separate full sequence 1–10

Say every ordered sequence exactly, including two separate independent counts from one to ten. A final number alone never passes.

## Toy-based help

1. Prepare n glowing toy keys.
2. Point once per key and count aloud.
3. If unsure, model the count and return to gate missions.

Each camera-visible stage waits for its required stable total and a complete
spoken count (or zero for an empty group). A disturbed table cancels the pending
answer without scoring an error. A wrong guided count is modeled, then that same physical stage is tried again. Setup itself is unscored. The camera verifies totals, not toy identities,
the meaning of the scene, or the child's pointing.

Independent gates use counting mode: only the full sequence `1..n` passes.
A missing, repeated, or extra number, or saying only the final number, fails
that attempt. The last two gates each require a separate complete count to ten.
Guided counts never complete gates.

## Controls, progress, and integration

Strategy ID: `unmath-count-to-ten`. Use `Lesson::FireflyGates`
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
`unmath.lesson6.adventure.v1`; saved values keep completed missions, the next
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
