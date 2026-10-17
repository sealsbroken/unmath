# Lesson 1: Count to five

Strategy: `unmath-count-to-five`.

Goal: independently say `1, 2, 3, 4, 5` twice in order, separated by an activity
counting five real toys and answering the total. A single answer of "five" does
not demonstrate the full sequence. Modeled practice does not earn either
independent success.

## Independent questions (one pass)

```mermaid
flowchart TD
    Q0["Can you count from one to five?"]
    Q0 -->|"Exact sequence 1,2,3,4,5"| First["Save first independent success"]
    Q0 -->|"Needs help"| Help["Begin a practice pass"]
    First --> Setup["Prepare five toys; wait for stable total"]
    Setup --> Q1["Can you count these toys aloud?"]
    Q1 -->|"Exact sequence 1,2,3,4,5"| Q2["How many altogether?"]
    Q1 -->|"Needs help"| Help
    Q2 -->|"Five"| Move["Invite rearranging the five toys"]
    Q2 -->|"Other answer"| Retry["New pass at the five-toy count"]
    Move --> Q3["Count from one to five again, by yourself"]
    Q3 -->|"Exact sequence 1,2,3,4,5"| Pass["PASS: two independent counts"]
    Q3 -->|"Needs help"| Help
```

## Practice questions (one successful pass)

```mermaid
flowchart TD
    One["One toy: tutor models 1; child counts"]
    One -->|"Correct"| Two["Two toys: model 1,2; child counts"]
    Two -->|"Correct"| Three["Three toys: model 1,2,3; child counts"]
    Three -->|"Correct"| Four["Four toys: model 1,2,3,4; child counts"]
    Four -->|"Correct"| Five["Five toys: model 1,2,3,4,5; child counts"]
    Five -->|"Correct"| Fresh["New independent pass: count from one to five"]
    One -->|"Needs help"| Retry["New practice pass; reduce set on second miss, minimum one"]
    Two -->|"Needs help"| Retry
    Three -->|"Needs help"| Retry
    Four -->|"Needs help"| Retry
    Five -->|"Needs help"| Retry
    Retry -->|"Third consecutive miss"| Pause["PAUSE"]
```

Stable table totals gate toy questions. A table change cancels the pending toy
answer. The strategy cannot verify one touch per toy or actual rearrangement.
Counting uses the complete `Numbers` sequence, consumed with its following
`Heard` event. Taps cannot satisfy the sequence checks.

Three consecutive unsuccessful responses/timeouts pause. Silence creates no
failed task record. Stop or the task limit also pauses; Repeat reissues the
question; Skip records a skipped attempt and enters practice (subject to the
same limits). Setup has no enforced timeout. Paused sessions preserve the first
independent success and can resume at the five-toy activity.
