# Lesson 5: Parcel detectives

**Skill:** Missing addends within five. **Prerequisites:** Addition within five; comparison helps.
Prepare **5 recognizable toys**. Story props are optional; real food is not needed.

Be a parcel detective: deliveries have a target size, but some parcels are missing.

Example question: “The delivery needs five parcels. We have two. How many more do we need?”

## Directed cyclic question graph

```mermaid
flowchart TD
    Choose["Forest or moon adventure?"] --> Intro["Parcel detectives: missing addends within five"]
    Intro --> Q["The delivery needs five parcels. We have two. How many more do we need?"]
    Q -->|"Independent success"| Reward["Stamp and deliver a pretend parcel to a toy friend."]
    Reward --> Left{"Any of the six missions left?"}
    Left -->|"Yes: choose an unfinished mission"| Q
    Left -->|"No"| Done["Adventure complete"]
    Q -->|"Help, wrong answer, or skip"| Setup["Prepare the starting parcels"]
    Setup -->|"Required table total is stable"| Guided["Count this toy group aloud; zero is a spoken total"]
    Guided -->|"Correct; more physical stages"| Next["Add one at a time until the target total"]
    Next --> SetupStage["Prepare the next required group"]
    SetupStage -->|"Stable total"| Guided
    Guided -->|"Last stage correct"| Explain["Explain the operation; keep this mission unfinished"]
    Explain --> Left
    Guided -->|"Incorrect: model and restart"| Setup
    Guided -->|"Table changed: cancel pending answer"| SetupStage
    Q -->|"Three misses, task limit, or break"| Rest["Rest and save progress"]
    Guided -->|"Three misses, task limit, or break"| Rest
    Guided -->|"Starts at zero while toys remain"| Zero["Zero means no toys; first toy gets one"]
    Zero --> Setup
```

Question nodes represent the current mission, so their numbers change as the
adventure advances. Return arrows are actual strategy transitions. Guided work
never marks a mission complete. After help, another unfinished mission comes
first when possible, and the helped mission returns later.

## Six missions and acceptance

1. 1 + ? = 3 → 2
2. 2 + ? = 5 → 3
3. 3 + ? = 4 → 1
4. 4 + ? = 5 → 1
5. 0 + ? = 2 → 2
6. 5 + ? = 5 → 0

Independently find the six missing additions, including starting empty and needing no extra parcels.

## Toy-based help

1. Prepare the starting parcels.
2. Add one at a time until the target total.
3. Count only NEW parcels while adding; recount the total.

Each camera-visible stage waits for its required stable total and a complete
spoken count (or zero for an empty group). A disturbed table cancels the pending
answer without scoring an error. A wrong guided count is modeled, then preparation
restarts. Setup itself is unscored. The camera verifies totals, not toy identities,
the meaning of the scene, or the child's pointing.

## Controls, progress, and integration

Strategy ID: `unmath-missing-to-five`. Use `Lesson::ParcelDetectives`
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
`unmath.lesson5.adventure.v1`; saved values keep completed missions, the next
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
