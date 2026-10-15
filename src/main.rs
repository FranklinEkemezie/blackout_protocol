use std::{cmp::{max, min}, process::exit};


type SystemState = u8;

const SYSTEM_ONLINE:    SystemState = 1;
const SYSTEM_OFFLINE:   SystemState = 2;
const SYSTEM_DAMAGED:   SystemState = 3;

type Option = u8;

// Main Menu

const MAIN_MENU: &str = "
1. Start New Mission
2. View Mission Briefing
3. View Instructions
4. Exit
";
const MAX_MAIN_MENU_OPTION: Option = 4;

const MISSION_MENU: &str = "
1. Inspect Facility
2. Restore a System
3. Repair a System
4. Manage Emergency Power
5. Evacuate Occupants
6. End Mission
";
const MAX_MISSION_MENU_OPTION: Option = 6;

const MISSION_BRIEFING: &str = "

OPERATION: BLACKOUT PROTOCOL
LOCATION: Remote Research Facility
THREAT LEVEL: CRITICAL

SITUATION
------------------------------------
A catastrophic power failure has left
the facility in a critical condition.

Several essential systems are offline,
and the emergency generator has limited
fuel and power reserves.

Twelve occupants remain inside the
facility. You are the emergency response
operator responsible for restoring
essential systems and coordinating
their evacuation.

MISSION OBJECTIVES
------------------------------------
1. Restore Life Support.
2. Restore Communications.
3. Ensure Security is operational.
4. Restore the Evacuation Lift.
5. Evacuate all 12 occupants.

INITIAL RESOURCES
------------------------------------
Fuel:              100 units
Emergency Power:   100 units
Time Limit:        30 minutes
Occupants:         12 people

MISSION CONDITIONS
------------------------------------
Every action consumes time or resources,
and some actions may affect other
systems.

You must carefully manage the facility's
limited resources while preparing for
evacuation.

MISSION SUCCESS
------------------------------------
Evacuate all 12 occupants before any
mission failure condition is triggered.

Your final performance will be evaluated
based on evacuation time, remaining
resources, and system faults.

";

const INSTRUCTIONS: &str = "

1. NAVIGATING THE MENUS
------------------------------------
Enter the number corresponding to the
action you want to perform.

During a mission, the status report
shows your current resources, elapsed
time, system conditions, and evacuation
progress.

Invalid selections will be rejected.

2. INSPECTING THE FACILITY
------------------------------------
Inspect the facility to review system
conditions, available resources, and
evacuation readiness.

Inspection consumes 2 minutes.

3. RESTORING SYSTEMS
------------------------------------
Select an offline system to restore it.

Each system requires specific amounts
of fuel, emergency power, and time.

Only offline systems can be restored.
Damaged systems must be repaired first.

A restoration attempt may trigger a
system fault. If this happens, the
selected system becomes damaged.

4. REPAIRING SYSTEMS
------------------------------------
Repairing a damaged system costs:
- 4 fuel units
- 5 power units
- 3 minutes

Repair returns the system to its
offline state. You must restore it
separately afterward.

5. MANAGING EMERGENCY POWER
------------------------------------
CONSERVE POWER:
- Gain 10 power units.
- Consume 5 fuel units.
- Advance time by 2 minutes.

EMERGENCY SHUTDOWN:
- Gain 20 power units.
- Consume 10 fuel units.
- Advance time by 4 minutes.
- Security goes offline if it was online.

Emergency power cannot exceed 100.

6. EVACUATING OCCUPANTS
------------------------------------
Evacuation requires all four systems
to be online.

You must also have at least:
- 5 fuel units.
- 10 power units.

Each evacuation operation:
- Evacuates up to 4 occupants.
- Consumes 5 fuel units.
- Consumes 10 power units.
- Advances time by 3 minutes.

7. MISSION FAILURE
------------------------------------
The mission fails if:
- Fuel reaches zero before evacuation
  is complete.
- Emergency power reaches zero before
  evacuation is complete.
- Time reaches 30 minutes before
  evacuation is complete.
- Life Support becomes damaged at the
  end of an action.
