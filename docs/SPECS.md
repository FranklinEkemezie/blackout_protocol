# Rust Project 1: Blackout Protocol

**Type:** Terminal-based tactical simulation  
**Language:** Rust  
**Difficulty:** Intermediate to advanced  
**Scope:** The Rust concepts covered in Chapters 1–4  
**Dependencies:** Standard library only for the initial version

## 1. The Scenario

A remote research facility has suffered a catastrophic power failure. Its automated security system has locked down the facility, several critical systems are offline, and the backup generator is running out of fuel.

You are the emergency response operator.

Your job is to restore power, keep essential systems running, and evacuate the facility before the generator runs out of fuel.

You cannot restore everything at once. Every action consumes time, some actions consume fuel, and some decisions have consequences for other systems.

**Your objective is to complete the evacuation before the facility's emergency power is exhausted.**

The simulation must be playable from beginning to end through a terminal interface.

---

## 2. The Simulation World

The facility has four critical systems:

| System | Initial condition | Importance |
|---|---|---|
| Life Support | Offline | Must be restored to protect the occupants. |
| Communications | Offline | Must be restored to contact the external rescue team. |
| Security | Online | Must remain operational to control access to restricted areas. |
| Evacuation Lift | Offline | Must be restored before evacuation can begin. |

The facility has the following initial resources:

- **Fuel:** 100 units.
- **Emergency Power:** 100 units.
- **Occupants:** 12 people.
- **Evacuated Occupants:** 0 people.
- **Elapsed Time:** 0 minutes.

These values are fixed at the beginning of every new game.

The simulation proceeds in discrete turns. Each successful operational action advances the elapsed time by a specified amount.

### System states

Every system has one of three states:

- `Online`
- `Offline`
- `Damaged`

For the initial version, represent these states using integer constants rather than defining a custom enumeration.

For example, you could assign a different integer to each state and use `match` to interpret it.

The initial version does not use `struct`, custom `enum`, or `Vec<T>`. The second iteration deliberately replaces the integer constants with a `SystemStatus` enum and introduces the structs described in Section 17.

---

## 3. The Main Menu

When the program starts, display the following menu:

```text
====================================
       BLACKOUT PROTOCOL
====================================

1. Start New Mission
2. View Mission Briefing
3. View Instructions
4. Exit

Select an option:
```

Requirements:

1. Accept input from the terminal.
2. Reject invalid menu selections without crashing.
3. Display the appropriate screen for each valid selection.
4. Return to the menu when the user finishes viewing the briefing or instructions.
5. Exit only when the user selects the exit option.

Starting a new mission initializes all game variables to their initial values.

For this project, restarting the program is not necessary to start a new mission.

---

## 4. The Mission Interface

During a mission, display a status report before presenting the available actions.

For example:

```text
====================================
           MISSION STATUS
====================================

Elapsed Time:       12 minutes
Fuel:               76 units
Emergency Power:    82 units

Life Support:       ONLINE
Communications:     OFFLINE
Security:           ONLINE
Evacuation Lift:    OFFLINE

Occupants Remaining: 12
Occupants Evacuated: 0

====================================
```

The displayed values must reflect the current game state.

You must not hardcode the status report. Each value should come from the variables that track the simulation.

After displaying the report, show the available actions.

```text
1. Inspect Facility
2. Restore a System
3. Repair a System
4. Manage Emergency Power
5. Evacuate Occupants
6. End Mission
```

The user should be able to select an action, receive its result, and return to the mission interface.

The mission interface repeats until the mission ends or the user explicitly chooses to end it.

---

## 5. Action One: Inspect the Facility

Inspection does not consume fuel or emergency power, but it consumes 2 minutes.

The inspection report must show:

- The current state of every critical system.
- The current fuel and emergency power levels.
- The number of occupants remaining.
- Whether communications have been restored.
- Whether evacuation is currently possible.

The report must be generated from the current simulation state.

For example, evacuation is possible only when both communications and the evacuation lift are online.

If either system is offline or damaged, the report must say that evacuation is unavailable and explain which requirement has not been satisfied.

Inspection is repeatable.

---

## 6. Action Two: Restore a System

The operator can attempt to restore an offline system.

Display a numbered list of the four systems and allow the operator to select one.

Use the following restoration rules:

| System | Fuel consumed | Emergency power consumed | Time consumed |
|---|---:|---:|---:|
| Life Support | 8 | 12 | 5 minutes |
| Communications | 5 | 8 | 3 minutes |
| Security | 6 | 10 | 4 minutes |
| Evacuation Lift | 10 | 15 | 6 minutes |

