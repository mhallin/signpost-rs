# Signpost library for macOS

[Signposts](https://developer.apple.com/documentation/os/logging/recording_performance_data)
are a low-overhead way of measuring performance of tasks and algorithms on macOS
and iOS.

This library exposes a Rust interface to the Signposts API which automatically
turns itself off on unsupported platforms.

## Usage

Use a combination of events and intervals to measure times of algorithms and
tasks in your application when running under Instruments. The use of macros
allow construction of null-terminated C strings at compile time rather than at
runtime.

```rust
use signpost::{OsLog, const_poi_logger};

static LOGGER: OsLog = const_poi_logger!("com.yourcompany.project");

fn myalgorithm() {
    // Create a signpost interval for your function. The interval ends
    // when the variable goes out of scope.
    let _interval = signpost::begin_interval!(
        LOGGER,
        /* Interval ID */ 1,
        /* Interval name */ "My Algorithm"
    );

    if condition {
        // Emit a single event
        signpost::emit_event!(
            LOGGER,
            /* Event ID */ 2,
            /* Event name */ "Condition happened"
        );
    }
}
```

## Disabling the signposts

Enable the `disable-signposts` feature to make the logging function no-ops even
on macOS/iOS.
