# Lesson 4: Compare toy groups

Compare two toy groups within five: more, fewer, or equal.

The lesson starts directly with a toy task. There is no destination menu and
no pretend role assigned to any toy. English and Russian prompts describe real
toys and groups of toys. Instructions never ask for a grown-up.

Correct answers may be repeated: `1, 2, 1, 2` for two toys is one successful
response, not two tasks. Each cycle must be complete. Zero-starts, skipped
numbers, duplicates within a count, and incomplete restarts need correction.
After an incorrect spoken answer, the next teaching question begins with
“А ты уверен?” / “Are you sure?” once. Skips and silence do not trigger that prefix.

## Tasks and acceptance

1. 2 and 4: more
2. 5 and 3: fewer
3. 3 and 3: more or equal
4. 4 and 1: more
5. 2 and 5: fewer
6. 1 and 1: fewer or equal

## Acyclic teaching and correction paths

A mistake opens concrete teaching subquestions, not a restart. Failed guided
counts retry the current group. Later errors revisit only the remaining concept.
The graph unfolds one correction episode into distinct forward nodes. `Continue`
marks the next episode at the current learning step; prompt replays and physical
waiting do not advance it. The session task budget bounds scored steps.

```mermaid
flowchart TD
    Start["Compare two toy groups within five: more, fewer, or equal."] -->|"Independent success"| Done["Keep progress; next question or finish"]
    Start -->|"Wrong group for fewer"| E0["The row that runs out first has fewer toys"]
    E0 --> S0
    Start -->|"Chooses a side when equal"| E1["If every toy has a partner, the groups are equal"]
    E1 --> S0
    Start -->|"Other wrong choice"| E2["Count and match the two toy groups"]
    E2 --> S0
    S0["Prepare and count the FIRST toy group"] -->|"Guided success; no mastery credit"| S1
    S0 -->|"Mistake here"| H0["Model this settled toy group; say one number per toy"]
    H0 --> R0["Fresh guided attempt at this same step"]
    R0 -->|"Guided success"| S1
    R0 -->|"More teaching needed"| Continue["Next correction episode; retain learned steps"]
    S1["Prepare and count the SECOND toy group"] -->|"Guided success; no mastery credit"| S2
    S1 -->|"Mistake here"| H1["Model this settled toy group; say one number per toy"]
    H1 --> R1["Fresh guided attempt at this same step"]
    R1 -->|"Guided success"| S2
    R1 -->|"More teaching needed"| Continue["Next correction episode; retain learned steps"]
    S2["Pair the groups: which has toys WITHOUT a partner?"] -->|"Guided success; no mastery credit"| S3
    S2 -->|"Mistake here"| H2["Make pairs again; look for toys without partners"]
    H2 --> R2["Fresh guided attempt at this same step"]
    R2 -->|"Guided success"| S3
    R2 -->|"More teaching needed"| Continue["Next correction episode; retain learned steps"]
    S3["Do toys without partners mean MORE, FEWER, or SAME?"] -->|"Guided success; no mastery credit"| Follow
    S3 -->|"Mistake here"| H3["Toys without partners mean more; complete pairs in both groups mean equal"]
    H3 --> R3["Fresh guided attempt at this same step"]
    R3 -->|"Guided success"| Follow
    R3 -->|"More teaching needed"| Continue["Next correction episode; retain learned steps"]
    Follow["We need MORE or FEWER toys: which group should we choose?"] -->|"Independent success"| Done
    Follow -->|"Still needs help"| Continue
    Start -->|"Break or budget reached"| Rest["Rest; save progress"]
    Continue -->|"Break, task limit, or three consecutive misses"| Rest
```

## Controls, progress, and limits

Say `help` / `помоги` for teaching steps; `start again` / `сначала` to repeat the
current prompt; `stop`, `break`, `I'm tired` / `стоп`, `перерыв`, `я устал` to pause.
Help and corrections keep completed work. Guided answers never complete an
independent check. Three consecutive unsuccessful responses/timeouts or the
task budget pause. Skip during toy setup pauses without inventing a failed answer.

Toy setup waits for the required stable detected total. An overshoot during
addition undoes ALL newly added toys, restores the original group, and repeats
the same addition; the child is never asked to subtract the excess. Setup has
no enforced timeout. The camera checks totals, not toy identities or individual
touches; it cannot prove which toys were moved or that two groups were paired.

Save `SessionSummary.remember` between sessions. Completed arithmetic checks
are preserved; legacy destination values are ignored and cleared. English and
Russian use the same transitions. Existing strategy IDs and page URLs remain
compatible with integrations.

Strategy ID: `unmath-compare-to-five`. See the [edge-case audit](Edge-case-audit).