- You choose to end the mission.

8. MISSION SCORING
------------------------------------
Your successful mission is scored out
of 100 points.

Points are awarded for:
- Evacuating all 12 occupants.
- Completing evacuation within 20 min.
- Finishing with at least 30 fuel units.
- Finishing with at least 30 power units.
- Completing the mission without faults.

---------------------------------------
Plan carefully. Every action counts!
---------------------------------------
";

type Resources = (
    u8, // fuel units
    u8, // emergency power units
    u8, // time limit (in minutes)
);

type SystemConfiguration = (
    SystemState,    // life support
    SystemState,    // communications
    SystemState,    // security
    SystemState     // evacuation lift
);

type FacilityStatus = (
    Resources,          // resources - fuel, power and time
    u8,                 // remaining occupants
    SystemConfiguration // all system statement
);

const INITIAL_RESOURCES: Resources = (100, 100, 30);
const INITIAL_OCCUPANTS: u8 = 12;
const INITIAL_SYSTEMS: SystemConfiguration = (
    SYSTEM_OFFLINE,
    SYSTEM_OFFLINE,
    SYSTEM_ONLINE,
    SYSTEM_OFFLINE,
);

const INITIAL_FACILITY_STATUS: FacilityStatus = (
    INITIAL_RESOURCES, INITIAL_OCCUPANTS, INITIAL_SYSTEMS
);

fn main() {


    let mut facility_status = INITIAL_FACILITY_STATUS;

    loop {
        handle_main_menu_option(get_main_menu_option(), &mut facility_status);
    }

}

fn print_heading(heading: &str) {

    println!("");
    println!("{:=^40}", "");
    println!("{:^40}", String::from(heading).to_uppercase());
    println!("{:=^40}", "");
}

fn print_subheading(heading: &str) {
    println!("");
    println!("{:-^40}", "");
    println!("{heading:^40}");
    println!("{:-^40}", "");
}

fn prompt_user(label: &str) -> String {
    loop {

        println!("{label}: ");

        let mut buf = String::from("");

        let result = std::io::stdin().read_line(&mut buf);
        if ! result.is_ok() {
            print_error_msg("An error occurred. Please try again!");
            continue;
        }

        return String::from(buf.trim());
    }
}

fn prompt_user_option(label: &str, max_option: Option) -> Option {

    loop {

        let input = prompt_user(label);

        let result = input.parse::<Option>();
        if ! result.is_ok() {
            print_error_msg("Invalid input. Please try again");
            continue;
        }

        let option: Option = result.expect("Invalid input");
        if option > max_option {
            print_error_msg(&std::format!(
                "Invalid option {option}. Option must be between 1 and {max_option}"));
            continue;
        }

        return option;
    }
}

fn prompt_user_char(label: &str) -> char {

    loop {
        let input_char = prompt_user(label);

        if input_char.len() != 1 {
            print_error_msg("Invalid input: Please enter a character to continue");
            continue;
        }

        let input_char = input_char.clone().chars().next();

        return input_char.expect("Invalid input");
    }
}

fn print_info_msg(msg: &str) {
    println!("\n[INFO] {msg}\n");
}

fn print_error_msg(msg: &str) {
    println!("\n[ERROR] {msg}\n");
}

fn get_main_menu_option() -> Option {

    print_heading("Blackout Protocol - Main Menu");
    println!("{MAIN_MENU}");

    prompt_user_option("Select an option", MAX_MAIN_MENU_OPTION)
}

fn handle_main_menu_option(option: Option, facility_status: &mut FacilityStatus) {

    match option {
        1 => main_menu_start_new_mission(facility_status),
        2 => main_menu_view_mission_briefing(),
        3 => main_menu_view_instructions(),
        4 => main_menu_exit_application(),
        _ => print_error_msg(&std::format!("Unknown option: {option}")),
    }
}

fn main_menu_exit_application() {
    print_info_msg("Exiting application. Bye...");
    exit(0);
}

fn main_menu_view_mission_briefing() {
    print_heading("Mission Briefing");
    println!("{MISSION_BRIEFING}");
}

