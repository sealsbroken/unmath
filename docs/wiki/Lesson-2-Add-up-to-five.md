# Lesson 2: Picnic baskets

Prerequisite: count real toys one by one to five. Lesson 1 now stops at three;
practice four and five with a grown-up before beginning these missions.

The passengers have arrived, and the child is in charge of the picnic. Choose
**forest** to feed pretend bears or **moon** to feed pretend aliens. Any toy can
stand for a snack; no real food or special props are needed.

First, pack one toy snack and add another. Count the combined basket. Then solve
seven short picnic missions with totals up to five. Each completed mission earns
a celebration and a pretend action: spread a blanket, pass a basket, pour a drink,
smell the soup, or offer a favorite toy a snack. These actions are invitations,
not tasks judged by the camera.

Example: “The bears have two snacks and bring three more. Point to the toy
snacks one by one and say one number for each, starting at one.”
If the child asks for help, build that basket with toys, count it, explain how
joining made the total, and return to the missions. Use the actual toy basket for each count.

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
    Q0A1["Opening: join one toy and one toy; count both: attempt 1"] -->|"Correct"| Q1A1
    Q0A1 -->|"Mistake or skip"| Q0C1["Point to each toy once; start at one; keep the assembled basket"]
    Q0C1 --> Q0A2["Same question: attempt 2"]
    Q0A2 -->|"Correct"| Q1A1
    Q0A2 -->|"Mistake or skip"| Q0C2["Repeat the local hint; keep this question"]
    Q0C2 --> Q0A3["Same question: attempt 3"]
    Q0A3 -->|"Correct"| Q1A1
    Q0A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q1A1["Basket: 1 plus 2; count each toy: attempt 1"] -->|"Correct"| Q2A1
    Q1A1 -->|"Mistake or skip"| Q1C1["Point to each toy once; start at one; keep the assembled basket"]
    Q1C1 --> Q1A2["Same question: attempt 2"]
    Q1A2 -->|"Correct"| Q2A1
    Q1A2 -->|"Mistake or skip"| Q1C2["Repeat the local hint; keep this question"]
    Q1C2 --> Q1A3["Same question: attempt 3"]
    Q1A3 -->|"Correct"| Q2A1
    Q1A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q2A1["Basket: 2 plus 2; count each toy: attempt 1"] -->|"Correct"| Q3A1
    Q2A1 -->|"Mistake or skip"| Q2C1["Point to each toy once; start at one; keep the assembled basket"]
    Q2C1 --> Q2A2["Same question: attempt 2"]
    Q2A2 -->|"Correct"| Q3A1
    Q2A2 -->|"Mistake or skip"| Q2C2["Repeat the local hint; keep this question"]
    Q2C2 --> Q2A3["Same question: attempt 3"]
    Q2A3 -->|"Correct"| Q3A1
    Q2A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q3A1["Basket: 2 plus 3; count each toy: attempt 1"] -->|"Correct"| Q4A1
    Q3A1 -->|"Mistake or skip"| Q3C1["Point to each toy once; start at one; keep the assembled basket"]
    Q3C1 --> Q3A2["Same question: attempt 2"]
    Q3A2 -->|"Correct"| Q4A1
    Q3A2 -->|"Mistake or skip"| Q3C2["Repeat the local hint; keep this question"]
    Q3C2 --> Q3A3["Same question: attempt 3"]
    Q3A3 -->|"Correct"| Q4A1
    Q3A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q4A1["Basket: 3 plus 1; count each toy: attempt 1"] -->|"Correct"| Q5A1
    Q4A1 -->|"Mistake or skip"| Q4C1["Point to each toy once; start at one; keep the assembled basket"]
    Q4C1 --> Q4A2["Same question: attempt 2"]
    Q4A2 -->|"Correct"| Q5A1
    Q4A2 -->|"Mistake or skip"| Q4C2["Repeat the local hint; keep this question"]
    Q4C2 --> Q4A3["Same question: attempt 3"]
    Q4A3 -->|"Correct"| Q5A1
    Q4A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q5A1["Basket: 1 plus 4; count each toy: attempt 1"] -->|"Correct"| Q6A1
    Q5A1 -->|"Mistake or skip"| Q5C1["Point to each toy once; start at one; keep the assembled basket"]
    Q5C1 --> Q5A2["Same question: attempt 2"]
    Q5A2 -->|"Correct"| Q6A1
    Q5A2 -->|"Mistake or skip"| Q5C2["Repeat the local hint; keep this question"]
    Q5C2 --> Q5A3["Same question: attempt 3"]
    Q5A3 -->|"Correct"| Q6A1
    Q5A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q6A1["Basket: 3 plus 0; count each toy: attempt 1"] -->|"Correct"| Q7A1
    Q6A1 -->|"Mistake or skip"| Q6C1["Point to each toy once; start at one; keep the assembled basket"]
    Q6C1 --> Q6A2["Same question: attempt 2"]
    Q6A2 -->|"Correct"| Q7A1
    Q6A2 -->|"Mistake or skip"| Q6C2["Repeat the local hint; keep this question"]
    Q6C2 --> Q6A3["Same question: attempt 3"]
    Q6A3 -->|"Correct"| Q7A1
    Q6A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Q7A1["Basket: 1 plus 1; count each toy: attempt 1"] -->|"Correct"| Done
    Q7A1 -->|"Mistake or skip"| Q7C1["Point to each toy once; start at one; keep the assembled basket"]
    Q7C1 --> Q7A2["Same question: attempt 2"]
    Q7A2 -->|"Correct"| Done
    Q7A2 -->|"Mistake or skip"| Q7C2["Repeat the local hint; keep this question"]
    Q7C2 --> Q7A3["Same question: attempt 3"]
    Q7A3 -->|"Correct"| Done
    Q7A3 -->|"Third consecutive miss"| Rest["Rest; save earned progress"]
    Done["Lesson complete"]
