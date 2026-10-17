# Arithmetic adventures: lessons 3–10

This is the implementation plan for the next eight lessons after Five passengers
and Picnic baskets. Each is a short, choice-led adventure with six independent
missions, its own physical help sequence, a directed cyclic question graph, and
reachable completion/rest exits. The child may count with fingers; fast recall
is not required. These are authored teaching choices, not age-based placement rules.
Offer the next lesson when its prerequisite skills are comfortable; there is no
automatic promotion or grade assignment.

| Lesson | Story and skill | Prerequisite | Counted toys |
| --- | --- | --- | --- |
| 3 | Cave train: taking away within five | Lessons 1–2: counting and combining within five | 5 |
| 4 | Bridge builders: more, fewer, and equal within five | Count sets to five | 5 |
| 5 | Parcel detectives: missing addends within five | Addition within five; comparison helps | 5 |
| 6 | Firefly gates: ordered counting through ten | Count to five independently | 10 |
| 7 | Supply launch: combining quantities with totals six through ten | Count to ten; addition within five | 10 |
| 8 | Homeward trail: taking away within ten | Count to ten; subtraction within five | 10 |
| 9 | Ten-seat ferry: complements to ten | Count to ten; missing addends within five | 10 |
| 10 | Fair-share café: equal sharing between two | Count to ten; understanding equal amounts | 10 |

The progression moves from taking away and comparing small sets to missing
quantities, extends counting to ten, then applies combining/taking away to larger
sets. Completing ten connects addition with missing quantities. Equal sharing
introduces a new operation through one-for-each dealing rather than symbols.

## Acceptance and implementation

Each lesson has exactly six defined missions, listed in its wiki page. Only
independent numerical/sequence answers complete missions. Wrong answers branch
to toy preparation and counting, explanation, and a fresh challenge. Guided
answers are recorded but do not earn mastery. All missions must be completed;
finishing a time/task allowance never implies success.

Lessons 3–5 need five recognizable toys; 6–10 need ten counted toys. The fair-share
café's guest characters stay outside the counting area. Setup totals and guided
counts are observable. Matching identities, pointing once per toy, pretend rewards,
and fair distribution require child/adult participation and are not inferred from
camera totals. No engine changes, network calls, image assets, or video features
are required. English and Russian are implemented.

The eight strategies use `curriculum::Adventure` for event handling, with separate
mission content and physical stage sequences. `NEXT_LESSONS` exposes the catalogue;
`unmath::register` and link-time inventory include all ten lessons. Checkpoints are
versioned and isolated per lesson. Existing strategy IDs and lessons 1–2 remain.

## Validation

Tests cover every lesson's registration, exact planned answers, full completion,
help cycles, deferred independent retries, sequence-only counting, table changes,
spoken comparison choices, task/material limits, Russian controls, and resume
isolation. README and wiki graphs show explicit cycles. Host camera, speech, and
child usability testing remain product-level checks; software tests cannot
establish that a child enjoys or has retained a lesson.

## Lesson pages

- [Lesson 3: Cave train](https://github.com/sealsbroken/unmath/wiki/Lesson-3-Cave-train)
- [Lesson 4: Bridge builders](https://github.com/sealsbroken/unmath/wiki/Lesson-4-Bridge-builders)
- [Lesson 5: Parcel detectives](https://github.com/sealsbroken/unmath/wiki/Lesson-5-Parcel-detectives)
- [Lesson 6: Firefly gates](https://github.com/sealsbroken/unmath/wiki/Lesson-6-Firefly-gates)
- [Lesson 7: Supply launch](https://github.com/sealsbroken/unmath/wiki/Lesson-7-Supply-launch)
- [Lesson 8: Homeward trail](https://github.com/sealsbroken/unmath/wiki/Lesson-8-Homeward-trail)
- [Lesson 9: Ten-seat ferry](https://github.com/sealsbroken/unmath/wiki/Lesson-9-Ten-seat-ferry)
- [Lesson 10: Fair-share café](https://github.com/sealsbroken/unmath/wiki/Lesson-10-Fair-share-cafe)