fn main_menu_view_instructions() {
    print_heading("How to Play");
    println!("{INSTRUCTIONS}");
}

fn system_state_get_label(system_state: SystemState) -> String {
    match system_state {
        SYSTEM_ONLINE   => String::from("Online"),
        SYSTEM_OFFLINE  => String::from("Offline"),
        SYSTEM_DAMAGED  => String::from("Damaged"),
        _               => String::from("Unknown")
    }
}

fn facility_status_get_evacuated_occupants(facility_status: &FacilityStatus) -> u8 {
    INITIAL_OCCUPANTS - facility_status.1
}

fn facility_status_print(facility_status: &FacilityStatus) {

    let (resources, remaining_occupants, system_config) = facility_status;

    println!(
        "
Elapsed Time:       {} minutes
Fuel:               {} units
Emergency Power:    {} units

Life Support:       {}
Communications:     {}
Security:           {}
Evacuation Lift:    {}

Occupants Remaining: {}
Occupants Evacuated: {}
        ",
        facility_status_get_time_elapsed(facility_status), 
        resources.1, 
        resources.0,

        system_state_get_label(system_config.0),
        system_state_get_label(system_config.1),
        system_state_get_label(system_config.2),
        system_state_get_label(system_config.3),

        remaining_occupants, facility_status_get_evacuated_occupants(facility_status)
    );
}

fn main_menu_start_new_mission(facility_status: &mut FacilityStatus) {

    print_heading("Mission Initial Status");
    facility_status_print(&INITIAL_FACILITY_STATUS);

    reset_facility_status(facility_status);

    loop {
        let option = get_mission_option(facility_status.0.2);

        handle_mission_option(option, facility_status);

        if option == 6 { break; } // End mission

        if mission_is_mission_failed(facility_status) {
            let reason = mission_mission_failed_reason(facility_status);
            mission_print_mission_failed_debrief_report(&reason, facility_status);        
            return;
        }

        if mission_is_mission_successful(facility_status) {
            print_info_msg("Mission Successful!");

            let mission_score = mission_calculate_mission_score(facility_status);
            let mission_score_grade = mission_get_mission_score_grade(mission_score);

            print_heading(&std::format!(
                "You scored {mission_score}%. {mission_score_grade}"));
            return;
        }

    }

}

fn get_mission_option(remaining_time_mins: u8) -> Option {

    print_heading("Blackout Protocol - Mission On");
    println!("{MISSION_MENU}");

    print_info_msg(&std::format!("{remaining_time_mins} minutes remaining"));

    prompt_user_option("Choose an action", MAX_MISSION_MENU_OPTION)
}

fn handle_mission_option(option: Option, facility_status: &mut FacilityStatus) {

    match option {
        1 => mission_inspect_facility(facility_status),
        2 => mission_restore_system(facility_status),
        3 => mission_repair_system(facility_status),
        4 => mission_manage_emergency_power(facility_status),
        5 => mission_evacuate_occupants(facility_status),
        6 => mission_end_mission(facility_status),
        _ => print_error_msg(&std::format!("Unknown option: {option}")),
    }
}

fn reset_facility_status(facility_status: &mut FacilityStatus) {
    *facility_status = INITIAL_FACILITY_STATUS;
}

fn confirm_action(info: &str) -> bool {

    loop {

        let input = prompt_user(&std::format!(
            "{info}. Do you wish to contine? [Y/N]"));
        
        if input == "y" || input == "yes" { return true; }
        else if input == "n" || input == "no" { return false; }

        print_error_msg("Invalid input. Please try again!");
    }
}

// Facility status management functions

fn facility_status_decrease_fuel(facility_status: &mut FacilityStatus, fuel_units: u8) {
    facility_status.0.0 = max(facility_status.0.0.saturating_sub(fuel_units), 0);
}

fn facility_status_decrease_power(facility_status: &mut FacilityStatus, power_units: u8) {
    facility_status.0.1 = max(facility_status.0.1.saturating_sub(power_units), 0);
}

