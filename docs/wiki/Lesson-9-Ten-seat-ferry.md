# Lesson 9: Ten-seat ferry

**Skill:** Complements to ten. **Prerequisites:** Count to ten; missing addends within five.
Prepare **10 recognizable toys**. Story props are optional; real food is not needed.

Captain a ferry with ten pretend seats. Work out how many friends can still board.

Example question: “The ferry has ten seats and seven passengers. How many more can board to fill it?”

## Acyclic correction paths

Each scored question has a forward path: first attempt, a specific correction,
a fresh attempt, another correction, and a final attempt. A correct answer
advances to the next question; three consecutive misses pause. Completed work
is kept. A mistake never sends the child to the first question.

The diagram expands the ordinary answer/correction route into separate attempt
nodes, so it is acyclic. It omits table-waiting events, replay requests, and the
optional physical-help walkthrough. Help keeps the current mission; mistakes
during help retry the current physical stage. After help, the same mission gets
a fresh independent check. Guided work never earns independent mastery.
The task limit or a requested break may pause earlier than shown.

```mermaid
flowchart TD
    Choose["Choose forest or moon; prepare the current toys"] --> Q0A1
    Q0A1["Ten seats; 5 passengers. How many more fit?: attempt 1"] -->|"Correct"| Q1A1
    Q0A1 -->|"Mistake or skip"| Q0C1["Count empty seats; do not count passengers already aboard"]
    Q0C1 --> Q0A2["Same question: attempt 2"]
    Q0A2 -->|"Correct"| Q1A1
    Q0A2 -->|"Mistake or skip"| Q0C2["Repeat the local hint; keep this question"]
    Q0C2 --> Q0A3["Same question: attempt 3"]
    Q0A3 -->|"Correct"| Q1A1
    Q0A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q1A1["Ten seats; 7 passengers. How many more fit?: attempt 1"] -->|"Correct"| Q2A1
    Q1A1 -->|"Mistake or skip"| Q1C1["Count empty seats; do not count passengers already aboard"]
    Q1C1 --> Q1A2["Same question: attempt 2"]
    Q1A2 -->|"Correct"| Q2A1
    Q1A2 -->|"Mistake or skip"| Q1C2["Repeat the local hint; keep this question"]
    Q1C2 --> Q1A3["Same question: attempt 3"]
    Q1A3 -->|"Correct"| Q2A1
    Q1A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q2A1["Ten seats; 3 passengers. How many more fit?: attempt 1"] -->|"Correct"| Q3A1
    Q2A1 -->|"Mistake or skip"| Q2C1["Count empty seats; do not count passengers already aboard"]
    Q2C1 --> Q2A2["Same question: attempt 2"]
    Q2A2 -->|"Correct"| Q3A1
    Q2A2 -->|"Mistake or skip"| Q2C2["Repeat the local hint; keep this question"]
    Q2C2 --> Q2A3["Same question: attempt 3"]
    Q2A3 -->|"Correct"| Q3A1
    Q2A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q3A1["Ten seats; 9 passengers. How many more fit?: attempt 1"] -->|"Correct"| Q4A1
    Q3A1 -->|"Mistake or skip"| Q3C1["Count empty seats; do not count passengers already aboard"]
    Q3C1 --> Q3A2["Same question: attempt 2"]
    Q3A2 -->|"Correct"| Q4A1
    Q3A2 -->|"Mistake or skip"| Q3C2["Repeat the local hint; keep this question"]
    Q3C2 --> Q3A3["Same question: attempt 3"]
    Q3A3 -->|"Correct"| Q4A1
    Q3A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q4A1["Ten seats; 0 passengers. How many more fit?: attempt 1"] -->|"Correct"| Q5A1
    Q4A1 -->|"Mistake or skip"| Q4C1["Count empty seats; do not count passengers already aboard"]
    Q4C1 --> Q4A2["Same question: attempt 2"]
    Q4A2 -->|"Correct"| Q5A1
    Q4A2 -->|"Mistake or skip"| Q4C2["Repeat the local hint; keep this question"]
    Q4C2 --> Q4A3["Same question: attempt 3"]
    Q4A3 -->|"Correct"| Q5A1
    Q4A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q5A1["Ten seats; 10 passengers. How many more fit?: attempt 1"] -->|"Correct"| Done
    Q5A1 -->|"Mistake or skip"| Q5C1["Count empty seats; do not count passengers already aboard"]
    Q5C1 --> Q5A2["Same question: attempt 2"]
    Q5A2 -->|"Correct"| Done
    Q5A2 -->|"Mistake or skip"| Q5C2["Repeat the local hint; keep this question"]
    Q5C2 --> Q5A3["Same question: attempt 3"]
    Q5A3 -->|"Correct"| Done
    Q5A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Done["Lesson complete"]
```

## Too many toys were added

If the child is asked to add two toys but adds three, the tutor says:
“Too many toys joined. Put ALL the toys you just added back beside the table.
Leave the toys that were here before. A grown-up can help.”

The child undoes the whole attempted addition. The tutor waits for the original
stable total, then asks for the original addition again. It never asks the child
to calculate an excess or remove just that many toys. Even passing through the
correct combined total during removal does not bypass the reset. Setup errors
do not score failed answers or erase completed missions. Partial additions and
moving/unknown observations do not trigger this overshoot correction.

```mermaid
flowchart TD
    Choose["Original toy group verified"] --> Add["Ask for the new toy group"]
    Add -->|"Correct stable combined total"| Count["Count the combined group"]
    Count --> Done["Continue the current mission"]
    Add -->|"Stable total is too large"| Undo["Remove ALL toys just added; keep original toys"]
    Undo -->|"Wait until original stable total is restored"| Retry["Ask for the SAME addition again"]
    Retry --> Done
    Undo -->|"Break or skip requested"| Rest["Rest; keep earned progress"]
```

This acyclic segment shows one physical setup attempt. The retry opens a new
attempt of the same addition; it does not restart the lesson. Physical setup
has no enforced timeout or fixed retry count. The camera verifies totals, not
which individual toys were removed; a grown-up can help keep the original group.


## Six missions and acceptance

1. 5 + ? = 10 → 5
2. 7 + ? = 10 → 3
3. 3 + ? = 10 → 7
4. 9 + ? = 10 → 1
5. 0 + ? = 10 → 10
6. 10 + ? = 10 → 0

Independently find all six complements, including an empty and a full ferry.

## Toy-based help

1. Prepare the passengers already aboard.
2. Board new toy passengers one at a time up to ten.
3. Count only the new passengers, then recount everyone.

Each camera-visible stage waits for its required stable total and a complete
spoken count (or zero for an empty group). A disturbed table cancels the pending
answer without scoring an error. A wrong guided count is modeled, then preparation
restarts. Setup itself is unscored. The camera verifies totals, not toy identities,
the meaning of the scene, or the child's pointing.

## Controls, progress, and integration

Strategy ID: `unmath-make-ten`. Use `Lesson::TenSeatFerry`
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
`unmath.lesson9.adventure.v1`; saved values keep completed missions, the next
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
