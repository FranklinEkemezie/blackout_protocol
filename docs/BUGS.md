# Known Bugs — Second Iteration

This file is the offline record of specification deviations found after the structured refactor. Each section links to the corresponding GitHub issue.

At the time of writing, `cargo test` reports **20 passed, 6 failed**. The failures correspond to issues 1, 3, and 4 below. The other issues are interactive behaviours that the current test suite cannot reach without refactoring the action functions or adding process-level integration tests.

| # | Severity | Bug | Issue |
|---|---|---|---|
| 1 | Critical | Mission failure check is inverted for Life Support | [#1](https://github.com/FrankEkemezie/blackout_protocol/issues/1) |
| 2 | High | Mission success is checked after failure | [#2](https://github.com/FrankEkemezie/blackout_protocol/issues/2) |
| 3 | Critical | Life Support restoration leaves the system offline | [#3](https://github.com/FrankEkemezie/blackout_protocol/issues/3) |
| 4 | High | Security repair and shutdown transitions are wrong | [#4](https://github.com/FrankEkemezie/blackout_protocol/issues/4) |
| 5 | Critical | Actions do not enforce affordability before mutating state | [#5](https://github.com/FrankEkemezie/blackout_protocol/issues/5) |
| 6 | Medium | Fault-free scoring bonus is always awarded | [#6](https://github.com/FrankEkemezie/blackout_protocol/issues/6) |
| 7 | Low | Terminal messages and end-of-input handling are inaccurate | [#7](https://github.com/FrankEkemezie/blackout_protocol/issues/7) |

---

## 1. Mission failure check is inverted for Life Support

**Issue:** [#1](https://github.com/FrankEkemezie/blackout_protocol/issues/1)

`Mission::is_mission_failed` (src/main.rs:1067) uses:

```rust
let SystemStatus::Damaged = systems_state.life_support else { return true; };
```

`let ... else` runs the `else` branch when the pattern does **not** match. As a result, the mission is treated as failed whenever Life Support is `Online` or `Offline`, and is allowed to continue when Life Support is `Damaged` — the opposite of SPECS section 10.

`Mission::mission_failed_reason` (src/main.rs:1089) has the same inversion, so it returns `"Life support is broken"` when Life Support is not damaged and returns an empty string when it is damaged.

### Reproduction

```bash
printf '1\n1\ny\n' | cargo run
```

After inspecting the facility, the mission immediately ends with `LIFE SUPPORT IS BROKEN`, even though the debrief shows `Life Support: Offline`.

### Failing tests

- `new_mission_is_not_failed`
- `mission_failure_reason_is_empty_when_no_failure_condition_holds`
- `mission_fails_when_life_support_is_damaged`

---

## 2. Mission success is checked after failure

**Issue:** [#2](https://github.com/FrankEkemezie/blackout_protocol/issues/2)

`Mission::start_new` (src/main.rs:682-698) checks `is_mission_failed` before `is_mission_successful`. SPECS section 10 requires the opposite: if the final action evacuates the last occupant, the mission succeeds even if that action also reaches the time limit.

A final evacuation that exhausts time, fuel, or power is currently reported as a failure without a score.

---

## 3. Life Support restoration leaves the system offline

**Issue:** [#3](https://github.com/FrankEkemezie/blackout_protocol/issues/3)

`Facility::restore_life_support` (src/main.rs:581) assigns `SystemStatus::Offline` instead of `SystemStatus::Online`. The action message claims the system is online, but the state remains offline.

The Life Support cost block in `Mission::restore_system` (src/main.rs:827-835) also advances time by 3 minutes instead of the specified 5 minutes.

Because evacuation requires Life Support to be online, the mission is currently unwinnable.

### Reproduction

```bash
printf '1\n2\nL\n' | cargo run
```

Observed:

```text
[INFO] The life support system is now restored and fully online!

Elapsed Time:       3 minutes
Life Support:       Offline
```

Expected: `Elapsed Time: 5 minutes` and `Life Support: Online`.

### Failing test

- `offline_systems_restore_online`

---

## 4. Security repair and shutdown transitions are wrong

**Issue:** [#4](https://github.com/FrankEkemezie/blackout_protocol/issues/4)

`Facility::repair_security` (src/main.rs:629) checks and repairs `evacuation_lift` instead of `security`. Repairing Security therefore panics unless the Evacuation Lift is damaged, and otherwise silently repairs the lift.

`Facility::shutdown_security` (src/main.rs:637) sets Security to `Damaged`. SPECS section 8 says Emergency Shutdown makes Security `Offline`, after which it is restored through the normal restoration process. Setting it to `Damaged` incorrectly requires a repair first.

### Failing tests

- `damaged_security_repairs_to_offline`
- `emergency_shutdown_takes_security_offline`

---

## 5. Actions do not enforce affordability before mutating state

**Issue:** [#5](https://github.com/FrankEkemezie/blackout_protocol/issues/5)

Three actions mutate resources before proving the player can afford them:

- `Mission::restore_system` (src/main.rs:762) checks the fault condition before affordability and never checks fuel/power before deducting. Saturating subtraction clamps resources to zero, then the system is still restored.
- `Mission::repair_system` (src/main.rs:877) deducts 4 fuel, 5 power, and 3 minutes without checking availability.
- `Mission::manage_emergency_power` (src/main.rs:954) performs Conserve Power and Emergency Shutdown without checking fuel.

The fault branch in `Mission::restore_system` also runs before validating that the selected system is offline, so a fault can damage an already-online system.

SPECS sections 6-8 require the action to be rejected with no state change when resources are insufficient, and require affordability to be checked before the fault mechanic.

These paths are not covered by the current automated suite because they are embedded in interactive terminal functions.

---

## 6. Fault-free scoring bonus is always awarded

**Issue:** [#6](https://github.com/FrankEkemezie/blackout_protocol/issues/6)

`Mission::calculate_mission_score` (src/main.rs:1138) always adds the 10-point `COMPLETE_MISSION_WITHOUT_SYSTEM_FAULT` bonus. The code contains a `TODO` noting that faults are not tracked.

SPECS section 11 awards the bonus only when no system fault occurred. A deterministic fault counter needs to be added to the mission state.

---

## 7. Terminal messages and end-of-input handling are inaccurate

**Issue:** [#7](https://github.com/FrankEkemezie/blackout_protocol/issues/7)

- Emergency Shutdown confirmation says `10 fuel units and 2 minutes`, but the action advances 4 minutes (src/main.rs:987).
- Evacuation always prints `Evacuated 4 persons successfully!` even when fewer occupants remain (src/main.rs:1044-1047).
- Restore System uses a letter table (`[L]`, `[C]`, `[S]`, `[E]`) rather than the numbered list required by SPECS section 6 (src/main.rs:739-756).
- At end-of-input, `Ui::prompt_user` (src/main.rs:265-283) returns an empty string and `prompt_user_option` loops forever printing `Invalid input` instead of terminating.
- The restore table heading says `MenuOption`, leaking an implementation alias into the UI.

---

## Manual verification notes

The following commands were run against the current working tree:

```bash
printf '1\n1\ny\n' | timeout 3 cargo run
printf '1\n2\nL\n' | timeout 3 cargo run
```

The first command demonstrates issue 1. The second demonstrates issue 3.