fn facility_status_increase_power(facility_status: &mut FacilityStatus, power_units: u8) {
    facility_status.0.1 = min(facility_status.0.1 + power_units, INITIAL_FACILITY_STATUS.0.0);
}

fn facility_status_decrease_time(facility_status: &mut FacilityStatus, time_units: u8) {
    facility_status.0.2 = max(facility_status.0.2.saturating_sub(time_units), 0);
}

// Mission functions

fn mission_inspect_facility(facility_status: &mut FacilityStatus) {

    const TIME_COST: u8 = 2;

    let confirm = confirm_action(&std::format!("This action cost {TIME_COST} minutes"));

    if confirm {
        facility_status_decrease_time(facility_status, TIME_COST);

        facility_status_print_inspection_report(facility_status);
    }
}

fn mission_restore_system_table_heading() -> String {
    let row = std::format!("|{:^16}|{:^24}|{:^16}|{:^16}|{:^16}|", 
        "Option", "System", "Fuel Units", "Power Units", "Time");
    let bottom_border = String::from("_").repeat(row.len());
    let top_border = String::from("—").repeat(row.len());

    std::format!("{top_border}\n{row}\n{bottom_border}\n")
}

fn mission_restore_system_table_row(option_char: char, system: &str, fuel_units: u8, 
    power_units: u8, time_mins: u8
) -> String {
    let row = std::format!("|{:^16}|{:^24}|{:^16}|{:^16}|{:^16}|", 
        std::format!("[{option_char}]"), system, fuel_units, power_units, time_mins);
    let bottom_border = String::from("_").repeat(row.len());
    
    std::format!("{row}\n{bottom_border}\n")
}

fn mission_restore_system_should_trigger_fault(facility_status: &FacilityStatus) -> bool {
    ((facility_status_get_time_elapsed(facility_status) + facility_status.0.0) % 7) == 0
}

fn mission_restore_system(facility_status: &mut FacilityStatus) {

    let restore_system_menu_title = mission_restore_system_table_heading();

    let life_support_option  = mission_restore_system_table_row('L', "Life Support", 8, 12, 5);
    let communications_option = mission_restore_system_table_row('C', "Communications", 5, 8, 3);
    let security_option = mission_restore_system_table_row('S', "Security", 6, 10, 4);
    let evacuation_lift_option = mission_restore_system_table_row('E', "Evacuation Lift", 10, 15, 6);

    println!("{restore_system_menu_title}");

    if facility_status_is_life_support_offline(facility_status) {
        println!("{life_support_option}");
    }

    if facility_status_is_communication_offline(facility_status) {
        println!("{communications_option}");
    }

    if facility_status_is_security_offline(facility_status) {
        println!("{security_option}");
    }

    if facility_status_is_evacuation_lift_offline(facility_status) {
        println!("{evacuation_lift_option}");
    }

    print_info_msg("Enter 'X' to cancel\n");

    loop {
        let option_char = prompt_user_char("Select system");

        if option_char == 'X' {
            print_info_msg("Operation Cancelled");
            return;
        }
        
        if mission_restore_system_should_trigger_fault(facility_status) {
            match option_char {
                'L' => {
                    facility_status_shutdown_life_support(facility_status);
                    print_error_msg("The life support system damaged during restoration");
                },
                'C' => {
                    facility_status_shutdown_communication(facility_status);
                    print_error_msg("The communication system damaged during restoration");
                },
                'S' => {
                    facility_status_shutdown_security(facility_status);
                    print_error_msg("The security system damaged during restoration");
                },
                'E' => {
                    facility_status_shutdown_evacuation_lift(facility_status);
                    print_error_msg("The evacuation system damaged during restoration");
                },
                _   => {
                    print_error_msg(&std::format!(
                        "Invalid input [{option_char}]. Please try again!"));
                    continue;
                }
            }

            return;
        }

        match option_char {
            'L' => {
                facility_status_decrease_fuel(facility_status, 8);
                facility_status_decrease_power(facility_status, 12);
                facility_status_decrease_time(facility_status, 3);

                facility_status_restore_life_support(facility_status);

                print_info_msg("The life support system is now restored and fully online!");
            },
            'C' => {
                facility_status_decrease_fuel(facility_status, 5);
                facility_status_decrease_power(facility_status, 8);
                facility_status_decrease_time(facility_status, 3);

                facility_status_restore_communication(facility_status);

                print_info_msg("The communication system is now restored and fully online!");
            },
            'S' => {
                facility_status_decrease_fuel(facility_status, 6);
                facility_status_decrease_power(facility_status, 10);
                facility_status_decrease_time(facility_status, 4);

                facility_status_restore_security(facility_status);

                print_info_msg("The security system is now restored and fully online!");
            },
            'E' => {
                facility_status_decrease_fuel(facility_status, 10);
                facility_status_decrease_power(facility_status, 15);
                facility_status_decrease_time(facility_status, 6);

                facility_status_restore_evacuation_lift(facility_status);

                print_info_msg("The evacuation lift system is now restored and fully online!");
            },
            _   => {
                print_error_msg(&std::format!(
                    "Invalid input [{option_char}]. Please try again!"));
                continue;
            }
        }

        return;
    }

    
}

