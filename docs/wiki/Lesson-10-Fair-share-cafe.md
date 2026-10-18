# Lesson 10: Fair-share café

**Skill:** Equal sharing between two. **Prerequisites:** Count to ten; understanding equal amounts.
Prepare **10 recognizable toys**. Story props are optional; real food is not needed.

Run a toy café. Choose two guests and deal pretend treats fairly, one to each in turn.

Example question: “There are eight treats for two guests. Both must get the same. How many does EACH guest get?”

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
    Choose["How many treats does EACH of two guests get?"] -->|"Independent success"| Done["Keep progress; continue to next question or finish"]
    Choose -->|"Answers with all the treats"| E0["We need ONE guest share, not all treats"]
    E0 --> S0
    Choose -->|"Answers two because there are two guests"| E1["Count treats for one guest, not the guests"]
    E1 --> S0
    Choose -->|"Other wrong answer"| E2["Deal alternately before counting shares"]
    E2 --> S0
    S0["Prepare and count all the treats"] -->|"Guided success; no mastery credit"| S1
    S0 -->|"Mistake at this step"| H0["Model the current settled count; one number per toy"]
    H0 --> R0["New guided attempt at this SAME step"]
    R0 -->|"Guided success"| S1
    R0 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    S1["Deal alternately; count ONLY the first guest share"] -->|"Guided success; no mastery credit"| S2
    S1 -->|"Mistake at this step"| H1["Model the current settled count; one number per toy"]
    H1 --> R1["New guided attempt at this SAME step"]
    R1 -->|"Guided success"| S2
    R1 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    S2["Count ONLY the second guest share"] -->|"Guided success; no mastery credit"| Follow
    S2 -->|"Mistake at this step"| H2["Model the current settled count; one number per toy"]
    H2 --> R2["New guided attempt at this SAME step"]
    R2 -->|"Guided success"| Follow
    R2 -->|"Needs another teaching step"| Continue["Next forward correction episode; keep learned steps"]
    Follow["How many treats did EACH guest receive?"] -->|"Independent success"| Done
    Follow -->|"Still needs help"| Focus["Revisit the current share; do not introduce a division rule"]
    Focus --> Continue
    Choose -->|"Break or budget reached"| Rest["Rest; save earned progress"]
    Continue -->|"Child requests rest, task limit, or three consecutive misses"| Rest
```

## Six missions and acceptance

1. Share 2 → 1 each
2. Share 4 → 2 each
3. Share 6 → 3 each
4. Share 8 → 4 each
5. Share 10 → 5 each
6. New guests: share 6 → 3 each

Independently name each guest’s equal share for every mission, including a fresh six-treat situation. This introduces sharing, not formal division notation.

## Toy-based help

1. Count all toy treats; guests stay outside the counting area.
2. Deal alternately to two guests with adult help.
3. Count only the first guest’s share on the table.
4. Replace it with and count only the second guest’s share.

Each camera-visible stage waits for its required stable total and a complete
spoken count (or zero for an empty group). A disturbed table cancels the pending
answer without scoring an error. A wrong guided count is modeled, then that same physical stage is tried again. Setup itself is unscored. The camera verifies totals, not toy identities,
the meaning of the scene, or the child's pointing.

Keep the two guest toys outside the detector's counting area. Ten treats plus two
guests can therefore use twelve toys in the room, but only ten counted at once;
pretend guests can also be hands or imagined characters. The camera cannot
verify equal distribution or distinguish the two shares. An adult should check
the alternating deal and that each presented group belongs to the intended guest.
The independent check measures the child's numerical sharing answer.

## Controls, progress, and integration

Strategy ID: `unmath-share-between-two`. Use `Lesson::FairShareCafe`
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
`unmath.lesson10.adventure.v1`; saved values keep completed missions, the next
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
