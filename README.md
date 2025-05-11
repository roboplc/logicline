# Logic Line - a logic processing engine

## Introduction

Processing system state and events has got the following problems:

* Visual programming languages are good for visualizing the flow of data, but
  they are extremely limited in terms using them for large and complex logic
  rule sets.

* Text-based programming languages are good for writing complex logic, but they
  lack the ability to visualize the flow of data, both for schema validation in
  development and for flow visualization in debugging or production.

* If the set contains a lot of rules, their notation must be as compact as
  possible, otherwise it becomes unreadable and hard to maintain.

Logic Line is a logic processing engine that combines the best of both worlds.
Inspired by [Ladder Logic](https://en.wikipedia.org/wiki/Ladder_logic),
[Monads](https://en.wikipedia.org/wiki/Monad_(functional_programming)) and some
others, it allows to write chains of logic rules as a regular Rust code, but
provides built-in tools for state recording, debugging and visualization.

## Architecture

The library has the following components:

**Rack** → **Processor** → **Line** → **Step** → **Action**

Where:

* **Rack** is a logic state either for the whole process or for a group of
  rules. In the first case, a [`global`] module can be used which contains a
  pre-defined process ladder instance. Rack can also act as an object factory
  for [`Processor`] instances, in case if created from a [`Rack`]
  instance, the processors have the common recording flag.

```rust
use logicline::Rack;

let ladder = Rack::new();
```

* **Processor** is a logic processor that processes a chain of rules. The
  processor creates `lines`. The processors can share common state between
  threads or other program parts.

```rust
use logicline::Rack;

let ladder = Rack::new();
let processor = ladder.processor();
```

* **Line** is an instance which is used to structure logic as a sequence of
  steps (monad-like objects). Each step includes a single or multiple action
  objects which are [`std::ops::FnOnce`] instances, wrapped into a structure
  which also contains the function name and its input parameter:

    (M a) → (a → M b) → (M b)

```rust,ignore
use logicline::{action, Rack};

let mut state = Rack::new().with_recording_enabled();
let mut processor = state.processor();

// Some fan state
let mut fan = false;

// A temperature sensor value
let temperature = 31.0;

processor
    // a sequence to turn on the fan on if the temperature is above 30 degrees
    .line("fan_on", temperature)
    .then(action!("temp_high", |t| (t > 30.0).then_some(())))
    .then(action!("fan_on", |()| {
        fan = true;
        Some(())
    }));
processor
    // a sequence to turn off the fan if the temperature is below 25 degrees
    .line("fan_off", temperature)
    .then(action!("temp_low", |t| (t < 25.0).then_some(())))
    .then(action!("fan_off", |()| {
        fan = false;
        Some(())
    }));

state.ingress(&mut processor);
```

When recorded, the state can be printed in a human-readable format:

```rust,ignore
println!("{}", state);
```

```ignore
fan_off: temp_low(31.0) ! -> fan_off
fan_on: temp_high(31.0) -> fan_on
```

Or serialized, e.g. to JSON for web visualization:

```rust,ignore
let serialized = serde_json::to_string_pretty(&state).unwrap();
```

```json
{
  "lines": {
    "fan_off": {
      "name": "fan_off",
      "steps": [
        {
          "name": "temp_low",
          "input": 31.0,
          "input_kind": "flow",
          "passed": false
        },
        {
          "name": "fan_off",
          "input": null,
          "input_kind": "flow",
          "passed": false
        }
      ]
    },
    "fan_on": {
      "name": "fan_on",
      "steps": [
        {
          "name": "temp_high",
          "input": 31.0,
          "input_kind": "flow",
          "passed": true
        },
        {
          "name": "fan_on",
          "input": null,
          "input_kind": "flow",
          "passed": true
        }
      ]
    }
  }
}
```

Technically, [`Step`] repeats certain [`std::option::Option`] functionality,
but adds features to record and visualize the flow of data:

* All lines and steps are named.

* Step inputs are recorded.

Additionally, [`Step`] brings logical `OR` operation, which allows to combine
two following closures in a single step. The closures must accept the same
input type and the input must implement `Clone` trait.

```rust,ignore
use logicline::{action, Rack};

// Here we use Env as a reference, so the `Clone` trait is not required for the
// structure itself.
#[derive(serde::Serialize)]
struct Env {
    temperature: f32,
    humidity: f32,
}

let env = Env {
    temperature: 25.0,
    humidity: 40.0,
};

let ladder = Rack::new();
let mut processor = ladder.processor();

let mut env_healthy = true;

processor
    .line("env_unhealthy", &env)
    .then_any(
        action!("temp_high", |env: &Env| (env.temperature > 30.0).then_some(())),
        action!("humidity_high", |env: &Env| (env.humidity > 60.0).then_some(())),
    )
    .then(action!("set_unhealthy", |()| {
        env_healthy = false;
        Some(())
    }));

```

The same example without the structure, where the second closure accepts an
external variable which is recorded as an input:

```rust
use logicline::{action, Rack};

let ladder = Rack::new();
let mut processor = ladder.processor();

let temperature = 25.0;
let humidity = 40.0;

let mut env_healthy = false;

processor
    // the temperature sensor value
    .line("env_healthy", temperature)
    .then_any(
        action!("temp_ok", |t| (t < 30.0).then_some(())),
        // skip the chain input, the method `with_recorded_input` is used to
        // record the real closure input
        action!("humidity_ok", |_| (humidity < 60.0).then_some(()))
            .with_recorded_input(&humidity)
    )
    .then(action!("set_healthy", |()| {
        env_healthy = true;
        Some(())
    }));
```

As Rust has got no exception, the way to break the chain is the same as for the
traditional combinators: return `None`.

## Recording

By default `recording` feature is enabled. When disabled, no line state is
recorded, certain recording-specific methods are not available.

The state recording can be also enabled/disabled in runtime. By default, the
runtime recording is disabled.

## Ordering

In a classic logic rack, it is supposed that the order of the lines is
unpredictable, as classic logic programming languages copy hardware relay-rack
logic.

In this library, the order of the lines is defined by the developer however it
is strongly recommended to avoid creating conflicting lines to keep the overall
state clear and consistent.

The recorded lines are placed into a [`std::collections::BTreeMap`] and
automatically sorted by their names.

## Performance

When the recording is disabled (either feature or the runtime state/processor
flag), the logic lines bring almost no overhead in comparison to the
traditional combinators, such as similar methods of [`std::option::Option`] and
[`std::result::Result`].

## Data visualization

The crate `exporter` feature provides a built-in exporter (HTTP server) for
[`global`] process state.

```rust,ignore
logicline::global::install_exporter().unwrap();
```

The web server binds to the port `9001` and provides the global rack state
snapshots in JSON format at the `/state` endpoint. The server can be also
configured to bind a specific address using [`global::install_exporter_on`]
method.

The snapshots can be visualized using
[`logicline-view`](https://github.com/roboplc/logicline/tree/main/logicline-view)
TypeScript library which is a part of this project.

In case if a feature `exporter-ui` is enabled, the built-in web server also
provides a basic interface to visualize the global state snapshots. The
interface is available at the root (`http://host:9001/`) endpoint.

For custom programs, state snapshots can be serialized to any
`serde`-compatible format and pushed/pulled in any required way.

In case of periodic processing, such as local/remote context/sensor analysis in
traditional [PLC](https://en.wikipedia.org/wiki/Programmable_logic_controller)
logic state is update on every iteration and the visualization always contains
all the lines programmed.

In case of event-processing model, it is recommended to send dummy events at
the program start to fill the logic state with the initial chain values.

## Locking safety

By default, the crate (both the server and the client modules) uses
[parking_lot](https://crates.io/crates/parking_lot) for locking. For real-time
applications, the following features are available:

* `locking-rt` - use [parking_lot_rt](https://crates.io/crates/parking_lot_rt)
  crate which is a spin-free fork of parking_lot.

* `locking-rt-safe` - use [rtsc](https://crates.io/crates/rtsc)
  priority-inheritance locking, which is not affected by priority inversion
  (Linux only).

Note: to switch locking policy, disable the crate default features.

