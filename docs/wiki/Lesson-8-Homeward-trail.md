# Lesson 8: Take toys away up to ten

Take toys away within ten and count those remaining.

The lesson starts directly with a toy task. There is no destination menu and
no pretend role assigned to any toy. English and Russian prompts describe real
toys and groups of toys. Instructions never ask for a grown-up.

Correct answers may be repeated: `1, 2, 1, 2` for two toys is one successful
response, not two tasks. Each cycle must be complete. Zero-starts, skipped
numbers, duplicates within a count, and incomplete restarts need correction.
After an incorrect spoken answer, the next teaching question begins with
“А ты уверен?” / “Are you sure?” once. Skips and silence do not trigger that prefix.

## Tasks and acceptance

1. 6 - 1 = 5
2. 7 - 3 = 4
3. 8 - 2 = 6
4. 9 - 4 = 5
5. 10 - 5 = 5
6. 10 - 10 = 0

## Acyclic teaching and correction paths

A mistake opens concrete teaching subquestions, not a restart. Failed guided
counts retry the current group. Later errors revisit only the remaining concept.
The graph unfolds one correction episode into distinct forward nodes. `Continue`
marks the next episode at the current learning step; prompt replays and physical
waiting do not advance it. The session task budget bounds scored steps.

```mermaid
flowchart TD
    Start["Take toys away within ten and count those remaining."] -->|"Independent success"| Done["Keep progress; next question or finish"]
    Start -->|"Counts original or removed group"| E0["Separate removed toys; count only those remaining"]
    E0 --> S0
    S0["Prepare and count the original toys"] -->|"Guided success; no mastery credit"| S1
    S0 -->|"Mistake here"| H0["Model this settled toy group; say one number per toy"]
    H0 --> R0["Fresh guided attempt at this same step"]
    R0 -->|"Guided success"| S1
    R0 -->|"More teaching needed"| Continue["Next correction episode; retain learned steps"]
    S1["Move the requested toys aside; count only those remaining"] -->|"Guided success; no mastery credit"| Follow
    S1 -->|"Mistake here"| H1["Model this settled toy group; say one number per toy"]
    H1 --> R1["Fresh guided attempt at this same step"]
    R1 -->|"Guided success"| Follow
    R1 -->|"More teaching needed"| Continue["Next correction episode; retain learned steps"]
    Follow["How many toys remain on the table?"] -->|"Independent success"| Done
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

Strategy ID: `unmath-subtract-to-ten`. See the [edge-case audit](Edge-case-audit).
