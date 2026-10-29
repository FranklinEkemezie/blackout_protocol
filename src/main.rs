use std::{cmp::{max, min}, process::exit};


// Main Menu

const MAIN_MENU: &str = "
1. Start New Mission
2. View Mission Briefing
3. View Instructions
4. Exit
";
const MAX_MAIN_MENU_OPTION: MenuOption = 4;

const MISSION_MENU: &str = "
1. Inspect Facility
2. Restore a System
3. Repair a System
4. Manage Emergency Power
5. Evacuate Occupants
6. End Mission
";
const MAX_MISSION_MENU_OPTION: MenuOption = 6;

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

enum SystemStatus {
    Online,
    Offline,
    Damaged
}

type MenuOption = u8;

struct Resources {
    fuel: u32,
    emergency_power: u32,
    time: u32
}

struct SystemState {
    life_support: SystemStatus,
    communications: SystemStatus,
    security: SystemStatus,
    evacuation_lift: SystemStatus
}

struct Facility {
    resources: Resources,
    n_remaining_occupants: u8,
    systems_state: SystemState
}

const INITIAL_RESOURCES: Resources = Resources {
    fuel: 100, 
    emergency_power: 100, 
    time: 30
};

const INITIAL_N_REMAINING_OCCUPANTS: u8 = 12;
const INITIAL_SYSTEM_STATE: SystemState = SystemState {
    life_support: SystemStatus::Offline,
    communications: SystemStatus::Offline,
    security: SystemStatus::Online,
    evacuation_lift: SystemStatus::Offline,
};

const INITIAL_FACILITY: Facility = Facility {
    resources: INITIAL_RESOURCES, 
    n_remaining_occupants: INITIAL_N_REMAINING_OCCUPANTS,
    systems_state: INITIAL_SYSTEM_STATE
};

struct Ui ();

fn main() {


    let mut facility = INITIAL_FACILITY;

    loop {
        handle_main_menu_option(get_main_menu_option(), &mut facility);
    }

}