fn mission_repair_system(facility_status: &mut FacilityStatus) {

    let mut repair_system_options = String::from("");

    print_subheading("Damaged Systems");

    if facility_status_is_life_support_damaged(facility_status) {
        println!("[L] Life Support");
        repair_system_options += "L";
    }

    if facility_status_is_communication_damaged(facility_status) {
        println!("[C] Communications");
        repair_system_options += "C";
    }

    if facility_status_is_security_damaged(facility_status) {
        println!("[S] Security");
        repair_system_options += "S";
    }

    if facility_status_is_evacuation_lift_damaged(facility_status) {
        println!("[E] Evacuation Lift");
        repair_system_options += "E";
    }

    if repair_system_options.is_empty() {
        print_info_msg("No damaged system found");
        return;
    }

    print_info_msg("Enter 'X' to Cancel");

    loop {

        let option_char = prompt_user_char("Select system");

        if option_char == 'X' {
            print_info_msg("Operation Cancelled");
            return;
        }

        if ! repair_system_options.contains(option_char) {
            print_error_msg("Invalid option. Please try again!");
            continue;
        }

        let confirm = confirm_action("This will cost 4 fuel units, 5 emergency power units, and 3 minutes");
        if ! confirm {
            print_info_msg("Operation Cancelled");
            return;
        }

        facility_status_decrease_fuel(facility_status, 4);
        facility_status_decrease_power(facility_status, 5);
        facility_status_decrease_time(facility_status, 3);

        match option_char {
            'L' => facility_status_repair_life_support(facility_status),
            'C' => facility_status_repair_communication(facility_status),
            'S' => facility_status_repair_security(facility_status),
            'E' => facility_status_repair_evacuation_lift(facility_status),
            _   => {
                print_info_msg(&std::format!(
                    "Invalid input: {option_char}. Please try again!"));
                continue;
            }
        }

        print_info_msg("System repaired. You may now restore it.");
        return;
    }

    

}

fn mission_manage_emergency_power(facility_status: &mut FacilityStatus) {

    const MANAGE_EMERGENCY_POWER_MENU: &str = "
1. Conserve Power - Reduce the facility's energy consumption.
    Increases emergency power by 10 units for 5 fuel units. 
    Takes 2 minutes.
2. Emergency Shutdown - Shut down nonessential systems to reduce power consumption.
    Increases emergency power by 20 units for 10 fuel units.
    Takes 4 minutes. Security system goes offline.
    ";

    println!("{MANAGE_EMERGENCY_POWER_MENU}");

    let option = prompt_user_option("Choose an option", 2);

    match option {
        1   => {
            // Conserve power

            let confirm = confirm_action("This takes 5 fuel units and 2 minutes");
            if ! confirm {
                print_info_msg("Operation Cancelled!");
                return;
            }

            facility_status_increase_power(facility_status, 10);
            facility_status_decrease_fuel(facility_status, 5);
            facility_status_decrease_time(facility_status, 2);
        },
        2   => {

            // Shutdown nonessential systems

            let confirm = confirm_action("This takes 10 fuel units and 2 minutes, and shuts down the security system");
            if ! confirm {
                print_info_msg("Operation Cancelled!");
                return;
            }

            facility_status_increase_power(facility_status, 20);
            facility_status_decrease_fuel(facility_status, 10);
            facility_status_decrease_time(facility_status, 4);
            facility_status_shutdown_security(facility_status);

        },
        _   => print_error_msg(&std::format!("Unknown option: {option}. Please try again!"))
    }

}