```


## Challenge scope

| Picnic basket | Addition |
| --- | --- |
| Two snacks | 1 + 1 |
| Three snacks | 1 + 2 |
| Four snacks | 2 + 2 |
| Five snacks | 2 + 3 |
| Four snacks another way | 3 + 1 |
| Five snacks another way | 1 + 4 |
| A friend brings an empty basket | 3 + 0 |

The opening demonstration is guided `1 + 1`. Missions start at `1 + 2`, then
visit each unfinished basket, with `1 + 1` checked independently later. A failed
count gets a local hint and another attempt with the same basket. Explicit help
rebuilds that basket and then checks it independently; completed missions stay complete. Adding zero needs no camera change.

## What counts as success?

Build and count all seven real toy baskets without the tutor modeling the
current count. Each mission waits for the starting group, the added group, and
a stable combined total before listening. Toy-assisted guided responses do not complete missions. This checks
combining quantities, including two ways to make five; it does not claim mastery
of every addition fact or delayed retention. Both independent and guided counts require the full sequence `1..a+b`. A lone
final number or a sequence with skipped/repeated numbers cannot complete
a basket. After a correct count, the tutor names the total to connect counting
with “how many altogether.”

## Child controls and engine behavior

The same spoken destination choices, help/break requests, and restart requests
as lesson one are available; no screen interaction is required. Help rebuilds the current basket without an error
record. Skipping toy setup pauses. Three consecutive mistakes/timeouts, Stop, or
the task limit pause; silence does not create a failed task record. Setup has no
timeout. The camera verifies totals, not food identities or pretend actions.

Default `max_tasks = 10` fits an uninterrupted demonstration plus seven missions.
Use shorter sessions as needed. Persist `SessionSummary.remember` to keep the
chosen friends and completed baskets. The redesigned checkpoint is
`unmath.lesson2.one_by_one.v3`; older final-number and 21-fact scores are not reused. Completion clears
this checkpoint. `final_level` counts completed missions (seven at completion),
while `correct` also counts guided successes.

Strategy ID remains `unmath-add-to-five`; app title: **Lesson 2: picnic baskets**.
Prerequisite: count up to five objects. English and Russian are supported.

The camera verifies totals and the recognizer verifies the spoken sequence.
A grown-up can observe one touch per toy; the strategy API cannot verify unique
physical touches. Counts can span pauses in `ListenMode::Counting`.

See the [edge-case audit](Edge-case-audit) for zero-starts, counting mutations,
interruptions, saved-state validation, and engine-boundary regressions.