impl Ui {
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
                Ui::print_error_msg("An error occurred. Please try again!");
                continue;
            }

            return String::from(buf.trim());
        }
    }

    fn prompt_user_option(label: &str, max_option: MenuOption) -> MenuOption {

        loop {

            let input = Ui::prompt_user(label);

            let result = input.parse::<MenuOption>();
            if ! result.is_ok() {
                Ui::print_error_msg("Invalid input. Please try again");
                continue;
            }

            let option: MenuOption = result.expect("Invalid input");
            if option > max_option {
                Ui::print_error_msg(&std::format!(
                    "Invalid option {option}. MenuOption must be between 1 and {max_option}"));
                continue;
            }

            return option;
        }
    }

    fn prompt_user_char(label: &str) -> char {

        loop {
            let input_char = Ui::prompt_user(label);

            if input_char.len() != 1 {
                Ui::print_error_msg("Invalid input: Please enter a character to continue");
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

    fn confirm_action(info: &str) -> bool {

        loop {

            let input = Ui::prompt_user(&std::format!(
                "{info}. Do you wish to contine? [Y/N]"));
            
            if input == "y" || input == "yes" { return true; }
            else if input == "n" || input == "no" { return false; }

            Ui::print_error_msg("Invalid input. Please try again!");
        }
    }


}

fn get_main_menu_option() -> MenuOption {

    Ui::print_heading("Blackout Protocol - Main Menu");
    println!("{MAIN_MENU}");

    Ui::prompt_user_option("Select an option", MAX_MAIN_MENU_OPTION)
}

fn handle_main_menu_option(option: MenuOption, facility: &mut Facility) {

    match option {
        1 => mission_start_new(facility),
        2 => main_menu_view_mission_briefing(),
        3 => main_menu_view_instructions(),
        4 => main_menu_exit_application(),
        _ => Ui::print_error_msg(&std::format!("Unknown option: {option}")),
    }
}

fn main_menu_exit_application() {
    Ui::print_info_msg("Exiting application. Bye...");
    exit(0);
}

fn main_menu_view_mission_briefing() {
    Ui::print_heading("Mission Briefing");
    println!("{MISSION_BRIEFING}");
}

fn main_menu_view_instructions() {
    Ui::print_heading("How to Play");
    println!("{INSTRUCTIONS}");
}

impl SystemStatus {
    
    fn get_label(&self) -> String {
        match self {
            SystemStatus::Online   => String::from("Online"),
            SystemStatus::Offline  => String::from("Offline"),
            SystemStatus::Damaged  => String::from("Damaged"),
        }
    }
}

impl Facility {
    fn get_evacuated_occupants(&self) -> u8 {
        INITIAL_N_REMAINING_OCCUPANTS - self.n_remaining_occupants
    }

    fn print(&self) {

        let Facility { resources, n_remaining_occupants, systems_state } = self;

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
            self.get_time_elapsed(),
            resources.emergency_power, 
            resources.fuel,

            &systems_state.life_support.get_label(),
            &systems_state.communications.get_label(),
            &systems_state.security.get_label(),
            &systems_state.evacuation_lift.get_label(),

            n_remaining_occupants, self.get_evacuated_occupants()
        );
    }


    fn reset(&mut self) {
        *self = INITIAL_FACILITY;
    }

    fn decrease_fuel(&mut self, fuel_units: u32) {
        self.resources.fuel = max(self.resources.fuel.saturating_sub(fuel_units), 0);
    }

    fn decrease_power(&mut self, power_units: u32) {
        self.resources.emergency_power = max(self.resources.emergency_power.saturating_sub(power_units), 0);
    }

    fn increase_power(&mut self, power_units: u32) {
        self.resources.emergency_power = min(self.resources.emergency_power + power_units, INITIAL_FACILITY.resources.fuel);
    }

    fn decrease_time(&mut self, time_units: u32) {
        self.resources.time = max(self.resources.time.saturating_sub(time_units), 0);
    }

    fn print_inspection_report(&self) {

        let Facility { 
            resources, 
            n_remaining_occupants, 
            systems_state 
        } = self;

        Ui::print_heading("Facility Inspection Report");

        // System State

        Ui::print_subheading("System State");
        println!(
            "
    Life Support:       {}
    Communications:     {}
    Security:           {}
    Evacuation Lift:    {}
        ",
        &systems_state.life_support.get_label(),
        &systems_state.communications.get_label(),
        &systems_state.security.get_label(),
        &systems_state.evacuation_lift.get_label(),
        );

        // Resources

        Ui::print_subheading("Resources");
        println!(
            "
    Time:               {} minutes
    Emergency Power:    {} units
    Fuel:               {} units
            ",
            resources.time, resources.fuel, resources.emergency_power,
        );

        // Mission Progress

        Ui::print_subheading("Mission Progress");
        println!(
            "
    Occupants Remaining: {}
    Occupants Evacuated: {}
    Elapsed Time:        {} minutes
            ",
            n_remaining_occupants, 
            self.get_evacuated_occupants(),
            self.get_time_elapsed()
        );

        Ui::print_subheading("Summary");
        println!(
            "
    Evacuation Possible:    {}
    Communication Restored: {}
    Remaining Time:         {} minutes
            ",
            self.is_evacuation_possible(),
            self.is_communication_online(),
            self.resources.time, 
        );

    }

    fn is_life_support_online(&self) -> bool {
        if let SystemStatus::Online = self.systems_state.life_support { return true; } else { return false; }
    }

    fn is_life_support_offline(&self) -> bool {
        if let SystemStatus::Offline = self.systems_state.life_support { return true; } else { return false; }
    }

    fn is_life_support_damaged(&self) -> bool {
        if let SystemStatus::Damaged = self.systems_state.life_support { return true; } else { return false; }
    }

    fn is_communication_online(&self) -> bool {
        if let SystemStatus::Online = self.systems_state.communications { return true; } else { return false; }
    }

    fn is_communication_offline(&self) -> bool {
        if let SystemStatus::Offline = self.systems_state.communications { return true; } else { return false; }
    }

    fn is_communication_damaged(&self) -> bool {
        if let SystemStatus::Damaged = self.systems_state.communications { return true; } else { return false; }
    }

    fn is_security_online(&self) -> bool {
        if let SystemStatus::Online = self.systems_state.security { return true; } else { return false; }
    }

    fn is_security_offline(&self) -> bool {
        if let SystemStatus::Offline = self.systems_state.security { return true; } else { return false; }
    }

    fn is_security_damaged(&self) -> bool {
        if let SystemStatus::Damaged = self.systems_state.security { return true; } else { return false; }
    }

    fn is_evacuation_lift_online(&self) -> bool {
        if let SystemStatus::Online = self.systems_state.evacuation_lift { return true; } else { return false; }
    }

    fn is_evacuation_lift_offline(&self) -> bool {
        if let SystemStatus::Offline = self.systems_state.evacuation_lift { return true; } else { return false; }
    }

    fn is_evacuation_lift_damaged(&self) -> bool {
        if let SystemStatus::Damaged = self.systems_state.evacuation_lift { return true; } else { return false; }
    }

    fn get_time_elapsed(&self) -> u32 {
        INITIAL_FACILITY.resources.time - self.resources.time
    }

    fn is_evacuation_possible(&self) -> bool {
        self.is_communication_online() &&
        self.is_evacuation_lift_online() &&
        self.is_life_support_online() &&
        self.is_security_online() &&
        self.resources.fuel >= 5 &&
        self.resources.emergency_power >= 10
    }

    fn restore_life_support(&mut self) {
        let SystemStatus::Offline = self.systems_state.life_support else {
            panic!("The system must be offline to be restored");
        };

        self.systems_state.life_support = SystemStatus::Offline;
    }

    fn repair_life_support(&mut self) {
        let SystemStatus::Damaged = self.systems_state.life_support else {
            panic!("The system must be damaged to be repaired");
        };

        self.systems_state.life_support = SystemStatus::Offline;
    }

    fn shutdown_life_support(&mut self) {
        self.systems_state.life_support = SystemStatus::Damaged;
    }

    fn restore_communication(&mut self) {
        let SystemStatus::Offline = self.systems_state.communications else {
            panic!("The system must be offlined to be restored");
        };

        self.systems_state.communications = SystemStatus::Online;
    }

    fn repair_communication(&mut self) {
        let SystemStatus::Damaged = self.systems_state.communications else {
            panic!("The system must be damaged to be repaired");
        };

        self.systems_state.communications = SystemStatus::Offline;
    }

    fn shutdown_communication(&mut self) {
        self.systems_state.communications = SystemStatus::Damaged;
    }

    fn restore_security(&mut self) {
        let SystemStatus::Offline = self.systems_state.security else {
            panic!("The system must be offlined to be restored");
        };

        self.systems_state.security = SystemStatus::Online;
    }

    fn repair_security(&mut self) {
        let SystemStatus::Damaged = self.systems_state.evacuation_lift else {
            panic!("The system must be damaged to be repaired");
        };

        self.systems_state.evacuation_lift = SystemStatus::Offline;
    }

    fn shutdown_security(&mut self) {
        self.systems_state.security = SystemStatus::Damaged;
    }

    fn restore_evacuation_lift(&mut self) {
        let SystemStatus::Offline = self.systems_state.evacuation_lift else {
            panic!("The system must be offlined to be restored");
        };

        self.systems_state.evacuation_lift = SystemStatus::Online;
    }

    fn repair_evacuation_lift(&mut self) {
        let SystemStatus::Damaged = self.systems_state.evacuation_lift else {
            panic!("The system must be damaged to be repaired");
        };

        self.systems_state.evacuation_lift = SystemStatus::Offline;
    }

    fn shutdown_evacuation_lift(&mut self) {
        self.systems_state.evacuation_lift = SystemStatus::Damaged;
    }

    fn evacuate_occupants(&mut self, occupants: u8) {
        self.n_remaining_occupants = max(self.n_remaining_occupants.saturating_sub(occupants), 0);
    }
}


fn mission_start_new(facility: &mut Facility) {

    Ui::print_heading("Mission Initial Status");
    INITIAL_FACILITY.print();

    facility.reset();

    loop {
        let option = get_mission_option(facility.resources.time);

        handle_mission_option(option, facility);

        if option == 6 { break; } // End mission

        if mission_is_mission_failed(facility) {
            let reason = mission_mission_failed_reason(facility);
            mission_print_mission_failed_debrief_report(&reason, facility);

            return;
        }

        if mission_is_mission_successful(facility) {
            Ui::print_info_msg("Mission Successful!");

            let mission_score = mission_calculate_mission_score(facility);
            let mission_score_grade = mission_get_mission_score_grade(mission_score);

            Ui::print_heading(&std::format!(
                "You scored {mission_score}%. {mission_score_grade}"));
            return;
        }

    }

}

fn get_mission_option(remaining_time_mins: u32) -> MenuOption {

    Ui::print_heading("Blackout Protocol - Mission On");
    println!("{MISSION_MENU}");

    Ui::print_info_msg(&std::format!("{remaining_time_mins} minutes remaining"));

    Ui::prompt_user_option("Choose an action", MAX_MISSION_MENU_OPTION)
}

fn handle_mission_option(option: MenuOption, facility: &mut Facility) {

    match option {
        1 => mission_inspect_facility(facility),
        2 => mission_restore_system(facility),
        3 => mission_repair_system(facility),
        4 => mission_manage_emergency_power(facility),
        5 => mission_evacuate_occupants(facility),
        6 => mission_end_mission(facility),
        _ => Ui::print_error_msg(&std::format!("Unknown option: {option}")),
    }
}

fn mission_inspect_facility(facility: &mut Facility) {

    const TIME_COST: u32 = 2;

    let confirm = Ui::confirm_action(&std::format!("This action cost {TIME_COST} minutes"));

    if confirm {
        facility.decrease_time(TIME_COST);
        facility.print_inspection_report();
    }
}

fn mission_restore_system_table_heading() -> String {
    let row = std::format!("|{:^16}|{:^24}|{:^16}|{:^16}|{:^16}|", 
        "MenuOption", "System", "Fuel Units", "Power Units", "Time");
    let bottom_border = String::from("_").repeat(row.len());
    let top_border = String::from("—").repeat(row.len());

    std::format!("{top_border}\n{row}\n{bottom_border}\n")
}

fn mission_restore_system_table_row(option_char: char, system: &str, fuel_units: u32, 
    power_units: u32, time_mins: u32
) -> String {
    let row = std::format!("|{:^16}|{:^24}|{:^16}|{:^16}|{:^16}|", 
        std::format!("[{option_char}]"), system, fuel_units, power_units, time_mins);
    let bottom_border = String::from("_").repeat(row.len());

    std::format!("{row}\n{bottom_border}\n")
}

fn mission_restore_system_should_trigger_fault(facility: &Facility) -> bool {
    ((facility.get_time_elapsed() + facility.resources.fuel) % 7) == 0
}

fn mission_restore_system(facility: &mut Facility) {

    let restore_system_menu_title = mission_restore_system_table_heading();

    let life_support_option  = mission_restore_system_table_row('L', "Life Support", 8, 12, 5);
    let communications_option = mission_restore_system_table_row('C', "Communications", 5, 8, 3);
    let security_option = mission_restore_system_table_row('S', "Security", 6, 10, 4);
    let evacuation_lift_option = mission_restore_system_table_row('E', "Evacuation Lift", 10, 15, 6);

    println!("{restore_system_menu_title}");

    if facility.is_life_support_offline() {
        println!("{life_support_option}");
    }

    if facility.is_communication_offline() {
        println!("{communications_option}");
    }

    if facility.is_security_offline() {
        println!("{security_option}");
    }

    if facility.is_evacuation_lift_offline() {
        println!("{evacuation_lift_option}");
    }

    Ui::print_info_msg("Enter 'X' to cancel\n");

    loop {
        let option_char = Ui::prompt_user_char("Select system");

        if option_char == 'X' {
            Ui::print_info_msg("Operation Cancelled");
            return;
        }

        if mission_restore_system_should_trigger_fault(facility) {
            match option_char {
                'L' => {
                    facility.shutdown_life_support();
                    Ui::print_error_msg("The life support system damaged during restoration");
                },
                'C' => {
                    facility.shutdown_communication();
                    Ui::print_error_msg("The communication system damaged during restoration");
                },
                'S' => {
                    facility.shutdown_security();
                    Ui::print_error_msg("The security system damaged during restoration");
                },
                'E' => {
                    facility.shutdown_evacuation_lift();
                    Ui::print_error_msg("The evacuation system damaged during restoration");
                },
                _   => {
                    Ui::print_error_msg(&std::format!(
                        "Invalid input [{option_char}]. Please try again!"));
                    continue;
                }
            }

            return;
        }

        match option_char {
            'L' => {
                facility.decrease_fuel(8);
                facility.decrease_power(12);
                facility.decrease_time(3);

                facility.restore_life_support();

                Ui::print_info_msg("The life support system is now restored and fully online!");
            },
            'C' => {
                facility.decrease_fuel(5);
                facility.decrease_power(8);
                facility.decrease_time(3);

                facility.restore_communication();

                Ui::print_info_msg("The communication system is now restored and fully online!");
            },
            'S' => {
                facility.decrease_fuel(6);
                facility.decrease_power(10);
                facility.decrease_time(4);

                facility.restore_security();

                Ui::print_info_msg("The security system is now restored and fully online!");
            },
            'E' => {
                facility.decrease_fuel(10);
                facility.decrease_power(15);
                facility.decrease_time(6);

                facility.restore_evacuation_lift();

                Ui::print_info_msg("The evacuation lift system is now restored and fully online!");
            },
            _   => {
                Ui::print_error_msg(&std::format!(
                    "Invalid input [{option_char}]. Please try again!"));
                continue;
            }
        }

        return;
    }


}

fn mission_repair_system(facility: &mut Facility) {

    let mut repair_system_options = String::from("");

    Ui::print_subheading("Damaged Systems");

    if facility.is_life_support_damaged() {
        println!("[L] Life Support");
        repair_system_options += "L";
    }

    if facility.is_communication_damaged() {
        println!("[C] Communications");
        repair_system_options += "C";
    }

    if facility.is_security_damaged() {
        println!("[S] Security");
        repair_system_options += "S";
    }

    if facility.is_evacuation_lift_damaged() {
        println!("[E] Evacuation Lift");
        repair_system_options += "E";
    }

    if repair_system_options.is_empty() {
        Ui::print_info_msg("No damaged system found");
        return;
    }

    Ui::print_info_msg("Enter 'X' to Cancel");

    loop {

        let option_char = Ui::prompt_user_char("Select system");

        if option_char == 'X' {
            Ui::print_info_msg("Operation Cancelled");
            return;
        }

        if ! repair_system_options.contains(option_char) {
            Ui::print_error_msg("Invalid option. Please try again!");
            continue;
        }

        let confirm = Ui::confirm_action("This will cost 4 fuel units, 5 emergency power units, and 3 minutes");
        if ! confirm {
            Ui::print_info_msg("Operation Cancelled");
            return;
        }

        facility.decrease_fuel(4);
        facility.decrease_power(5);
        facility.decrease_time(3);

        match option_char {
            'L' => facility.repair_life_support(),
            'C' => facility.repair_communication(),
            'S' => facility.repair_security(),
            'E' => facility.repair_evacuation_lift(),
            _   => {
                Ui::print_info_msg(&std::format!(
                    "Invalid input: {option_char}. Please try again!"));
                continue;
            }
        }

        Ui::print_info_msg("System repaired. You may now restore it.");
        return;
    }



}

fn mission_manage_emergency_power(facility: &mut Facility) {

    const MANAGE_EMERGENCY_POWER_MENU: &str = "
1. Conserve Power - Reduce the facility's energy consumption.
    Increases emergency power by 10 units for 5 fuel units. 
    Takes 2 minutes.
2. Emergency Shutdown - Shut down nonessential systems to reduce power consumption.
    Increases emergency power by 20 units for 10 fuel units.
    Takes 4 minutes. Security system goes offline.
    ";

    println!("{MANAGE_EMERGENCY_POWER_MENU}");

    let option = Ui::prompt_user_option("Choose an option", 2);

    match option {
        1   => {
            // Conserve power

            let confirm = Ui::confirm_action("This takes 5 fuel units and 2 minutes");
            if ! confirm {
                Ui::print_info_msg("Operation Cancelled!");
                return;
            }

            facility.increase_power(10);
            facility.decrease_fuel(5);
            facility.decrease_time(2);
        },
        2   => {

            // Shutdown nonessential systems

            let confirm = Ui::confirm_action("This takes 10 fuel units and 2 minutes, and shuts down the security system");
            if ! confirm {
                Ui::print_info_msg("Operation Cancelled!");
                return;
            }

            facility.increase_power(20);
            facility.decrease_fuel(10);
            facility.decrease_time(4);
            facility.shutdown_security();

        },
        _   => Ui::print_error_msg(&std::format!("Unknown option: {option}. Please try again!"))
    }

}

fn mission_evacuate_occupants(facility: &mut Facility) {

    if ! facility.is_communication_online() {
        Ui::print_error_msg("Failed to evacuate: Communications is not online");
        return;
    }

    if ! facility.is_evacuation_lift_online() {
        Ui::print_error_msg("Failed to evacuate: Evacuation lift is not online");
        return;
    }

    if ! facility.is_life_support_online() {
        Ui::print_error_msg("Failed to evacuate: Life support is not online");
        return;
    }

    if ! facility.is_security_online() {
        Ui::print_error_msg("Failed to evacuate: Security is not online");
        return;
    }

    if ! (facility.resources.fuel >= 5) {
        Ui::print_error_msg("Failed to evacuate: Insufficient fuel");
        return;
    }

    if ! (facility.resources.emergency_power >= 10) {
        Ui::print_error_msg("Failed to evacuate: Insufficient emergency power");
        return;
    }

    facility.decrease_fuel(5);
    facility.decrease_power(10);
    facility.decrease_time(3);

    const MAX_OCCUPANTS_PER_EVACUATION: u8 = 4;

    facility.evacuate_occupants(MAX_OCCUPANTS_PER_EVACUATION);

    Ui::print_info_msg(&std::format!(
        "Evacuated {MAX_OCCUPANTS_PER_EVACUATION} persons successfully! Remaining {} occupants",
            facility.n_remaining_occupants)
        );


}

fn mission_end_mission(facility: &Facility) {
    mission_print_mission_failed_debrief_report("You ended the mission", facility);
}

fn mission_print_mission_failed_debrief_report(
    reason: &str, facility: &Facility
) {
    Ui::print_heading(reason);
    facility.print();
}

fn mission_is_mission_successful(facility: &Facility) -> bool {
    facility.get_evacuated_occupants() == INITIAL_N_REMAINING_OCCUPANTS
}

fn mission_is_mission_failed(facility: &Facility) -> bool {

    let Facility { 
        resources, 
        n_remaining_occupants: _, 
        systems_state 
    } = facility;

    if  resources.fuel == 0 ||     // fuel reaches zero
        resources.emergency_power == 0 ||     // emergency power reaches zero
        resources.time == 0        // time remaining reaches zero
    { return true; }

    // life support is damaged
    let SystemStatus::Damaged = systems_state.life_support else { return true; };

    // the player still has enough resources, mission is still on, 
    // so it is not failed

    return false;
}

fn mission_mission_failed_reason(facility: &Facility) -> String {

    if facility.resources.fuel == 0 {
        return String::from("You are out of fuel");
    }

    if facility.resources.emergency_power == 0 {
        return String::from("You are out of emergency power");
    }

    if facility.resources.time == 0 {
        return String::from("Time is up");
    }

    let SystemStatus::Damaged = facility.systems_state.life_support else {
        return String::from("Life support is broken");
    };

    return String::from("");
}

fn mission_calculate_mission_score(facility: &Facility) -> u8 {

    const EVACUATE_ALL_OCCUPANTS_SCORE:                 u8 = 40;
    const COMPLETE_IN_20_MINS_OR_LESS_SCORE:            u8 = 20;
    const FINISH_WITH_AT_LEAST_30_FUEL_UNITS_SCORE:     u8 = 15;
    const FINISH_WITH_AT_LEAST_30_POWER_UNITS_SCORE:    u8 = 15;
    const COMPLETE_MISSION_WITHOUT_SYSTEM_FAULT:        u8 = 10;

    let mut mission_score = 0;

    if facility.get_evacuated_occupants() == INITIAL_N_REMAINING_OCCUPANTS {
        mission_score += EVACUATE_ALL_OCCUPANTS_SCORE;
    }

    if facility.get_time_elapsed() <= 20 {
        mission_score += COMPLETE_IN_20_MINS_OR_LESS_SCORE;
    }

    if facility.resources.fuel >= 30 {
        mission_score += FINISH_WITH_AT_LEAST_30_FUEL_UNITS_SCORE;
    }

    if facility.resources.emergency_power >= 30 {
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