fn mission_evacuate_occupants(facility_status: &mut FacilityStatus) {

    if ! facility_status_is_communication_online(facility_status) {
        print_error_msg("Failed to evacuate: Communications is not online");
        return;
    }

    if ! facility_status_is_evacuation_lift_online(facility_status) {
        print_error_msg("Failed to evacuate: Evacuation lift is not online");
        return;
    }

    if ! facility_status_is_life_support_online(facility_status) {
        print_error_msg("Failed to evacuate: Life support is not online");
        return;
    }

    if ! facility_status_is_security_online(facility_status) {
        print_error_msg("Failed to evacuate: Security is not online");
        return;
    }

    if ! (facility_status.0.0 >= 5) {
        print_error_msg("Failed to evacuate: Insufficient fuel");
        return;
    }

    if ! (facility_status.0.1 >= 10) {
        print_error_msg("Failed to evacuate: Insufficient emergency power");
        return;
    }

    facility_status_decrease_fuel(facility_status, 5);
    facility_status_decrease_power(facility_status, 10);
    facility_status_decrease_time(facility_status, 3);

    const MAX_OCCUPANTS_PER_EVACUATION: u8 = 4;
    
    facility_status_evacuate_occupants(facility_status, MAX_OCCUPANTS_PER_EVACUATION);

    print_info_msg(&std::format!(
        "Evacuated {MAX_OCCUPANTS_PER_EVACUATION} persons successfully! Remaining {} occupants",
            facility_status.1)
        );


}

fn mission_end_mission(facility_status: &FacilityStatus) {
    mission_print_mission_failed_debrief_report("You ended the mission", facility_status);
}

fn mission_print_mission_failed_debrief_report(
    reason: &str, facility_status: &FacilityStatus
) {
    print_heading(reason);
    facility_status_print(facility_status);
}

fn mission_is_mission_successful(facility_status: &FacilityStatus) -> bool {
    facility_status_get_evacuated_occupants(facility_status) == INITIAL_OCCUPANTS
}

fn mission_is_mission_failed(facility_status: &FacilityStatus) -> bool {

    let (resources, _, system_config) = facility_status;

    if  resources.0 == 0 ||     // fuel reaches zero
        resources.1 == 0 ||     // emergency power reaches zero
        resources.2 == 0 ||     // time remaining reaches zero
        system_config.0  == SYSTEM_DAMAGED  // life support is damaged
        { return true; }

    // the player still has enough resources, mission is still on, 
    // so it is not failed

    return false;
}

fn mission_mission_failed_reason(facility_status: &FacilityStatus) -> String {
    
    if facility_status.0.0 == 0 {
        return String::from("You are out of fuel");
    }

    if facility_status.0.1 == 0 {
        return String::from("You are out of emergency power");
    }

    if facility_status.0.2 == 0 {
        return String::from("Time is up");
    }

    if facility_status.2.0 == SYSTEM_DAMAGED {
        return String::from("Life support is broken");
    }

    return String::from("");
}

