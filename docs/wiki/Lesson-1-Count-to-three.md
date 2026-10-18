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

## Directed cyclic question graph

Help loops through a smaller group and modeled practice, then returns to an
independent count. Success and rest are reachable exits.

```mermaid
flowchart TD
    Choose["Where shall we go: forest or moon?"] --> One["Put just one toy here; keep others aside"]
    One -->|"Stable total one"| CountOne["Point: one! Say one"]
    CountOne -->|"Full sequence"| Two["One more friend: prepare two toys"]
    Two -->|"Stable total two"| CountTwo["Point: one, two! Your turn"]
    CountTwo -->|"Full sequence"| Setup["One more friend: prepare three toys"]
    CountOne -->|"Needs help"| Small
    CountTwo -->|"Needs help"| Small
    CountOne -->|"Starts at zero"| Zero
    CountTwo -->|"Starts at zero"| Zero
    Setup -->|"Stable total three"| Doors["Point to each toy; count independently to three"]
    Doors -->|"Full sequence"| Tickets["Point to the toys one by one; count each ticket aloud"]
    Doors -->|"Needs help"| Small["Prepare n toy passengers; first practice starts at one"]
    Tickets -->|"Full sequence"| Total["How many passengers are coming to our picnic?"]
    Tickets -->|"Needs help"| Small
    Total -->|"Three"| Seats["Let the three passengers choose new seats"]
    Total -->|"Other answer"| Tickets
    Seats --> Engine["Point to each passenger again; count one by one to start our adventure"]
    Engine -->|"Full sequence and earlier independent success"| Picnic["Off to the picnic! Lesson complete"]
    Engine -->|"Needs help"| Small
    Small -->|"Required toys ready"| Model["Tutor counts the tickets; can you try by yourself?"]
    Model -->|"Correct; fewer than three"| More["Next stop: one more passenger"]
    More --> Small
    Model -->|"Correct at three"| Doors
    Model -->|"Needs help: same set or smaller on second miss"| Small
    Model -->|"Three consecutive misses"| Rest["Park and rest; save progress"]
    Doors -->|"Starts at zero with toys"| Zero["Zero means no toys; first toy gets one"]
    Tickets -->|"Starts at zero with toys"| Zero
    Engine -->|"Starts at zero with toys"| Zero
    Model -->|"Starts at zero with toys"| Zero
    Zero --> Small
```

## What counts as success?

Opening counts of one and two never declare mastery. The child must say the
full sequence `1, 2, 3` independently for three actual toys, complete a ticket
count and its total, then independently count `1, 2, 3` again for departure.
A final number alone, zero-start, skipped or repeated number cannot pass a
sequence check. The uninterrupted route takes six scored questions.

## Screen-free controls and progress

Say `forest` or `moon` to choose; `help` to practice with one toy; `start again`
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
