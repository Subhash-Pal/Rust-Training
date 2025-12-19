/*
```
Build a state machine using:

A custom enum to represent states.
Pattern matching (match) to transition between states based on events.
Demonstrate how Rust’s enums are more powerful than C++ enums (they can carry data!).
We’ll model a simple door: it can be Locked, Closed, or Open.
Events: Unlock, Lock, Open, Close.
```

*/

#[derive(Debug, PartialEq, Clone, Copy)]
enum DoorState {
    Locked,
    Closed,
    Open,
}

#[derive(Debug, Clone, Copy)]
enum DoorEvent {
    Unlock,
    Lock,
    Open,
    Close,
}

fn next_state(current: DoorState, event: DoorEvent) -> DoorState {
    match (current, event) {
        // From Locked
        (DoorState::Locked, DoorEvent::Unlock) => DoorState::Closed,
        (DoorState::Locked, DoorEvent::Lock) => DoorState::Locked,
        (DoorState::Locked, DoorEvent::Open) => DoorState::Locked,   // Cannot open when locked
        (DoorState::Locked, DoorEvent::Close) => DoorState::Locked,

        // From Closed (unlocked but shut)
        (DoorState::Closed, DoorEvent::Unlock) => DoorState::Closed,
        (DoorState::Closed, DoorEvent::Lock) => DoorState::Locked,
        (DoorState::Closed, DoorEvent::Open) => DoorState::Open,
        (DoorState::Closed, DoorEvent::Close) => DoorState::Closed,

        // From Open
        (DoorState::Open, DoorEvent::Unlock) => DoorState::Open,
        (DoorState::Open, DoorEvent::Lock) => DoorState::Open,        // Cannot lock when open
        (DoorState::Open, DoorEvent::Open) => DoorState::Open,
        (DoorState::Open, DoorEvent::Close) => DoorState::Closed,
    }
}

fn main() {
    let mut state = DoorState::Locked;
    println!("🚪 Door State Machine");
    println!("Initial state: {:?}", state);

    let events = [
        DoorEvent::Unlock,
        DoorEvent::Open,
        DoorEvent::Close,
        DoorEvent::Lock,
    ];

    for &event in &events {
        state = next_state(state, event);
        println!(" → Processed {:?} → New state: {:?}", event, state);
    }

    // Extra test: try to open while locked
    println!("\n🧪 Test: Attempt to OPEN while LOCKED");
    let result = next_state(DoorState::Locked, DoorEvent::Open);
    println!("Resulting state: {:?}", result);
}