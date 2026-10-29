mod app {
    include!("../src/main.rs");

    #[cfg(test)]
    mod spec_tests {
        use super::*;

        fn facility() -> Facility {
            INITIAL_FACILITY
        }

        fn set_all_systems_online(facility: &mut Facility) {
            facility.systems_state.life_support = SystemStatus::Online;
            facility.systems_state.communications = SystemStatus::Online;
            facility.systems_state.security = SystemStatus::Online;
            facility.systems_state.evacuation_lift = SystemStatus::Online;
        }

        fn set_all_systems_damaged(facility: &mut Facility) {
            facility.systems_state.life_support = SystemStatus::Damaged;
            facility.systems_state.communications = SystemStatus::Damaged;
            facility.systems_state.security = SystemStatus::Damaged;
            facility.systems_state.evacuation_lift = SystemStatus::Damaged;
        }

        #[test]
        fn new_mission_has_the_specified_initial_state() {
            let f = facility();

            assert_eq!(f.resources.fuel, 100);
            assert_eq!(f.resources.emergency_power, 100);
            assert_eq!(f.resources.time, 30);
            assert_eq!(f.get_time_elapsed(), 0);
            assert_eq!(f.n_remaining_occupants, 12);
            assert_eq!(f.get_evacuated_occupants(), 0);
            assert_eq!(f.systems_state.life_support.get_label(), "Offline");
            assert_eq!(f.systems_state.communications.get_label(), "Offline");
            assert_eq!(f.systems_state.security.get_label(), "Online");
            assert_eq!(f.systems_state.evacuation_lift.get_label(), "Offline");
        }

        #[test]
        fn elapsed_time_is_derived_from_remaining_time() {
            let mut f = facility();
            f.resources.time = 18;

            assert_eq!(f.get_time_elapsed(), 12);
        }

        #[test]
        fn fuel_cannot_become_negative() {
            let mut f = facility();
            f.resources.fuel = 2;
            f.decrease_fuel(5);

            assert_eq!(f.resources.fuel, 0);
        }

        #[test]
        fn emergency_power_cannot_exceed_one_hundred() {
            let mut f = facility();
            f.resources.emergency_power = 95;
            f.increase_power(10);

            assert_eq!(f.resources.emergency_power, 100);
        }

        #[test]
        fn remaining_time_cannot_become_negative() {
            let mut f = facility();
            f.resources.time = 1;
            f.decrease_time(5);

            assert_eq!(f.resources.time, 0);
        }

        #[test]
        fn evacuation_requires_every_system_online() {
            let mut f = facility();
            set_all_systems_online(&mut f);
            assert!(f.is_evacuation_possible());

            f.systems_state.life_support = SystemStatus::Offline;
            assert!(!f.is_evacuation_possible());
            f.systems_state.life_support = SystemStatus::Online;

            f.systems_state.communications = SystemStatus::Damaged;
            assert!(!f.is_evacuation_possible());
            f.systems_state.communications = SystemStatus::Online;

            f.systems_state.security = SystemStatus::Offline;
            assert!(!f.is_evacuation_possible());
            f.systems_state.security = SystemStatus::Online;

            f.systems_state.evacuation_lift = SystemStatus::Damaged;
            assert!(!f.is_evacuation_possible());
        }

        #[test]
        fn evacuation_requires_minimum_fuel_and_emergency_power() {
            let mut f = facility();
            set_all_systems_online(&mut f);

            f.resources.fuel = 4;
            assert!(!f.is_evacuation_possible());
            f.resources.fuel = 5;
            assert!(f.is_evacuation_possible());

            f.resources.emergency_power = 9;
            assert!(!f.is_evacuation_possible());
            f.resources.emergency_power = 10;
            assert!(f.is_evacuation_possible());
        }

        #[test]
        fn fault_condition_is_deterministic() {
            let mut f = facility();
            f.resources.time = 0;
            f.resources.fuel = 89;
            assert!(Mission::restore_system_should_trigger_fault(&f));

            f.resources.fuel = 90;
            assert!(!Mission::restore_system_should_trigger_fault(&f));

            f.resources.fuel = 96;
            assert!(Mission::restore_system_should_trigger_fault(&f));
        }

        #[test]
        fn damaged_system_repairs_to_offline() {
            let mut f = facility();

            f.systems_state.life_support = SystemStatus::Damaged;
            f.repair_life_support();
            assert_eq!(f.systems_state.life_support.get_label(), "Offline");

            f.systems_state.communications = SystemStatus::Damaged;
            f.repair_communication();
            assert_eq!(f.systems_state.communications.get_label(), "Offline");

            f.systems_state.evacuation_lift = SystemStatus::Damaged;
            f.repair_evacuation_lift();
            assert_eq!(f.systems_state.evacuation_lift.get_label(), "Offline");
        }

        #[test]
        fn damaged_security_repairs_to_offline() {
            let mut f = facility();
            f.systems_state.security = SystemStatus::Damaged;

            f.repair_security();

            assert_eq!(f.systems_state.security.get_label(), "Offline");
        }

        #[test]
        fn repair_does_not_bring_a_system_online() {
            let mut f = facility();
            f.systems_state.life_support = SystemStatus::Damaged;

            f.repair_life_support();

            assert_eq!(f.systems_state.life_support.get_label(), "Offline");
        }

        #[test]
        fn offline_systems_restore_online() {
            let mut f = facility();

            f.restore_life_support();
            assert_eq!(f.systems_state.life_support.get_label(), "Online");

            f.restore_communication();
            assert_eq!(f.systems_state.communications.get_label(), "Online");

            f.systems_state.security = SystemStatus::Offline;
            f.restore_security();
            assert_eq!(f.systems_state.security.get_label(), "Online");

            f.restore_evacuation_lift();
            assert_eq!(f.systems_state.evacuation_lift.get_label(), "Online");
        }

        #[test]
        fn emergency_shutdown_takes_security_offline() {
            let mut f = facility();
            assert_eq!(f.systems_state.security.get_label(), "Online");

            f.shutdown_security();

            assert_eq!(f.systems_state.security.get_label(), "Offline");
        }

        #[test]
        fn evacuation_never_removes_more_than_the_remaining_occupants() {
            let mut f = facility();
            f.n_remaining_occupants = 2;

            f.evacuate_occupants(4);

            assert_eq!(f.n_remaining_occupants, 0);
            assert_eq!(f.get_evacuated_occupants(), 12);
        }

        #[test]
        fn new_mission_is_not_failed() {
            assert!(!Mission::is_mission_failed(&facility()));
        }

        #[test]
        fn mission_fails_when_fuel_is_exhausted() {
            let mut f = facility();
            f.resources.fuel = 0;

            assert!(Mission::is_mission_failed(&f));
            assert_eq!(Mission::mission_failed_reason(&f), "You are out of fuel");
        }

        #[test]
        fn mission_fails_when_emergency_power_is_exhausted() {
            let mut f = facility();
            f.resources.emergency_power = 0;

            assert!(Mission::is_mission_failed(&f));
            assert_eq!(
                Mission::mission_failed_reason(&f),
                "You are out of emergency power"
            );
        }

        #[test]
        fn mission_fails_when_time_limit_is_reached() {
            let mut f = facility();
            f.resources.time = 0;

            assert!(Mission::is_mission_failed(&f));
            assert_eq!(Mission::mission_failed_reason(&f), "Time is up");
        }

        #[test]
        fn mission_fails_when_life_support_is_damaged() {
            let mut f = facility();
            f.systems_state.life_support = SystemStatus::Damaged;

            assert!(Mission::is_mission_failed(&f));
            assert_eq!(
                Mission::mission_failed_reason(&f),
                "Life support is broken"
            );
        }

        #[test]
        fn mission_failure_reason_is_empty_when_no_failure_condition_holds() {
            assert_eq!(Mission::mission_failed_reason(&facility()), "");
        }

        #[test]
        fn mission_failure_reason_checks_fuel_first() {
            let mut f = facility();
            f.resources.fuel = 0;
            f.resources.emergency_power = 0;
            f.resources.time = 0;
            f.systems_state.life_support = SystemStatus::Damaged;

            assert_eq!(Mission::mission_failed_reason(&f), "You are out of fuel");
        }

        #[test]
        fn mission_succeeds_only_after_all_twelve_are_evacuated() {
            let mut f = facility();
            assert!(!Mission::is_mission_successful(&f));

            f.evacuate_occupants(4);
            assert_eq!(f.n_remaining_occupants, 8);
            assert!(!Mission::is_mission_successful(&f));

            f.evacuate_occupants(4);
            assert_eq!(f.n_remaining_occupants, 4);
            assert!(!Mission::is_mission_successful(&f));

            f.evacuate_occupants(4);
            assert_eq!(f.n_remaining_occupants, 0);
            assert!(Mission::is_mission_successful(&f));
        }

        #[test]
        fn scoring_awards_every_met_criterion() {
            let mut f = facility();
            f.n_remaining_occupants = 0;
            f.resources.time = 10;
            f.resources.fuel = 30;
            f.resources.emergency_power = 30;

            assert_eq!(Mission::calculate_mission_score(&f), 100);
        }

        #[test]
        fn scoring_awards_only_the_criteria_that_are_met() {
            let mut f = facility();
            f.n_remaining_occupants = 0;
            f.resources.time = 9;
            f.resources.fuel = 29;
            f.resources.emergency_power = 29;

            assert_eq!(Mission::calculate_mission_score(&f), 50);
        }

        #[test]
        fn scoring_uses_the_classification_thresholds() {
            assert_eq!(Mission::get_mission_score_grade(100), "Exceptional");
            assert_eq!(Mission::get_mission_score_grade(90), "Exceptional");
            assert_eq!(Mission::get_mission_score_grade(89), "Successful");
            assert_eq!(Mission::get_mission_score_grade(70), "Successful");
            assert_eq!(Mission::get_mission_score_grade(69), "Marginal");
            assert_eq!(Mission::get_mission_score_grade(50), "Marginal");
            assert_eq!(Mission::get_mission_score_grade(49), "Poor");
            assert_eq!(Mission::get_mission_score_grade(0), "Poor");
        }

        #[test]
        fn reset_returns_the_facility_to_its_initial_state() {
            let mut f = facility();
            f.resources.fuel = 1;
            f.resources.emergency_power = 2;
            f.resources.time = 3;
            f.n_remaining_occupants = 4;
            set_all_systems_damaged(&mut f);

            f.reset();

            assert_eq!(f.resources.fuel, 100);
            assert_eq!(f.resources.emergency_power, 100);
            assert_eq!(f.resources.time, 30);
            assert_eq!(f.n_remaining_occupants, 12);
            assert_eq!(f.systems_state.life_support.get_label(), "Offline");
            assert_eq!(f.systems_state.communications.get_label(), "Offline");
            assert_eq!(f.systems_state.security.get_label(), "Online");
            assert_eq!(f.systems_state.evacuation_lift.get_label(), "Offline");
        }
    }
}
