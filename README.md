# unmath

Ten play-based arithmetic lessons for unengine apps. It depends only on the public `un-core`
strategy interface, performs no I/O, and uses no network services.

Lesson IDs: `unmath-count-to-five` and `unmath-add-to-five`.
English and Russian prompts are included. `unmath::register` registers all ten.

## Plug into an app

Keep this checkout next to `unengine` (the dependency path in `Cargo.toml` expects
`../unengine`). Add this dependency to the **product binary or mobile library**:

```toml
[dependencies]
unmath = { path = "../unmath" } # adjust relative to the product's Cargo.toml
```

Explicit registration is reliable even when static linking drops unused crates:

```rust
let mut registry = un_core::strategy::StrategyRegistry::linked();
unmath::register(&mut registry);
let config = un_core::strategy::SessionConfig {
    total_objects: 5,
    max_tasks: 40,
    ..Default::default()
};
let strategy = registry.create(unmath::ID, &config).unwrap();
// Pass strategy and config to un_engine::Engine::new, as in unengine's demo.
```

For a desktop product with `un-desktop` as a dependency:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = un_core::strategy::StrategyRegistry::linked();
    unmath::register(&mut registry);
    un_desktop::run(registry)?;
    Ok(())
}
```

Select **Lesson 1: five passengers** in the app's strategy selector. Registration
also participates in `inventory`, so a CLI product can retain the crate with
`unmath::register(&mut un_core::strategy::StrategyRegistry::new());` before
calling `un_cli::main()`. Select it with
`lesson --strategy unmath-count-to-five`.

For a mobile product, call this in the Rust startup function invoked by the app,
before listing or creating sessions:

```rust
un_ffi::add_strategies(unmath::register);
```

Build the product library and regenerate its Swift/Kotlin bindings using the
unengine workflow. This crate is a linked Rust strategy, not a dynamically loaded
plugin; existing prebuilt apps must be rebuilt with it.

## Play adventures

The child chooses a forest picnic by bus or a moon picnic by rocket. Any five
recognizable toys become passengers, then pretend snacks for bears or aliens.
English and Russian prompts support story choices, ticket checks, basket packing,
and playful celebrations. Say `help` to build with toys, or `stop` / `I'm tired`
to pause. No special props are needed. Pretend actions are spoken invitations;
the engine does not render story scenes or judge pretend play.

### Lesson 1: Five passengers

Open the doors by counting, check each passenger's ticket, let the toys choose
new seats, and count again to start the engine. Passing still requires two
independent exact sequences `[1, 2, 3, 4, 5]` separated by the five-toy activity
and its total. Guided practice never earns an independent sequence check.

```mermaid
flowchart TD
    Choose["Where shall we go: forest or moon?"] --> Doors["Captain, count from one to five to open the doors"]
    Doors -->|"Full sequence"| Setup["Can you bring five toy passengers?"]
    Doors -->|"Needs help"| Small["Prepare n toy passengers; first practice starts at one"]
    Setup -->|"Stable total five"| Tickets["Can you count each passenger's ticket aloud?"]
    Tickets -->|"Full sequence"| Total["How many passengers are coming to our picnic?"]
    Tickets -->|"Needs help"| Small
    Total -->|"Five"| Seats["Let the five passengers choose new seats"]
    Total -->|"Other answer"| Tickets
    Seats --> Engine["Can you count from one to five to start our adventure?"]
    Engine -->|"Full sequence and earlier independent success"| Picnic["Off to the picnic! Lesson complete"]
    Engine -->|"Needs help"| Small
    Small -->|"Required toys ready"| Model["Tutor counts the tickets; can you try by yourself?"]
    Model -->|"Correct; fewer than five"| More["Next stop: one more passenger"]
    More --> Small
    Model -->|"Correct at five"| Doors
    Model -->|"Needs help: same set or smaller on second miss"| Small
    Model -->|"Three consecutive misses"| Rest["Park and rest; save progress"]
```


### Lesson 2: Picnic baskets

Pack a guided basket, then solve seven story missions: `1+1`, `1+2`, `2+2`,
`2+3`, `3+1`, `1+4`, and `3+0`. Independent answers earn pretend picnic actions
and move to the next basket. Help returns to combining toys, counting, and a
fresh mission. Two different ways to make five are included. Completion checks
these seven challenges; it does not claim recall of every addition fact.

```mermaid
flowchart TD
    Choose["Forest picnic or moon picnic?"] --> Base["Can you pack a toy snacks? First basket: one"]
    Base -->|"Stable total a"| Add["A friend brings b more; can you add them?"]
    Add -->|"Stable total a+b; zero needs no change"| Count["Can you count all the toy snacks aloud?"]
    Count -->|"Correct"| Explain["a plus b makes this many; joining fills our basket"]
    Explain --> Select{"Any picnic missions left?"}
    Select -->|"Yes: choose an unfinished mission"| Mission["Friends have a snacks and bring b more. How many altogether?"]
    Mission -->|"Correct without tutor giving the answer"| Play["Celebrate; pretend to spread a blanket or serve a friend"]
    Play --> Select
    Mission -->|"Needs help, incorrect, or skip"| Base
    Count -->|"Incorrect: model total"| Base
    Count -->|"Table changed: discard pending answer"| Base
    Select -->|"All seven complete"| Feast["The picnic is ready! Pretend feast"]
    Count -->|"Three consecutive misses or task limit"| Rest["Rest; save picnic progress"]
    Mission -->|"Three consecutive misses or task limit"| Rest
```

