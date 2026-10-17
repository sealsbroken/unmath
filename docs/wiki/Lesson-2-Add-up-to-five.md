# Lesson 2: Picnic baskets

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

## Directed cyclic question graph

The practice path returns to the mission selector, and the success path selects
another mission. These are actual directed cycles with completion and rest exits.

```mermaid
flowchart TD
    Choose["Forest picnic or moon picnic?"] --> Base["Guided basket: prepare a toy snacks"]
    Base -->|"Stable total a"| Add["Add b more toy snacks"]
    Add -->|"Stable total a+b"| Count["Point to each toy; say one number per toy"]
    Count -->|"Full sequence"| Explain["Explain a plus b makes this many"]
    Explain --> Select{"Any picnic missions left?"}
    Select -->|"Yes"| Pack["New basket: prepare a actual toy snacks"]
    Pack -->|"Stable total a"| Join["Add b more toys; zero needs no change"]
    Join -->|"Stable total a+b"| Mission["Point to toys one by one; count each aloud without a tutor model"]
    Mission -->|"Full sequence 1 through a+b"| Play["Name the total; celebrate with a pretend picnic action"]
    Play --> Select
    Mission -->|"Help, incomplete count, or skip"| Base
    Mission -->|"Table changed: discard pending answer"| Pack
    Count -->|"Incorrect: model count and rebuild"| Base
    Select -->|"All seven complete"| Feast["The picnic is ready! Pretend feast"]
    Count -->|"Three misses or task limit"| Rest["Rest and save progress"]
    Mission -->|"Three misses or task limit"| Rest
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
mission returns after guided play; another unfinished mission comes first when
possible. Adding zero needs no camera change.

## What counts as success?

Build and count all seven real toy baskets without the tutor modeling the
current count. Each mission waits for the starting group, the added group, and
a stable combined total before listening. Toy-assisted guided responses do not complete missions. This checks
combining quantities, including two ways to make five; it does not claim mastery
of every addition fact or delayed retention. Both independent and guided counts require the full sequence `1..a+b`. A lone
final number, a tap, or a sequence with skipped/repeated numbers cannot complete
a basket. After a correct count, the tutor names the total to connect counting
with “how many altogether.”

## Child controls and engine behavior

The same destination choices, spoken help/break requests, Repeat, and Skip as
lesson one are available. Help rebuilds the current basket without an error
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
