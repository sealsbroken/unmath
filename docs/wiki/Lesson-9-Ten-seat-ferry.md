# Lesson 9: Ten-seat ferry

**Skill:** Complements to ten. **Prerequisites:** Count to ten; missing addends within five.
Prepare **10 recognizable toys**. Story props are optional; real food is not needed.

Captain a ferry with ten pretend seats. Work out how many friends can still board.

Example question: “The ferry has ten seats and seven passengers. How many more can board to fill it?”

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
    Choose["How many NEW passengers can fill the empty ferry seats?"] -->|"Independent success"| Done["Keep progress; continue to next question or finish"]
    Choose -->|"Answers ten or the whole group"| E0["We need only NEW passengers"]
    E0 --> S0
    Choose -->|"Counts the passengers already aboard"| E1["Those were here before"]
    E1 --> S0
    Choose -->|"Ferry full but asks for more"| E2["No empty seats means no new passengers"]
    E2 --> S0
    S0["Prepare and count the passengers already aboard"] -->|"Guided success; no mastery credit"| S1
    S0 -->|"Mistake at this step"| H0["Model the current settled count; one number per toy"]
    H0 --> R0["New guided attempt at this SAME step"]
    R0 -->|"Guided success"| S1
    R0 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    S1["Fill the empty seats; count the complete ferry"] -->|"Guided success; no mastery credit"| S2
    S1 -->|"Mistake at this step"| H1["Model the current settled count; one number per toy"]
    H1 --> R1["New guided attempt at this SAME step"]
    R1 -->|"Guided success"| S2
    R1 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    S2["Keep ONLY the new arrivals; count that group"] -->|"Guided success; no mastery credit"| Follow
    S2 -->|"Mistake at this step"| H2["Model the current settled count; one number per toy"]
    H2 --> R2["New guided attempt at this SAME step"]
    R2 -->|"Guided success"| Follow
    R2 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    Follow["How many NEW passengers could join the ferry?"] -->|"Independent success"| Done
    Follow -->|"Still needs help"| Focus["Recount only the new-arrival group; preserve the occupied-seat count"]
    Focus --> Continue
    Choose -->|"Break or budget reached"| Rest["Rest; save earned progress"]
    Continue -->|"Child requests rest, task limit, or three consecutive misses"| Rest
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
answer without scoring an error. A wrong guided count is modeled, then that same physical stage is tried again. Setup itself is unscored. The camera verifies totals, not toy identities,
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