fn mission_calculate_mission_score(facility_status: &FacilityStatus) -> u8 {

    const EVACUATE_ALL_OCCUPANTS_SCORE:                 u8 = 40;
    const COMPLETE_IN_20_MINS_OR_LESS_SCORE:            u8 = 20;
    const FINISH_WITH_AT_LEAST_30_FUEL_UNITS_SCORE:     u8 = 15;
    const FINISH_WITH_AT_LEAST_30_POWER_UNITS_SCORE:    u8 = 15;
    const COMPLETE_MISSION_WITHOUT_SYSTEM_FAULT:        u8 = 10;

    let mut mission_score = 0;

    if facility_status_get_evacuated_occupants(facility_status) == INITIAL_OCCUPANTS {
        mission_score += EVACUATE_ALL_OCCUPANTS_SCORE;
    }

    if facility_status_get_time_elapsed(facility_status) <= 20 {
        mission_score += COMPLETE_IN_20_MINS_OR_LESS_SCORE;
    }

    if facility_status.0.0 >= 30 {
        mission_score += FINISH_WITH_AT_LEAST_30_FUEL_UNITS_SCORE;
    }

    if facility_status.0.1 >= 30 {
        mission_score += FINISH_WITH_AT_LEAST_30_POWER_UNITS_SCORE;
    }

    // TODO: Track system fault during game play!

    mission_score += COMPLETE_MISSION_WITHOUT_SYSTEM_FAULT;

    return mission_score;
}

fn mission_get_mission_score_grade(mission_score: u8) -> String {
    if mission_score >= 90 { return String::from("Exceptional") }
    if mission_score >= 70 { return String::from("Successful"); }
    if mission_score >= 50 { return String::from("Marginal") }

    String::from("Poor")
}

fn facility_status_print_inspection_report(facility_status: &FacilityStatus) {

    let (resources, remaining_occupants, system_config) = facility_status;

    print_heading("Facility Inspection Report");

    // System State

    print_subheading("System State");
    println!(
        "
Life Support:       {}
Communications:     {}
Security:           {}
Evacuation Lift:    {}
    ",
    system_state_get_label(system_config.0),
    system_state_get_label(system_config.1),
    system_state_get_label(system_config.2),
    system_state_get_label(system_config.3),
    );

    // Resources

    print_subheading("Resources");
    println!(
        "
Time:               {} minutes
Emergency Power:    {} units
Fuel:               {} units
        ",
        resources.2, resources.1, resources.0,
    );

    // Mission Progress

    print_subheading("Mission Progress");
    println!(
        "
Occupants Remaining: {}
Occupants Evacuated: {}
Elapsed Time:        {} minutes
        ",
        remaining_occupants, 
        facility_status_get_evacuated_occupants(facility_status), 
        facility_status_get_time_elapsed(facility_status)
    );

    print_subheading("Summary");
    println!(
        "
Evacuation Possible:    {}
Communication Restored: {}
Remaining Time:         {} minutes
        ",
        facility_status_is_evacuation_possible(facility_status),
        facility_status_is_communication_online(facility_status),
        facility_status.0.2, 
    );

}

fn facility_status_is_life_support_online(facility_status: &FacilityStatus) -> bool {
    facility_status.2.0 == SYSTEM_ONLINE
}

fn facility_status_is_life_support_offline(facility_status: &FacilityStatus) -> bool {
    facility_status.2.0 == SYSTEM_OFFLINE
}

fn facility_status_is_life_support_damaged(facility_status: &FacilityStatus) -> bool {
    facility_status.2.0 == SYSTEM_DAMAGED
}

fn facility_status_is_communication_online(facility_status: &FacilityStatus) -> bool {
    facility_status.2.1 == SYSTEM_ONLINE
}

fn facility_status_is_communication_offline(facility_status: &FacilityStatus) -> bool {
    facility_status.2.1 == SYSTEM_OFFLINE
}

fn facility_status_is_communication_damaged(facility_status: &FacilityStatus) -> bool {
    facility_status.2.1 == SYSTEM_DAMAGED
}

fn facility_status_is_security_online(facility_status: &FacilityStatus) -> bool {
    facility_status.2.2 == SYSTEM_ONLINE
}

fn facility_status_is_security_offline(facility_status: &FacilityStatus) -> bool {
    facility_status.2.2 == SYSTEM_OFFLINE
}

fn facility_status_is_security_damaged(facility_status: &FacilityStatus) -> bool {
    facility_status.2.2 == SYSTEM_DAMAGED
}

