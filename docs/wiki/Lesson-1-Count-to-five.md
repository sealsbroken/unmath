# Lesson 1: Count to five

Strategy: `unmath-count-to-five`.

Goal: independently say `1, 2, 3, 4, 5` twice in order, separated by an activity
counting five real toys and answering the total. A single answer of "five" does
not demonstrate the full sequence. Modeled practice does not earn either
independent success.

## Independent questions (one pass)

```text
 [Q0: Can you count from one to five?]
                 |
        +--------+---------------------+
        | exact 1,2,3,4,5              | needs help
        v                              v
 [Save first independent success]   [Practice pass below]
        |
        v
 [Prepare five toys; wait for stable total of five]
        |
        v
 [Q1: Can you count these toys aloud?]
        |
        +------------------------------+
        | exact 1,2,3,4,5              | needs help
        v                              v
 [Q2: How many altogether?]          [Practice pass below]
        |
        +------------------------------+
        | five                         | other answer
        v                              v
 [Invite rearranging the five toys] [New pass at Q1]
        |
        v
 [Q3: Count from one to five again, by yourself]
        |
        +------------------------------+
        | exact 1,2,3,4,5              | needs help
        v                              v
 [PASS: two independent counts]     [Practice pass below]
```

## Practice questions (one successful pass)

```text
 [Prepare one toy]
        |
        v
 [Tutor models 1; child counts one toy]
        |
        | correct
        v
 [Prepare two toys; model 1,2; child counts]
        |
        | correct
        v
 [Prepare three toys; model 1,2,3; child counts]
        |
        | correct
        v
 [Prepare four toys; model 1,2,3,4; child counts]
        |
        | correct
        v
 [Prepare five toys; model 1,2,3,4,5; child counts]
        |
        | correct
        v
 [New independent pass at Q0]

 Any practice question
        |
        +-- needs help --> [New practice pass at current set size;
        |                   reduce by one on second miss, minimum one]
        |
        +-- third consecutive miss --> [PAUSE]
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