These values are fixed for the initial version.

### Restoration rules

1. A system can be restored only if it is offline.
2. A damaged system must be repaired before it can be restored.
3. An online system cannot be restored again.
4. The operator must have enough fuel and emergency power to complete the action.
5. If the operator cannot afford an action, the system state and all resource values must remain unchanged.
6. When restoration succeeds, deduct the required resources and advance the elapsed time.
7. After a successful restoration, change the system's state to online.

For example, attempting to restore Communications when only 3 fuel units remain must not consume those 3 units.

The operation either succeeds completely or fails without partially changing the simulation.

### Additional requirement

The restoration logic must be implemented using functions.

Do not put every restoration rule directly inside the main menu loop.

---

## 7. Action Three: Repair a System

A damaged system cannot be restored until it has been repaired.

Repairing a system has the following costs:

| Resource | Cost |
|---|---:|
| Fuel | 4 units |
| Emergency Power | 5 units |
| Time | 3 minutes |

### Repair rules

1. Only damaged systems can be repaired.
2. Offline systems do not require repairs.
3. Online systems cannot be repaired.
4. The operator must have enough resources.
5. A successful repair changes the system's state from damaged to offline.
6. Repairing a system does not automatically bring it online.

The operator must subsequently perform the restoration action.

### The fault mechanic

Introduce a limited fault mechanic.

Whenever an operator attempts to restore a system, calculate the following condition using the current elapsed time:

```text
fault_score = (elapsed_time + fuel_remaining) % 7
```

If `fault_score == 0`, the restoration attempt triggers a fault.

When a fault occurs:

- The selected system becomes damaged.
- The restoration costs are not deducted.
- The elapsed time does not advance.
- The operator must repair the system before trying again.

If the condition is not satisfied, the restoration proceeds normally, subject to the other restoration rules.

**Important:** Check whether the restoration is affordable before triggering the fault mechanic. An unaffordable action must never cause a fault.

This is a deterministic rule, not a random event. The same game state must produce the same result.

---

## 8. Action Four: Manage Emergency Power

The facility has an emergency generator with a limited fuel supply.

The operator can perform one of two actions:

### A. Conserve Power

Conserving power reduces the facility's energy consumption.

- Emergency power increases by 10 units.
- Fuel decreases by 5 units.
- Elapsed time increases by 2 minutes.

### B. Emergency Shutdown

The operator can shut down nonessential systems to reduce power consumption.

For this simulation, an emergency shutdown has the following effect:

- Emergency power increases by 20 units.
- Fuel decreases by 10 units.
- Elapsed time increases by 4 minutes.
- Security becomes offline if it was online.

The shutdown cannot be reversed automatically. Security must be restored through the normal restoration process.

### Resource limits

Emergency power cannot exceed 100 units.

Fuel cannot become negative.

If an action would exceed the emergency power limit, cap the resulting power at 100.

For example, if emergency power is 95 and conserving power would add 10 units, the final value must be 100, not 105.

However, the action still consumes the full 5 fuel units and advances time by 2 minutes.

If insufficient fuel is available, reject the action without changing any state.

---

## 9. Action Five: Evacuate Occupants

Evacuation is the final objective.

The operator can evacuate occupants only when:

- Communications is online.
- The Evacuation Lift is online.
- Life Support is online.
- Security is online.
- At least 5 units of fuel remain.
- At least 10 units of emergency power remain.

If any requirement is unsatisfied, reject the evacuation attempt and explain why.

When evacuation is available, the operator can evacuate up to 4 occupants per operation.

Each evacuation operation:

- Consumes 5 fuel units.
- Consumes 10 emergency power units.
- Advances time by 3 minutes.
- Evacuates up to 4 remaining occupants.

If only 2 occupants remain, evacuate those 2 rather than attempting to evacuate 4.

The operator must have enough resources to complete an evacuation operation.

If the required resources are unavailable, reject the operation without changing any values.

The mission is successful when all 12 occupants have been evacuated.

---

## 10. Mission Failure Conditions

The mission fails immediately if any of the following conditions occurs:

1. Fuel reaches zero before the evacuation is complete.
2. Emergency power reaches zero before the evacuation is complete.
3. Elapsed time reaches 30 minutes before the evacuation is complete.
4. Life Support is damaged at the end of an action.
5. The operator explicitly selects `End Mission`.

