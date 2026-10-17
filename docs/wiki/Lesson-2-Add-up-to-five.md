# Lesson 2: Add up to five

Strategy: `unmath-add-to-five`. Prerequisite: count up to five objects.

Goal: independently answer all 21 ordered addition facts with nonnegative
operands and a total no greater than five. Adding zero and reversed operands
are included. Fingers are allowed; memorized recall is not required. Guided
answers do not mark a fact mastered.

## Introduction and independent questions

```text
 [Start a new lesson]
          |
          v
 [Guided 1+1 demonstration: use practice graph below]
          |
          | guided success
          v
 [Choose an unmastered fact a+b, where a+b <= 5]
          |
          v
 [Q: What is a plus b? You can use your fingers.]
          |
     +----+--------------------------+
     | correct                       | incorrect / skip
     v                               v
 [Save this fact as mastered]     [Guided practice for this fact]
     |                               |
     v                               v
 [Are all 21 facts mastered?]      [New check pass; choose another
     |                             unchecked fact when possible]
     +-- yes --> [PASS]
     |
     +-- no ---> [New check pass for an unmastered fact]
```

A resumed lesson with saved mastered facts begins at fact selection and skips
the introductory demonstration. Spoken totals and number-pad answers are both
accepted for independent checks.

## Guided practice questions (one pass)

```text
 [Q0: Can you put a toys on the table? Adult may help.]
                         |
                         | stable total a
                         v
 [Q1: Can you add b more toys, then move your hands away?]
                         |
                         | stable total a+b
                         | (b=0 needs no table change)
                         v
 [Is the combined set empty?]
             |
     +-------+---------------------------+
     | yes                               | no
     v                                   v
 [Q2: No toys. How many?]        [Q3: Count all the toys aloud.]
     |                                   |
     | zero                              | full sequence 1..a+b
     +----------------+------------------+
                      v
 [Explain: a plus b makes the total; adding joins sets]
                      |
                      v
 [New independent check pass; guided answer earns no mastery]

 Q2 or Q3
     |
     +-- wrong / skip --> [Model total; new practice pass from Q0]
     |
     +-- table changed --> [Discard pending answer;
     |                      new preparation pass from Q0]
     |
     +-- third consecutive miss --> [PAUSE]
```

## Fact coverage

```text
 Total     Ordered facts
 -----     ---------------------------------------
   0       0+0
   1       0+1  1+0
   2       0+2  1+1  2+0
   3       0+3  1+2  2+1  3+0
   4       0+4  1+3  2+2  3+1  4+0
   5       0+5  1+4  2+3  3+2  4+1  5+0
```

This table lists coverage, not the precise selection order. The implementation
uses a fixed order, skips mastered facts, and revisits failed facts after guided
practice. It never asks an addition fact with a total above five.

Three consecutive mistakes/timeouts, Stop, or the task limit pauses. Silence
does not create a failed task record. Repeat reissues the current prompt.
Skipping a question records a skip and starts practice; skipping preparation
pauses. Setup has no enforced timeout. Save `SessionSummary.remember` to resume
with only unmastered facts. Completion clears the addition checkpoint.

Camera observations verify totals, not individual toy identities, the exact
movement of toys, or the child's reasoning. Completion checks current answers;
it does not establish delayed retention.
