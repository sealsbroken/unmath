# Lesson 6: Firefly gates

**Skill:** Ordered counting through ten. **Prerequisites:** Count to five independently.
Prepare **10 recognizable toys**. Story props are optional; real food is not needed.

Fireflies hide pretend gates. Toys are glowing keys; your count opens each gate.

Example question: “The gate needs eight glowing keys! Count from one to eight to open it.”

## Directed cyclic question graph

```mermaid
flowchart TD
    Choose["Forest or moon adventure?"] --> Intro["Firefly gates: ordered counting through ten"]
    Intro --> Q["The gate needs eight glowing keys! Count from one to eight to open it."]
    Q -->|"Independent success"| Reward["Swing open the pretend gate and flutter like a firefly."]
    Reward --> Left{"Any of the six missions left?"}
    Left -->|"Yes: choose an unfinished mission"| Q
    Left -->|"No"| Done["Adventure complete"]
    Q -->|"Help, wrong answer, or skip"| Setup["Prepare n glowing toy keys"]
    Setup -->|"Required table total is stable"| Guided["Count this toy group aloud; zero is a spoken total"]
    Guided -->|"Last stage correct"| Explain["Explain the operation; keep this mission unfinished"]
    Explain --> Left
    Guided -->|"Incorrect: model and restart"| Setup
    Guided -->|"Table changed: cancel pending answer"| Setup
    Q -->|"Three misses, task limit, or break"| Rest["Rest and save progress"]
    Guided -->|"Three misses, task limit, or break"| Rest
    Guided -->|"Starts at zero while toys remain"| Zero["Zero means no toys; first toy gets one"]
    Zero --> Setup
    Q -->|"Starts at zero with keys"| Zero
```

Question nodes represent the current mission, so their numbers change as the
adventure advances. Return arrows are actual strategy transitions. Guided work
never marks a mission complete. After help, another unfinished mission comes
first when possible, and the helped mission returns later.

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
answer without scoring an error. A wrong guided count is modeled, then preparation
restarts. Setup itself is unscored. The camera verifies totals, not toy identities,
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