When the mission fails, display a debriefing report containing:

- Final elapsed time.
- Final fuel level.
- Final emergency power level.
- Number of occupants evacuated.
- Number of occupants remaining.
- Final state of each system.
- The reason the mission failed.

Do not continue accepting mission actions after failure.

**Evaluation order:** After a successful action, update the simulation state and check the failure conditions before accepting another action. If all occupants are evacuated by that action, the mission succeeds even if the action also reaches the time limit.

---

## 11. Mission Scoring

A successful mission receives a score out of 100.

Calculate the score using the following rules:

| Achievement | Points |
|---|---:|
| Evacuate all 12 occupants | 40 |
| Complete evacuation in 20 minutes or less | 20 |
| Finish with at least 30 fuel units | 15 |
| Finish with at least 30 emergency power units | 15 |
| Complete the mission without a system fault | 10 |

The score is the sum of all applicable achievements.

A failed mission receives no completion bonus, but its debriefing report must still display the number of occupants evacuated and the resources remaining.

At the end of a successful mission, display the score and a performance classification:

| Score | Classification |
|---|---|
| 90–100 | Exceptional |
| 70–89 | Successful |
| 50–69 | Marginal |
| 0–49 | Poor |

The classification is based on the final score, not on the number of actions performed.

---

## 12. Input Validation

The program must never crash merely because the operator enters invalid input.

Examples of invalid input include:

```text
abc
-1
99
```

For menu selections, reject values that are not valid options.

For numeric input:

1. Read the input into a mutable `String`.
2. Remove surrounding whitespace using `.trim()`.
3. Parse the input into the required numeric type.
4. Use `match` to distinguish successful parsing from failure.
5. Display an appropriate error message when parsing fails.
6. Return to the relevant menu without changing the simulation state.

An invalid input must not consume fuel, consume emergency power, advance time, or change a system's state.

---

## 13. Technical Constraints

Implement the initial version using only concepts covered in Chapters 1–4 of The Rust Programming Language.

> **Scope of the restrictions:** the restrictions below describe the initial version only. The second iteration (Section 17) explicitly relaxes the struct and enum restrictions as part of the learning progression. All other restrictions, including standard-library-only dependencies and no unsafe Rust, continue to apply.

### Required language features

Use:

- Variables, mutability, constants, and shadowing.
- Integer, floating-point, Boolean, and character types where appropriate.
- Tuples and fixed-size arrays.
- Functions with parameters and return values.
- `if`, `else`, and `match`.
- `loop`, `while`, and `for`.
- `break` and `continue`.
- `String` and `&str`.
- Ownership, moves, and borrowing.
- Immutable and mutable references.
- String slices and array slices.
- `Result` handling for terminal input.

### Restrictions

The initial version must not use:

- `struct`.
- Custom `enum` definitions.
- `Vec<T>`.
- Hash maps or other collections not covered in Chapters 1–4.
- File I/O.
- External crates.
- Global mutable state.
- Unsafe Rust.

You may use the standard library.

You may use integer constants to represent system states, but the code must use descriptive constant names rather than unexplained numeric literals.

For example, prefer:

```rust
const SYSTEM_ONLINE: u8 = 1;
const SYSTEM_OFFLINE: u8 = 2;
const SYSTEM_DAMAGED: u8 = 3;
```

over scattering `1`, `2`, and `3` throughout the program.

Choose suitable types for resource quantities, elapsed time, system states, and occupant counts.

---

## 14. Program Structure

Organize the program into functions with clear responsibilities.

Your implementation should contain functions serving these purposes:

1. Display the main menu.
2. Display the mission briefing.
3. Initialize a new mission.
4. Display the mission status.
5. Read and validate user input.
6. Inspect the facility.
7. Restore a selected system.
8. Repair a selected system.
9. Manage emergency power.
10. Evacuate occupants.
11. Check mission failure conditions.
12. Calculate the final score.
13. Display the debriefing report.

You may introduce additional functions when necessary.

Do not implement the entire simulation inside `main()`.

### State management

Because this version does not use custom structures, you must decide how to represent and pass the mission state using the available language features.

For example, you can use fixed-size arrays for the four system states and separate variables for the resources.

Pass references to functions that need to inspect state without taking ownership. Use mutable references where a function must update the state.

This is an intentional part of the exercise.

You should discover which functions need to modify the mission state and which only need to read it.

---

## 15. Testing Requirements

Test the following cases manually.

### Main menu