Both graphs are directed and cyclic: arrows lead back to earlier questions during
practice and onward through repeated missions. See the [wiki](https://github.com/sealsbroken/unmath/wiki)
for examples, acceptance criteria, child controls, and resume behavior. Page sources
are in `docs/wiki`.

## Next eight lessons: 3–10

See the [curriculum plan](docs/CURRICULUM.md) for sequencing, prerequisites, and
acceptance criteria. Each adventure has six missions and a directed cyclic
question graph in the wiki:

| Lesson | Strategy ID | Skill |
| --- | --- | --- |
| [3: Cave train](https://github.com/sealsbroken/unmath/wiki/Lesson-3-Cave-train) | `unmath-subtract-to-five` | Taking away within five |
| [4: Bridge builders](https://github.com/sealsbroken/unmath/wiki/Lesson-4-Bridge-builders) | `unmath-compare-to-five` | More, fewer, and equal within five |
| [5: Parcel detectives](https://github.com/sealsbroken/unmath/wiki/Lesson-5-Parcel-detectives) | `unmath-missing-to-five` | Missing addends within five |
| [6: Firefly gates](https://github.com/sealsbroken/unmath/wiki/Lesson-6-Firefly-gates) | `unmath-count-to-ten` | Ordered counting through ten |
| [7: Supply launch](https://github.com/sealsbroken/unmath/wiki/Lesson-7-Supply-launch) | `unmath-add-to-ten` | Combining quantities with totals six through ten |
| [8: Homeward trail](https://github.com/sealsbroken/unmath/wiki/Lesson-8-Homeward-trail) | `unmath-subtract-to-ten` | Taking away within ten |
| [9: Ten-seat ferry](https://github.com/sealsbroken/unmath/wiki/Lesson-9-Ten-seat-ferry) | `unmath-make-ten` | Complements to ten |
| [10: Fair-share café](https://github.com/sealsbroken/unmath/wiki/Lesson-10-Fair-share-cafe) | `unmath-share-between-two` | Equal sharing between two |

All are registered by the existing `unmath::register` function. Direct use:

```rust
use unmath::curriculum::{Adventure, Lesson, NEXT_LESSONS};
let strategy = Adventure::new(Lesson::CaveTrain, config.clone());
// Or registry.create(Lesson::CaveTrain.id(), &config).
// NEXT_LESSONS lists the eight new lessons in teaching order.
```

Lessons 3–5 require five recognizable toys; 6–10 require ten. Set `total_objects`
accurately: a lesson pauses with a materials prompt if the configured supply is
insufficient. Each independent success advances one mission; toy-guided counts
never complete missions. The default ten-task session fits six uninterrupted
missions. Separate checkpoints retain progress if help uses the remaining tasks.
See each wiki page for its specific physical help stages, accepted answers, and
perception limits. The shared runtime is not a claim that the lessons teach the
same operation: subtraction removes toys, missing quantities add to a target,
comparison counts two supplies, and sharing deals alternately to two guests.

## Integration behavior

`unmath::ID` and `unmath::ADDITION_ID` are unchanged. Registration includes all ten
lessons. Choose a destination by saying `forest` / `moon` (`лес` / `луна`) or
by tapping 1 / 2. Destination choices never score as maths answers. Help requests
(`help`, `help me`, `I don't know`, `помоги`, `не знаю`) enter toy practice without
scoring a mistake. Break requests (`stop`, `break`, `I'm tired`, `стоп`, `перерыв`,
`я устал`, `я устала`) pause, including during toy setup.

Counting checks use full `Numbers` sequences, judged on the following `Heard`
event so the same response cannot answer the next question. Addition's independent
totals accept voice or taps; guided counting requires the full sequence. Stable
table totals gate toy tasks; table changes cancel pending toy counts. The strategy
cannot verify one touch per toy or pretend actions.

Three consecutive unsuccessful responses/timeouts or the task limit pause;
silence creates no failed task record. Repeat reissues a prompt; Skip records
an answer as skipped and enters practice, while skipping addition setup pauses.
Setup has no enforced timeout. Default ten tasks fits each uninterrupted adventure;
use shorter sessions and resume as needed.

Persist `SessionSummary.remember` via `apply_remembered`, then supply it in the
next `SessionConfig.extra`. Counting keeps the first independent success;
addition keeps completed picnic missions under `unmath.lesson2.picnic.v2`.
Old 21-fact scores are not reused. Each lesson also remembers its destination.
Completion clears its checkpoint. Only `PhaseKind::Finished` and a non-paused
summary mean completion; `correct` also includes guided responses.

## Verify

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Tests exercise strategy events and effects. Camera, microphone, and host app
builds need testing in the product. No `unengine` changes are required.

## License

This repository is licensed under the GNU Lesser General Public License,
version 3 or (at your option) any later version (`LGPL-3.0-or-later`).
See [LICENSE](LICENSE) and the incorporated [GPL terms](LICENSE.GPL).
