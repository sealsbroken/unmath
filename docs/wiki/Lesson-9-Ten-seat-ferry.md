# Lesson 9: Make ten toys

Find how many toys must be added to make ten.

The lesson starts directly with a toy task. There is no destination menu and
no pretend role assigned to any toy. English and Russian prompts describe real
toys and groups of toys. Instructions never ask for a grown-up.

Correct answers may be repeated: `1, 2, 1, 2` for two toys is one successful
response, not two tasks. Each cycle must be complete. Zero-starts, skipped
numbers, duplicates within a count, and incomplete restarts need correction.
After an incorrect spoken answer, the next teaching question begins with
“А ты уверен?” / “Are you sure?” once. Skips and silence do not trigger that prefix.

## Tasks and acceptance

1. 5 + ? = 10
2. 7 + ? = 10
3. 3 + ? = 10
4. 9 + ? = 10
5. 0 + ? = 10
6. 10 + ? = 10

## Acyclic teaching and correction paths

A mistake opens concrete teaching subquestions, not a restart. Failed guided
counts retry the current group. Later errors revisit only the remaining concept.
The graph unfolds one correction episode into distinct forward nodes. `Continue`
marks the next episode at the current learning step; prompt replays and physical
waiting do not advance it. The session task budget bounds scored steps.

```mermaid
flowchart TD
    Start["Find how many toys must be added to make ten."] -->|"Independent success"| Done["Keep progress; next question or finish"]
    Start -->|"Counts all ten or the original group"| E0["Count only the NEW toys"]
    E0 --> S0
    Start -->|"Already at ten"| E1["No new toys are needed"]
    E1 --> S0
    S0["Prepare and count the starting toys"] -->|"Guided success; no mastery credit"| S1
    S0 -->|"Mistake here"| H0["Model this settled toy group; say one number per toy"]
    H0 --> R0["Fresh guided attempt at this same step"]
    R0 -->|"Guided success"| S1
    R0 -->|"More teaching needed"| Continue["Next correction episode; retain learned steps"]
    S1["Add toys until there are ten; count the combined group"] -->|"Guided success; no mastery credit"| S2
    S1 -->|"Mistake here"| H1["Model this settled toy group; say one number per toy"]
    H1 --> R1["Fresh guided attempt at this same step"]
    R1 -->|"Guided success"| S2
    R1 -->|"More teaching needed"| Continue["Next correction episode; retain learned steps"]
    S2["Keep ONLY the new toys; count them"] -->|"Guided success; no mastery credit"| Follow
    S2 -->|"Mistake here"| H2["Model this settled toy group; say one number per toy"]
    H2 --> R2["Fresh guided attempt at this same step"]
    R2 -->|"Guided success"| Follow
    R2 -->|"More teaching needed"| Continue["Next correction episode; retain learned steps"]
    Follow["How many NEW toys did we need to add to make ten?"] -->|"Independent success"| Done
    Follow -->|"Still needs help"| Continue
    Start -->|"Break or budget reached"| Rest["Rest; save progress"]
    Continue -->|"Break, task limit, or three consecutive misses"| Rest
```

## Too many toys were added

“Too many toys were added. Put ALL the toys you just added back beside the table.
Leave the toys that were here before.” The tutor waits for the original stable
total, then repeats the same addition. Removing only the excess does not bypass
this correction. A setup error does not score a failed maths answer.

```mermaid
flowchart TD
    Start["Original toy group verified"] --> Add["Ask for the new toys"]
    Add -->|"Correct stable combined total"| Count["Count all the toys"]
    Count --> Done["Continue current task"]
    Add -->|"Stable total too large"| Undo["Remove ALL newly added toys; leave original toys"]
    Undo -->|"Original stable total restored"| Retry["Ask for the SAME addition again"]
    Retry --> Done
    Undo -->|"Break or skip"| Rest["Rest; keep progress"]
```

This segment shows one physical attempt. Retry opens a fresh attempt of the
same task; it does not restart the lesson.

## Controls, progress, and limits

In Russian, **«раз» is accepted as «один»**: `раз, два` and repeated complete
counts such as `раз, два, раз, два` are valid for two toys. The tutor may model
`один`; either word is accepted. `Ещё раз` / `повтори ещё раз` requests a replay
without becoming the numerical answer one. Zero and skipped numbers still need
correction.

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

Strategy ID: `unmath-make-ten`. See the [edge-case audit](Edge-case-audit).