- Start a new mission.
- View the briefing.
- View the instructions.
- Enter invalid input.
- Exit the application.

### System restoration

- Restore an offline system successfully.
- Attempt to restore an already-online system.
- Attempt to restore a damaged system.
- Attempt restoration with insufficient fuel.
- Attempt restoration with insufficient emergency power.
- Trigger a fault and verify that no restoration resources are deducted.
- Repair a damaged system and restore it afterward.

### Resource management

- Conserve power when fuel is sufficient.
- Attempt to conserve power when fuel is insufficient.
- Conserve power when emergency power is already near its maximum.
- Perform an emergency shutdown while Security is online.
- Verify that Security becomes offline after shutdown.

### Evacuation

- Attempt evacuation before the required systems are online.
- Attempt evacuation with insufficient resources.
- Evacuate 4 occupants.
- Evacuate the remaining occupants.
- Verify that the mission succeeds when all occupants are evacuated.

### Mission termination

- Exhaust the fuel supply.
- Exhaust emergency power.
- Reach the time limit.
- Trigger a Life Support fault.
- End the mission manually.
- Verify that the debriefing report displays the correct final state.

### Scoring

- Complete a mission meeting all scoring criteria.
- Complete a mission meeting only some scoring criteria.
- Verify that failed missions do not receive a successful completion score.

---

## 16. Definition of Done

The project is complete when:

- The program compiles and runs through Cargo.
- The operator can navigate the menus.
- All four systems have functioning state transitions.
- All resource costs are enforced.
- Invalid input does not crash the application.
- No resource value becomes negative.
- Emergency power never exceeds 100.
- The fault mechanic behaves deterministically.
- The evacuation requirements are enforced.
- Success and failure conditions terminate the mission correctly.
- The final report and score reflect the actual mission state.
- The program is divided into meaningful functions rather than one enormous `main()`.

Use `cargo check` frequently, run the program with `cargo run`, and format your code with `cargo fmt`.

Do not begin by writing the entire program at once. Implement and test one subsystem at a time.

**The challenge is not simply to make the simulation work. It is to design the program so that the rules remain consistent as the state changes, while using Rust's ownership and borrowing model correctly.**

---

## 17. Second Iteration: Structured Refactor

The second iteration keeps the simulation rules from Sections 1–16 and refactors the implementation to introduce the next set of Rust concepts. This section records the intentional differences so that they do not conflict with the initial-version restrictions in Section 13.

### Language features introduced

The second iteration may use:

- `struct` definitions and named fields.
- Custom `enum` definitions and `match` on enum variants.
- Implementation blocks (`impl`) for associated functions and methods.
- `if let` and `let ... else` pattern matching.
- `u32` for resource counters and elapsed-time arithmetic.
- `min`/`max` and saturating arithmetic for resource limits.
- Standard-library helpers such as `String`, `std::format!`, and `std::io::Write`.

The project remains standard-library only.

### Structural changes

- `SystemState` (an integer alias with `SYSTEM_*` constants) becomes `SystemStatus`, an enum with `Online`, `Offline`, and `Damaged` variants.
- The menu option alias is renamed from `Option` to `MenuOption` so it does not shadow the standard-library `Option<T>`.
- The `Resources`, `SystemState`, and `FacilityStatus` tuples become the `Resources`, `SystemState`, and `Facility` structs.
- Facility state helpers become methods on `Facility`; the status label helper becomes a method on `SystemStatus`.
- UI helpers are grouped in an implementation block for `Ui`.
- Main-menu helpers are grouped in an implementation block for `MainMenu`.
- Mission flow, actions, failure checks, and scoring are grouped in an implementation block for `Mission`.
- Resource quantities and remaining time use `u32`; remaining occupants use `u8`.

### Behavioural clarifications

The rule set in Sections 4–12 is unchanged. In particular, the second iteration must still:

- Check affordability before running the fault mechanic.
- Leave all state unchanged when an action is rejected.
- Make Emergency Shutdown put Security into the `Offline` state, not `Damaged`.
- Make Life Support restoration cost 5 minutes and leave Life Support `Online`.
- Evaluate success before failure after the final action.
- Award the fault-free scoring bonus only when no fault occurred.

### Tests

The automated tests live in `tests/spec.rs`. They include the source directly so deterministic private helpers can be exercised without modifying the implementation. Terminal-only behaviour remains manually testable with `cargo run`.

Known deviations from this specification at the time of writing are recorded in `docs/BUGS.md`.
