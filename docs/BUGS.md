# Known Bugs — Current

This file is the offline record of specification deviations that remain after the latest refactor. Each section links to the corresponding GitHub issue.

At the time of writing, `cargo test --test flows` reports **15 passed, 8 failed**. The failures correspond to issues 3, 5, 8, and 9 below. Issues 6 and 7 are interactive/UI gaps that are not directly covered by the current flow tests.

| # | Severity | Bug | Issue |
|---|---|---|---|
| 3 | Critical | Life Support restoration uses the wrong power and time costs | [#3](https://github.com/FranklinEkemezie/blackout_protocol/issues/3) |
| 5 | Critical | Actions do not validate affordability or system state before mutating | [#5](https://github.com/FranklinEkemezie/blackout_protocol/issues/5) |
| 6 | Medium | Fault-free scoring bonus is always awarded | [#6](https://github.com/FranklinEkemezie/blackout_protocol/issues/6) |
| 7 | Low | Terminal messages and end-of-input handling are inaccurate | [#7](https://github.com/FranklinEkemezie/blackout_protocol/issues/7) |
| 8 | High | Evacuation Lift checks and repair list target the wrong systems | [#8](https://github.com/FranklinEkemezie/blackout_protocol/issues/8) |
| 9 | High | Mission starts with 100 minutes instead of 30 | [#9](https://github.com/FranklinEkemezie/blackout_protocol/issues/9) |

---

## 3. Life Support restoration uses the wrong power and time costs

**Issue:** [#3](https://github.com/FranklinEkemezie/blackout_protocol/issues/3)

Life Support now correctly returns to `Online`, but `Mission::restore_system` deducts:

```rust
facility.resources().decrease_by(8, 8, 2);
```

SPECS section 6 requires **8 fuel, 12 emergency power, and 5 minutes**. The current code undercharges power by 4 and time by 3 minutes.

This also changes mission scoring: a run that restores Life Support, Communications, and the Evacuation Lift before three evacuations finishes at 20 elapsed minutes instead of the specified 23, which wrongly earns the 20-minute bonus.

### Failing tests

- `restoring_life_support_uses_the_specified_costs`
- `successful_mission_score_matches_the_specified_costs`

---

## 5. Actions do not validate affordability or system state before mutating

**Issue:** [#5](https://github.com/FranklinEkemezie/blackout_protocol/issues/5)

Actions still mutate resources before proving the player can afford them:

- `Mission::restore_system` runs the fault check before affordability and does not verify that fuel, power, and time are sufficient.
- `Mission::repair_system` deducts 4 fuel, 5 power, and 3 minutes without checking availability.
- `Mission::manage_emergency_power` performs Conserve Power and Emergency Shutdown without checking fuel.

System state is also not validated before acting. Restoring an already-online system reaches `System::restore`, which panics instead of rejecting the action without a state change. The fault branch similarly runs before confirming the selected system is offline.

SPECS sections 6-8 require rejection with no state change when resources are insufficient, and affordability must be checked before the fault mechanic.

### Failing test

- `restoring_an_already_online_system_is_rejected_without_crashing`

---

## 6. Fault-free scoring bonus is always awarded

**Issue:** [#6](https://github.com/FranklinEkemezie/blackout_protocol/issues/6)

`Mission::calculate_mission_score` always adds the 10-point `COMPLETE_MISSION_WITHOUT_SYSTEM_FAULT` bonus. The code still contains a `TODO` noting that faults are not tracked.

SPECS section 11 awards the bonus only when no system fault occurred, so a deterministic fault counter is still needed in mission state.

---

## 7. Terminal messages and end-of-input handling are inaccurate

**Issue:** [#7](https://github.com/FranklinEkemezie/blackout_protocol/issues/7)

The Emergency Shutdown confirmation now correctly says 4 minutes. The remaining issues are:

- Evacuation always prints `Evacuated 4 persons successfully!`, even when fewer occupants remain.
- Restore System uses a letter table (`[L]`, `[C]`, `[S]`, `[E]`) instead of the numbered list required by SPECS section 6.
- At end-of-input, `Ui::prompt_user` returns an empty string and `prompt_user_option` loops forever printing `Invalid input` instead of terminating.
- The restore table heading says `MenuOption`, leaking an implementation alias into the UI.

---

## 8. Evacuation Lift checks and repair list target the wrong systems

**Issue:** [#8](https://github.com/FranklinEkemezie/blackout_protocol/issues/8)

Two system checks were copied from other systems:

- `Mission::evacuate_occupants` checks Communications twice. The second check should check Evacuation Lift, so evacuation can proceed while the lift is offline.
- `Mission::repair_system` checks Security twice when building the damaged-systems list. A damaged Evacuation Lift is never listed or selectable for repair.

### Failing tests

- `evacuation_is_rejected_when_the_evacuation_lift_is_offline`
- `repair_menu_lists_a_damaged_evacuation_lift`

---

## 9. Mission starts with 100 minutes instead of 30

**Issue:** [#9](https://github.com/FranklinEkemezie/blackout_protocol/issues/9)

`INITIAL_FACILITY` initializes the time resource with 100 units. SPECS requires a 30-minute mission limit.

Observed effects:

- The mission menu starts at `100 minutes remaining`.
- An inspection reports `98 minutes` remaining instead of `28`.
- The time-limit failure never triggers at 30 minutes.

### Failing tests

- `mission_clock_starts_with_thirty_minutes`
- `inspection_consumes_two_minutes_and_reports_readiness`
- `mission_fails_when_the_thirty_minute_limit_is_reached`

---

## Reproducing the current failures

```bash
cargo test --test flows
```
