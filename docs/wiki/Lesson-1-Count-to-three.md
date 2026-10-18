# Lesson 1: Three passengers

Start with **one toy**, then add one friend at a time: **one → two → three**.
The child chooses a forest bus or moon rocket and makes their toys passengers.
Keep other toys aside; extra visible toys never skip the one-toy opening.

Use short spoken turns: “Point to the toy. One! Say one.” Then “One more
friend! Point to each toy. One, two! Your turn.” The first two counts are modeled;
the child then counts three independently, checks tickets, answers how many,
and counts again for departure. Each number belongs to one toy.

This is a gentle starting activity intended for a two-year-old, with a grown-up
helping prepare toys. Pause whenever the child wants; there is no time pressure
for arranging toys. One available toy is enough to begin; three are needed to
complete. With fewer toys, progress is saved at the next stop.

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
    Q0A1["One toy: point and say one: attempt 1"] -->|"Correct"| Q1A1
    Q0A1 -->|"Mistake or skip"| Q0C1["Fix the start, skipped number, or repeated number; keep the same toys"]
    Q0C1 --> Q0A2["Same question: attempt 2"]
    Q0A2 -->|"Correct"| Q1A1
    Q0A2 -->|"Mistake or skip"| Q0C2["Repeat the local hint; keep this question"]
    Q0C2 --> Q0A3["Same question: attempt 3"]
    Q0A3 -->|"Correct"| Q1A1
    Q0A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q1A1["Two toys: count each toy: attempt 1"] -->|"Correct"| Q2A1
    Q1A1 -->|"Mistake or skip"| Q1C1["Fix the start, skipped number, or repeated number; keep the same toys"]
    Q1C1 --> Q1A2["Same question: attempt 2"]
    Q1A2 -->|"Correct"| Q2A1
    Q1A2 -->|"Mistake or skip"| Q1C2["Repeat the local hint; keep this question"]
    Q1C2 --> Q1A3["Same question: attempt 3"]
    Q1A3 -->|"Correct"| Q2A1
    Q1A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q2A1["Three toys: independent count: attempt 1"] -->|"Correct"| Q3A1
    Q2A1 -->|"Mistake or skip"| Q2C1["Fix the start, skipped number, or repeated number; keep the same toys"]
    Q2C1 --> Q2A2["Same question: attempt 2"]
    Q2A2 -->|"Correct"| Q3A1
    Q2A2 -->|"Mistake or skip"| Q2C2["Repeat the local hint; keep this question"]
    Q2C2 --> Q2A3["Same question: attempt 3"]
    Q2A3 -->|"Correct"| Q3A1
    Q2A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q3A1["Tickets: count the same three toys: attempt 1"] -->|"Correct"| Q4A1
    Q3A1 -->|"Mistake or skip"| Q3C1["Fix the start, skipped number, or repeated number; keep the same toys"]
    Q3C1 --> Q3A2["Same question: attempt 2"]
    Q3A2 -->|"Correct"| Q4A1
    Q3A2 -->|"Mistake or skip"| Q3C2["Repeat the local hint; keep this question"]
    Q3C2 --> Q3A3["Same question: attempt 3"]
    Q3A3 -->|"Correct"| Q4A1
    Q3A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q4A1["How many passengers altogether?: attempt 1"] -->|"Correct"| Q5A1
    Q4A1 -->|"Mistake or skip"| Q4C1["Fix the start, skipped number, or repeated number; keep the same toys"]
    Q4C1 --> Q4A2["Same question: attempt 2"]
    Q4A2 -->|"Correct"| Q5A1
    Q4A2 -->|"Mistake or skip"| Q4C2["Repeat the local hint; keep this question"]
    Q4C2 --> Q4A3["Same question: attempt 3"]
    Q4A3 -->|"Correct"| Q5A1
    Q4A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q5A1["Departure: independently count three again: attempt 1"] -->|"Correct"| Done
    Q5A1 -->|"Mistake or skip"| Q5C1["Fix the start, skipped number, or repeated number; keep the same toys"]
    Q5C1 --> Q5A2["Same question: attempt 2"]
    Q5A2 -->|"Correct"| Done
    Q5A2 -->|"Mistake or skip"| Q5C2["Repeat the local hint; keep this question"]
    Q5C2 --> Q5A3["Same question: attempt 3"]
    Q5A3 -->|"Correct"| Done
    Q5A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Done["Lesson complete"]
```


## What counts as success?

Opening counts of one and two never declare mastery. The child must say the
full sequence `1, 2, 3` independently for three actual toys, complete a ticket
count and its total, then independently count `1, 2, 3` again for departure.
A final number alone, zero-start, skipped or repeated number cannot pass a
sequence check. The uninterrupted route takes six scored questions.

## Screen-free controls and progress

Say `forest` or `moon` to choose; `help` to practice with toys and then return to the current question; `start again`
to repeat; `stop`, `break`, or `I'm tired` to rest. English and Russian are supported.
Three consecutive misses/timeouts or the task limit pause without mastery.
Table changes invalidate unfinished counts; pointing while the detected count
stays unchanged does not interrupt. The camera verifies totals, not individual touches.

Strategy: `unmath-count-to-three`; Rust type: `CountToThree`. Old Rust type names
remain compatibility aliases. The old strategy ID is replaced. Old five-toy
checkpoints are ignored; save `SessionSummary.remember` using the new keys
`unmath.lesson1.count_three.v1` and `unmath.lesson1.warmup_three.v1`.

Before lesson 2, separately practice counting four and five with a grown-up:
finishing this lesson demonstrates counting to three, not readiness for all
addition questions up to five.

See the [edge-case audit](Edge-case-audit).