fn facility_status_is_evacuation_lift_online(facility_status: &FacilityStatus) -> bool {
    facility_status.2.3 == SYSTEM_ONLINE
}

fn facility_status_is_evacuation_lift_offline(facility_status: &FacilityStatus) -> bool {
    facility_status.2.3 == SYSTEM_OFFLINE
}

fn facility_status_is_evacuation_lift_damaged(facility_status: &FacilityStatus) -> bool {
    facility_status.2.3 == SYSTEM_DAMAGED
}

fn facility_status_get_time_elapsed(facility_status: &FacilityStatus) -> u8 {
    INITIAL_FACILITY_STATUS.0.2 - facility_status.0.2
}

fn facility_status_is_evacuation_possible(facility_status: &FacilityStatus) -> bool {
    facility_status_is_communication_online(facility_status) &&
    facility_status_is_evacuation_lift_online(facility_status) &&
    facility_status_is_life_support_online(facility_status) &&
    facility_status_is_security_online(facility_status) &&
    facility_status.0.0 >= 5 &&
    facility_status.0.1 >= 10
}

fn facility_status_restore_life_support(facility_status: &mut FacilityStatus) {
    if facility_status.2.0 != SYSTEM_OFFLINE {
        panic!("The system must be offlined to be restored");
    }

    facility_status.2.0 = SYSTEM_ONLINE;
}

fn facility_status_repair_life_support(facility_status: &mut FacilityStatus) {
    if facility_status.2.0 != SYSTEM_DAMAGED {
        panic!("The system must be damaged to be repaired");
    }

    facility_status.2.0 = SYSTEM_OFFLINE;
}

fn facility_status_shutdown_life_support(facility_status: &mut FacilityStatus) {
    facility_status.2.0 = SYSTEM_DAMAGED;
}

fn facility_status_restore_communication(facility_status: &mut FacilityStatus) {
    if facility_status.2.1 != SYSTEM_OFFLINE {
        panic!("The system must be offlined to be restored");
    }

    facility_status.2.1 = SYSTEM_ONLINE;
}

fn facility_status_repair_communication(facility_status: &mut FacilityStatus) {
    if facility_status.2.1 != SYSTEM_DAMAGED {
        panic!("The system must be damaged to be repaired");
    }

    facility_status.2.1 = SYSTEM_OFFLINE;
}

fn facility_status_shutdown_communication(facility_status: &mut FacilityStatus) {
    facility_status.2.1 = SYSTEM_DAMAGED;
}

fn facility_status_restore_security(facility_status: &mut FacilityStatus) {
    if facility_status.2.2 != SYSTEM_OFFLINE {
        panic!("The system must be offlined to be restored");
    }

    facility_status.2.2 = SYSTEM_ONLINE;
}

fn facility_status_repair_security(facility_status: &mut FacilityStatus) {
    if facility_status.2.3 != SYSTEM_DAMAGED {
        panic!("The system must be damaged to be repaired");
    }

    facility_status.2.3 = SYSTEM_OFFLINE;
}

fn facility_status_shutdown_security(facility_status: &mut FacilityStatus) {
    facility_status.2.2 = SYSTEM_DAMAGED;
}

fn facility_status_restore_evacuation_lift(facility_status: &mut FacilityStatus) {
    if facility_status.2.3 != SYSTEM_OFFLINE {
        panic!("The system must be offlined to be restored");
    }

    facility_status.2.3 = SYSTEM_ONLINE;
}

fn facility_status_repair_evacuation_lift(facility_status: &mut FacilityStatus) {
    if facility_status.2.3 != SYSTEM_DAMAGED {
        panic!("The system must be damaged to be repaired");
    }

    facility_status.2.3 = SYSTEM_OFFLINE;
}

fn facility_status_shutdown_evacuation_lift(facility_status: &mut FacilityStatus) {
    facility_status.2.3 = SYSTEM_DAMAGED;
}

fn facility_status_evacuate_occupants(facility_status: &mut FacilityStatus, occupants: u8) {
    facility_status.1 = max(facility_status.1.saturating_sub(occupants), 0);
}