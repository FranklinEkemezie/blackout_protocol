// Include the binary source in this test crate so its private, deterministic
// state helpers can be checked without changing src/main.rs.
mod app {
    include!("../src/main.rs");

    #[cfg(test)]
    mod spec_tests {
    use super::*;

    fn state() -> FacilityStatus {
        INITIAL_FACILITY_STATUS
    }

    #[test]
    fn new_mission_has_the_specified_initial_state() {
        let s = state();
        assert_eq!(s.0.0, 100, "fuel");
        assert_eq!(s.0.1, 100, "emergency power");
        assert_eq!(facility_status_get_time_elapsed(&s), 0);
        assert_eq!(s.1, 12, "occupants remaining");
        assert_eq!(facility_status_get_evacuated_occupants(&s), 0);
        assert_eq!(s.2, (SYSTEM_OFFLINE, SYSTEM_OFFLINE, SYSTEM_ONLINE, SYSTEM_OFFLINE));
    }

    #[test]
    fn evacuation_requires_all_four_systems_and_minimum_resources() {
        let mut s = state();
        assert!(!facility_status_is_evacuation_possible(&s));
        s.2 = (SYSTEM_ONLINE, SYSTEM_ONLINE, SYSTEM_ONLINE, SYSTEM_ONLINE);
        assert!(facility_status_is_evacuation_possible(&s));
        s.0.0 = 4;
        assert!(!facility_status_is_evacuation_possible(&s));
        s.0.0 = 5;
        s.0.1 = 9;
        assert!(!facility_status_is_evacuation_possible(&s));
        s.0.1 = 10;
        assert!(facility_status_is_evacuation_possible(&s));
        s.2.3 = SYSTEM_DAMAGED;
        assert!(!facility_status_is_evacuation_possible(&s));
    }

    #[test]
    fn elapsed_time_is_derived_from_remaining_time() {
        let mut s = state();
        s.0.2 = 18;
        assert_eq!(facility_status_get_time_elapsed(&s), 12);
    }

    #[test]
    fn resource_helpers_enforce_the_specified_limits() {
        let mut s = state();
        s.0.0 = 2;
        facility_status_decrease_fuel(&mut s, 5);
        assert_eq!(s.0.0, 0, "fuel must not become negative");

        s.0.1 = 95;
        facility_status_increase_power(&mut s, 10);
        assert_eq!(s.0.1, 100, "power must be capped at 100");

        facility_status_decrease_time(&mut s, 31);
        assert_eq!(s.0.2, 0, "remaining time must not become negative");
    }

    #[test]
    fn fault_condition_uses_sum_of_elapsed_time_and_fuel_before_modulo() {
        let mut s = state();
        s.0.0 = 89;
        s.0.2 = 0; // elapsed 30; 30 + 89 is divisible by 7
        assert!(mission_restore_system_should_trigger_fault(&s));
        s.0.0 = 90;
        assert!(!mission_restore_system_should_trigger_fault(&s));
    }

    #[test]
    fn failed_mission_boundaries_and_reason_follow_spec() {
        let mut s = state();
        assert!(!mission_is_mission_failed(&s));
        s.0.0 = 0;
        assert!(mission_is_mission_failed(&s));
        assert_eq!(mission_mission_failed_reason(&s), "You are out of fuel");

        s = state();
        s.0.1 = 0;
        assert!(mission_is_mission_failed(&s));
        assert_eq!(mission_mission_failed_reason(&s), "You are out of emergency power");

        s = state();
        s.0.2 = 0;
        assert!(mission_is_mission_failed(&s));
        assert_eq!(mission_mission_failed_reason(&s), "Time is up");

        s = state();
        s.2.0 = SYSTEM_DAMAGED;
        assert!(mission_is_mission_failed(&s));
        assert_eq!(mission_mission_failed_reason(&s), "Life support is broken");
    }

    #[test]
    fn mission_succeeds_only_after_all_twelve_are_evacuated() {
        let mut s = state();
        assert!(!mission_is_mission_successful(&s));
        facility_status_evacuate_occupants(&mut s, 4);
        assert_eq!(s.1, 8);
        assert!(!mission_is_mission_successful(&s));
        facility_status_evacuate_occupants(&mut s, 20);
        assert_eq!(s.1, 0);
        assert_eq!(facility_status_get_evacuated_occupants(&s), 12);
        assert!(mission_is_mission_successful(&s));
    }

    #[test]
    fn scoring_awards_only_criteria_met_and_uses_classification_thresholds() {
        let mut s = state();
        s.1 = 0;
        assert_eq!(mission_calculate_mission_score(&s), 100);
        assert_eq!(mission_get_mission_score_grade(100), "Exceptional");
        assert_eq!(mission_get_mission_score_grade(90), "Exceptional");
        assert_eq!(mission_get_mission_score_grade(89), "Successful");
        assert_eq!(mission_get_mission_score_grade(70), "Successful");
        assert_eq!(mission_get_mission_score_grade(69), "Marginal");
        assert_eq!(mission_get_mission_score_grade(50), "Marginal");
        assert_eq!(mission_get_mission_score_grade(49), "Poor");

        s.0.2 = 9; // elapsed 21: misses the speed award
        s.0.0 = 29;
        s.0.1 = 29;
        assert_eq!(mission_calculate_mission_score(&s), 50);
    }

    #[test]
    fn system_repair_transitions_damaged_to_offline_then_online() {
        let mut s = state();
        s.2.0 = SYSTEM_DAMAGED;
        facility_status_repair_life_support(&mut s);
        assert_eq!(s.2.0, SYSTEM_OFFLINE);
        facility_status_restore_life_support(&mut s);
        assert_eq!(s.2.0, SYSTEM_ONLINE);
    }

    #[test]
    fn evacuation_never_removes_more_than_the_remaining_occupants() {
        let mut s = state();
        s.1 = 2;
        facility_status_evacuate_occupants(&mut s, 4);
        assert_eq!(s.1, 0);
        assert_eq!(facility_status_get_evacuated_occupants(&s), 12);
    }
}
}
