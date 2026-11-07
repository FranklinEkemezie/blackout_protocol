use std::io::{self, Write};

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

enum ResourceType {
    Fuel,
    EmergencyPower,
    Time
}
struct Resource {
    unit: u32,
    r#type: ResourceType
}

struct Resources {
    fuel: Resource,
    emergency_power: Resource,
    time: Resource
}

enum SystemType {
    LifeSupport,
    Communications,
    Security,
    EvacuationLift
}

struct System {
    status: SystemStatus,
    r#_type: SystemType
}

struct Systems {
    life_support: System,
    communications: System,
    security: System,
    evacuation_lift: System
}

struct Facility {
    resources: Resources,
    n_remaining_occupants: u32,
    systems: Systems
}


const INITIAL_FACILITY: Facility = Facility {
    resources: Resources::new(100, 100, 100), 
    n_remaining_occupants: 12,
    systems: Systems::new(
        SystemStatus::Offline,
        SystemStatus::Offline,
        SystemStatus::Online,
        SystemStatus::Offline
    )
};

struct Ui ();
struct MainMenu ();
struct Mission ();

fn main() {


    let mut facility = INITIAL_FACILITY;

    loop {
        MainMenu::handle_option(MainMenu::get_option(), &mut facility);
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

            print!("{label}: ");
            if ! io::stdout().flush().is_ok() {
                Ui::print_error_msg("Something went wrong");
                continue;
            }

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

impl MainMenu {
    fn get_option() -> MenuOption {

        Ui::print_heading("Blackout Protocol - Main Menu");
        println!("{MAIN_MENU}");

        Ui::prompt_user_option("Select an option", MAX_MAIN_MENU_OPTION)
    }

    fn handle_option(option: MenuOption, facility: &mut Facility) {

        match option {
            1 => Mission::start_new(facility),
            2 => MainMenu::view_mission_briefing(),
            3 => MainMenu::view_instructions(),
            4 => MainMenu::exit_application(),
            _ => Ui::print_error_msg(&std::format!("Unknown option: {option}")),
        }
    }

    fn exit_application() {
        Ui::print_info_msg("Exiting application. Bye...");
        std::process::exit(0);
    }

    fn view_mission_briefing() {
        Ui::print_heading("Mission Briefing");
        println!("{MISSION_BRIEFING}");
    }

    fn view_instructions() {
        Ui::print_heading("How to Play");
        println!("{INSTRUCTIONS}");
    }
}

impl Resource {
    
    const fn new(unit: u32, r#type: ResourceType) -> Self { 
        Self { unit, r#type } 
    }

    const fn fuel(unit: u32) -> Self { 
        Self::new(unit, ResourceType::Fuel) 
    }

    const fn emergency_power(unit: u32) -> Self { 
        Self::new(unit, ResourceType::EmergencyPower) 
    }

    const fn time(unit: u32) -> Self {
        Self::new(unit, ResourceType::Time)
    }

    fn get_unit(&self) -> u32 { self.unit }
    fn set_unit(&mut self, unit: u32) { self.unit = unit }

    fn get_type(&self) -> ResourceType {
        match self.r#type {
            ResourceType::Fuel              => ResourceType::Fuel,
            ResourceType::EmergencyPower    => ResourceType::EmergencyPower,
            ResourceType::Time              => ResourceType::Time
        }
    }

    fn increase_by(&mut self, unit: u32) { 
        self.set_unit(std::cmp::min(
            self.get_unit().saturating_add(unit), 
            INITIAL_FACILITY.get_resource_unit(self.get_type())
        ));
    }

    fn decrease_by(&mut self, unit: u32) {
        self.set_unit(std::cmp::max(self.get_unit().saturating_sub(unit), 0));
    }

    fn available(&self) -> bool { self.get_unit() > 0 }
    fn unavailable(&self) -> bool { ! self.available() }

}

impl Resources {
    
    const fn new(fuel: u32, emergency_power: u32, time: u32) -> Self {
        Self { 
            fuel: Resource::fuel(fuel), 
            emergency_power: Resource::emergency_power(emergency_power), 
            time: Resource::time(time)
        }
    }

    fn fuel(&mut self) -> &mut Resource { &mut self.fuel }
    fn emergency_power(&mut self) -> &mut Resource { &mut self.emergency_power }
    fn time(&mut self) -> &mut Resource { &mut self.time }

    fn get_fuel(&self) -> &Resource { &self.fuel }
    fn get_emergency_power(&self) -> &Resource { &self.emergency_power }
    fn get_time(&self) -> &Resource { &self.time }

    fn get_resource(&self, resource_type: ResourceType) -> &Resource {
        match resource_type {
            ResourceType::Fuel              => self.get_fuel(),
            ResourceType::EmergencyPower    => self.get_emergency_power(),
            ResourceType::Time              => self.get_time()
        }
    }

    fn increase_by(&mut self, fuel: u32, emergency_power: u32, time: u32) {
        self.fuel().increase_by(fuel);
        self.emergency_power().increase_by(emergency_power);
        self.time().increase_by(time);
    }

    fn decrease_by(&mut self, fuel: u32, emergency_power: u32, time: u32) {
        self.fuel().decrease_by(fuel);
        self.emergency_power().decrease_by(emergency_power);
        self.time().decrease_by(time);
    }

    fn any_unavailable(&self) -> bool {
        self.get_fuel().unavailable() || 
        self.get_emergency_power().unavailable() || 
        self.get_time().unavailable()
    }


}

impl SystemStatus {
    
    fn get_label(&self) -> String {
        match self {
            SystemStatus::Online   => String::from("Online"),
            SystemStatus::Offline  => String::from("Offline"),
            SystemStatus::Damaged  => String::from("Damaged"),
        }
    }

    fn label(&self) -> String { self.get_label() }

    fn is_online(&self) -> bool {
        if let Self::Online = self { return true; } else { return false; }
    }

    fn is_offline(&self) -> bool {
        if let Self::Offline = self { return true; } else { return false; }
    }

    fn is_damaged(&self) -> bool {
        if let Self::Damaged = self { return true; } else { return false; }
    }

}

impl System {

    const fn new(status: SystemStatus, r#type: SystemType) -> Self { 
        Self { status, _type: r#type } 
    }

    const fn life_support(status: SystemStatus) -> Self { 
        Self::new(status, SystemType::LifeSupport) 
    }

    const fn communications(status: SystemStatus) -> Self {
        Self::new(status, SystemType::Communications)
    }

    const fn security(status: SystemStatus) -> Self {
        Self::new(status, SystemType::Security)
    }

    const fn evacuation_lift(status: SystemStatus) -> Self {
        Self::new(status, SystemType::EvacuationLift)
    }

    fn is_online(&self) -> bool { self.status.is_online() }
    fn is_offline(&self) -> bool { self.status.is_offline() }
    fn is_damaged(&self) -> bool { self.status.is_damaged() }

    fn set_status(&mut self, status: SystemStatus) { self.status = status; }
    fn get_status(&self) -> &SystemStatus { &self.status }
    fn get_status_label(&self) -> String { self.get_status().label() }

    fn _get_type(&self) -> SystemType {
        match self.r#_type {
            SystemType::LifeSupport     => SystemType::LifeSupport,
            SystemType::Communications  => SystemType::Communications,
            SystemType::Security        => SystemType::Security,
            SystemType::EvacuationLift  => SystemType::EvacuationLift
        }
    }

    fn restore(&mut self) {
        if ! self.is_offline() {
            panic!("The system must be offline to be restored");
        };

        self.set_status(SystemStatus::Online);
    }

    fn repair(&mut self) {
        if ! self.is_damaged() {
            panic!("The system must be damaged to be repaired");
        };

        self.set_status(SystemStatus::Offline);
    }

    fn shutdown(&mut self) {
        self.set_status(SystemStatus::Damaged);
    }

    fn turnoff(&mut self) {
        if self.is_online() { 
            self.set_status(SystemStatus::Offline); 
        }
    }


}

impl Systems {

    const fn new(
        life_support: SystemStatus,
        communications: SystemStatus,
        security: SystemStatus,
        evacuation_lift: SystemStatus
    ) -> Self {
        Self { 
            life_support: System::life_support(life_support), 
            communications: System::communications(communications), 
            security: System::security(security), 
            evacuation_lift: System::evacuation_lift(evacuation_lift)
        }
    }
    
    fn life_support(&mut self) -> &mut System { &mut self.life_support }
    fn communications(&mut self) -> &mut System { &mut self.communications }
    fn security(&mut self) -> &mut System { &mut self.security }
    fn evacuation_lift(&mut self) -> &mut System { &mut self.evacuation_lift }

    fn get_life_support(&self) -> &System { &self.life_support }
    fn get_communications(&self) -> &System { &self.communications }
    fn get_security(&self) -> &System { &self.security }
    fn get_evacuation_lift(&self) -> &System { &self.evacuation_lift }

    fn system(&mut self, system_type: SystemType) -> &mut System {
        match system_type {
            SystemType::LifeSupport     => self.life_support(),
            SystemType::Communications  => self.communications(),
            SystemType::Security        => self.security(),
            SystemType::EvacuationLift  => self.evacuation_lift()
        }
    }

    fn get_system(&self, system_type: SystemType) -> &System {
        match system_type {
            SystemType::LifeSupport     => self.get_life_support(),
            SystemType::Communications  => self.get_communications(),
            SystemType::Security        => self.get_security(),
            SystemType::EvacuationLift  => self.get_evacuation_lift()
        }
    }
}

impl Facility {

    fn resources(&mut self) -> &mut Resources { &mut self.resources }
    fn systems(&mut self) -> &mut Systems { &mut self.systems }

    fn get_resources(&self) -> &Resources { &self.resources }
    fn get_n_remaining_occupants(&self) -> u32 { self.n_remaining_occupants }
    fn get_systems(&self) -> &Systems { &self.systems }

    fn system(&mut self, system_type: SystemType) -> &mut System { 
        self.systems().system(system_type) 
    }

    fn get_system(&self, system_type: SystemType) -> &System { 
        self.get_systems().get_system(system_type) 
    }

    fn get_resource_unit(&self, resource_type: ResourceType) -> u32 {
        self.get_resources().get_resource(resource_type).get_unit()
    }

    fn get_n_evacuated_occupants(&self) -> u32 {
        INITIAL_FACILITY.get_n_remaining_occupants() - self.n_remaining_occupants
    }

    fn print(&self) {

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
            self.get_resource_unit(ResourceType::Fuel),
            self.get_resource_unit(ResourceType::EmergencyPower),

            self.get_system(SystemType::LifeSupport).get_status_label(),
            self.get_system(SystemType::Communications).get_status_label(),
            self.get_system(SystemType::Security).get_status_label(),
            self.get_system(SystemType::EvacuationLift).get_status_label(),

            self.get_n_remaining_occupants(), 
            self.get_n_evacuated_occupants()
        );
    }


    fn reset(&mut self) {
        *self = INITIAL_FACILITY;
    }


    fn print_inspection_report(&self) {

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
        self.get_system(SystemType::LifeSupport).get_status_label(),
        self.get_system(SystemType::Communications).get_status_label(),
        self.get_system(SystemType::Security).get_status_label(),
        self.get_system(SystemType::EvacuationLift).get_status_label(),
    );

        // Resources

        Ui::print_subheading("Resources");
        println!(
            "
    Time:               {} minutes
    Emergency Power:    {} units
    Fuel:               {} units
            ",
            self.get_resource_unit(ResourceType::Time),
            self.get_resource_unit(ResourceType::EmergencyPower),
            self.get_resource_unit(ResourceType::Fuel),
        );

        // Mission Progress

        Ui::print_subheading("Mission Progress");
        println!(
            "
    Occupants Remaining: {}
    Occupants Evacuated: {}
    Elapsed Time:        {} minutes
            ",
            self.get_n_remaining_occupants(), 
            self.get_n_evacuated_occupants(),
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
            self.get_system(SystemType::Communications).is_online(),
            self.get_resource_unit(ResourceType::Time), 
        );

    }

    fn get_time_elapsed(&self) -> u32 {
        INITIAL_FACILITY.get_resource_unit(ResourceType::Time) - self.get_resource_unit(ResourceType::Time)
    }

    fn is_evacuation_possible(&self) -> bool {
        self.get_system(SystemType::LifeSupport).is_online() &&
        self.get_system(SystemType::Communications).is_online() &&
        self.get_system(SystemType::Security).is_online() &&
        self.get_system(SystemType::EvacuationLift).is_online() &&
        self.get_resource_unit(ResourceType::Fuel) >= 5 &&
        self.get_resource_unit(ResourceType::EmergencyPower) >= 10
    }

    fn evacuate_occupants(&mut self, occupants: u32) {
        self.n_remaining_occupants = std::cmp::max(
            self.get_n_remaining_occupants().saturating_sub(occupants), 0);
    }
}


impl Mission {
    fn start_new(facility: &mut Facility) {

        facility.reset();

        Ui::print_heading("Mission Initial Status");
        facility.print();

        loop {
            let option = Mission::get_option(facility.get_resource_unit(ResourceType::Time));

            Mission::handle_option(option, facility);

            if option == 6 { break; } // End mission

            // Check for success first
            if Mission::is_mission_successful(facility) {
                Ui::print_info_msg("Mission Successful!");

                let mission_score = Mission::calculate_mission_score(facility);
                let mission_score_grade = Mission::get_mission_score_grade(mission_score);

                Ui::print_heading(&std::format!(
                    "You scored {mission_score}%. {mission_score_grade}"));
                return;
            }

            // Then check for failure
            if Mission::is_mission_failed(facility) {
                let reason = Mission::mission_failed_reason(facility);
                Mission::print_mission_failed_debrief_report(&reason, facility);

                return;
            }

        }

    }

    fn get_option(remaining_time_mins: u32) -> MenuOption {

        Ui::print_heading("Blackout Protocol - Mission On");
        println!("{MISSION_MENU}");

        Ui::print_info_msg(&std::format!("{remaining_time_mins} minutes remaining"));

        Ui::prompt_user_option("Choose an action", MAX_MISSION_MENU_OPTION)
    }

    fn handle_option(option: MenuOption, facility: &mut Facility) {

        match option {
            1 => Mission::inspect_facility(facility),
            2 => Mission::restore_system(facility),
            3 => Mission::repair_system(facility),
            4 => Mission::manage_emergency_power(facility),
            5 => Mission::evacuate_occupants(facility),
            6 => Mission::end(facility),
            _ => Ui::print_error_msg(&std::format!("Unknown option: {option}")),
        }
    }

    fn inspect_facility(facility: &mut Facility) {

        const TIME_COST: u32 = 2;

        let confirm = Ui::confirm_action(&std::format!("This action cost {TIME_COST} minutes"));

        if confirm {
            facility.resources().decrease_by(0, 0, TIME_COST);
            facility.print_inspection_report();
        }
    }

    fn restore_system_table_heading() -> String {
        let row = std::format!("|{:^16}|{:^24}|{:^16}|{:^16}|{:^16}|", 
            "MenuOption", "System", "Fuel Units", "Power Units", "Time");
        let bottom_border = String::from("_").repeat(row.len());
        let top_border = String::from("—").repeat(row.len());

        std::format!("{top_border}\n{row}\n{bottom_border}\n")
    }

    fn restore_system_table_row(option_char: char, system: &str, fuel_units: u32, 
        power_units: u32, time_mins: u32
    ) -> String {
        let row = std::format!("|{:^16}|{:^24}|{:^16}|{:^16}|{:^16}|", 
            std::format!("[{option_char}]"), system, fuel_units, power_units, time_mins);
        let bottom_border = String::from("_").repeat(row.len());
        
        std::format!("{row}\n{bottom_border}\n")
    }

    fn restore_system_should_trigger_fault(facility: &Facility) -> bool {
        ((facility.get_time_elapsed() + facility.get_resource_unit(ResourceType::Fuel)) % 7) == 0
    }

    fn restore_system(facility: &mut Facility) {

        let restore_system_menu_title = Mission::restore_system_table_heading();

        let life_support_option  = Mission::restore_system_table_row('L', "Life Support", 8, 12, 5);
        let communications_option = Mission::restore_system_table_row('C', "Communications", 5, 8, 3);
        let security_option = Mission::restore_system_table_row('S', "Security", 6, 10, 4);
        let evacuation_lift_option = Mission::restore_system_table_row('E', "Evacuation Lift", 10, 15, 6);

        println!("{restore_system_menu_title}");

        if facility.system(SystemType::LifeSupport).is_offline() {
            println!("{life_support_option}");
        }

        if facility.system(SystemType::Communications).is_offline() {
            println!("{communications_option}");
        }

        if facility.system(SystemType::Security).is_offline() {
            println!("{security_option}");
        }

        if facility.system(SystemType::EvacuationLift).is_offline() {
            println!("{evacuation_lift_option}");
        }

        Ui::print_info_msg("Enter 'X' to cancel\n");

        loop {
            let option_char = Ui::prompt_user_char("Select system");

            if option_char == 'X' {
                Ui::print_info_msg("Operation Cancelled");
                return;
            }
            
            if Mission::restore_system_should_trigger_fault(facility) {
                match option_char {
                    'L' => {
                        facility.system(SystemType::LifeSupport).shutdown();
                        Ui::print_error_msg("The life support system damaged during restoration");
                    },
                    'C' => {
                        facility.system(SystemType::Communications).shutdown();
                        Ui::print_error_msg("The communication system damaged during restoration");
                    },
                    'S' => {
                        facility.system(SystemType::Security).shutdown();
                        Ui::print_error_msg("The security system damaged during restoration");
                    },
                    'E' => {
                        facility.system(SystemType::EvacuationLift).shutdown();
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
                    facility.resources().decrease_by(8, 8, 2);
                    facility.system(SystemType::LifeSupport).restore();

                    Ui::print_info_msg("The life support system is now restored and fully online!");

                    return;
                },
                'C' => {
                    facility.resources().decrease_by(5, 8, 3);
                    facility.system(SystemType::Communications).restore();

                    Ui::print_info_msg("The communication system is now restored and fully online!");
                },
                'S' => {
                    facility.resources().decrease_by(6, 10, 4);
                    facility.system(SystemType::Security).restore();

                    Ui::print_info_msg("The security system is now restored and fully online!");
                },
                'E' => {
                    facility.resources().decrease_by(10, 15, 6);
                    facility.system(SystemType::EvacuationLift).restore();

                    Ui::print_info_msg("The evacuation lift system is now restored and fully online!");
                },
                _   => {
                    Ui::print_error_msg(&std::format!(
                        "Invalid input [{option_char}]. Please try again!"));
                    continue;
                }
            };

            return;
        }

        
    }

    fn repair_system(facility: &mut Facility) {

        let mut repair_system_options = String::from("");

        Ui::print_subheading("Damaged Systems");

        if facility.system(SystemType::LifeSupport).is_damaged() {
            println!("[L] Life Support");
            repair_system_options += "L";
        }

        if facility.system(SystemType::Communications).is_damaged() {
            println!("[C] Communications");
            repair_system_options += "C";
        }

        if facility.system(SystemType::Security).is_damaged() {
            println!("[S] Security");
            repair_system_options += "S";
        }

        if facility.system(SystemType::Security).is_damaged() {
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

            facility.resources().decrease_by(4, 5, 3);

            match option_char {
                'L' => facility.system(SystemType::LifeSupport).repair(),
                'C' => facility.system(SystemType::Communications).repair(),
                'S' => facility.system(SystemType::Security).repair(),
                'E' => facility.system(SystemType::EvacuationLift).repair(),
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

    fn manage_emergency_power(facility: &mut Facility) {

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

                facility.resources().decrease_by(5, 0, 2);
                facility.resources().increase_by(0, 10, 0);
            },
            2   => {

                // Shutdown nonessential systems

                let confirm = Ui::confirm_action("This takes 10 fuel units and 2 minutes, and shuts down the security system");
                if ! confirm {
                    Ui::print_info_msg("Operation Cancelled!");
                    return;
                }

                facility.resources().decrease_by(10, 0, 4);
                facility.resources().increase_by(0, 20, 0);

                facility.system(SystemType::Security).shutdown();

            },
            _   => Ui::print_error_msg(&std::format!("Unknown option: {option}. Please try again!"))
        }

    }

    fn evacuate_occupants(facility: &mut Facility) {

        if ! facility.system(SystemType::Communications).is_online() {
            Ui::print_error_msg("Failed to evacuate: Communications is not online");
            return;
        }

        if ! facility.system(SystemType::Communications).is_online() {
            Ui::print_error_msg("Failed to evacuate: Evacuation lift is not available");
            return;
        }

        if ! facility.system(SystemType::LifeSupport).is_online() {
            Ui::print_error_msg("Failed to evacuate: Life support is not available");
            return;
        }

        if ! facility.system(SystemType::Security).is_online() {
            Ui::print_error_msg("Failed to evacuate: Security is not available");
            return;
        }

        if ! (facility.get_resource_unit(ResourceType::Fuel) >= 5) {
            Ui::print_error_msg("Failed to evacuate: Insufficient fuel");
            return;
        }

        if ! (facility.get_resource_unit(ResourceType::EmergencyPower) >= 10) {
            Ui::print_error_msg("Failed to evacuate: Insufficient emergency power");
            return;
        }

        facility.resources().decrease_by(5, 10, 3);

        const MAX_OCCUPANTS_PER_EVACUATION: u32 = 4;
        
        facility.evacuate_occupants(MAX_OCCUPANTS_PER_EVACUATION);

        Ui::print_info_msg(&std::format!(
            "Evacuated {MAX_OCCUPANTS_PER_EVACUATION} persons successfully! Remaining {} occupants",
                facility.n_remaining_occupants)
            );

    }

    fn end(facility: &Facility) {
        Mission::print_mission_failed_debrief_report("You ended the mission", facility);
    }

    fn print_mission_failed_debrief_report(
        reason: &str, facility: &Facility
    ) {
        Ui::print_heading(reason);
        facility.print();
    }

    fn is_mission_successful(facility: &Facility) -> bool {
        facility.get_n_evacuated_occupants() == INITIAL_FACILITY.get_n_remaining_occupants()
    }

    fn is_mission_failed(facility: &Facility) -> bool {

        if 
            facility.get_resources().any_unavailable() ||
            facility.get_system(SystemType::LifeSupport).is_damaged()
        { return true; }

        // the player still has enough resources, mission is still on, 
        // so it is not failed

        return false;
    }

    fn mission_failed_reason(facility: &Facility) -> String {
        

        if facility.get_resource_unit(ResourceType::Fuel) == 0 {
            return String::from("You are out of fuel");
        }

        if facility.get_resource_unit(ResourceType::EmergencyPower) == 0 {
            return String::from("You are out of emergency power");
        }

        if facility.get_resource_unit(ResourceType::Time) == 0 {
            return String::from("Time is up");
        }

        if let SystemStatus::Damaged = facility.get_system(SystemType::LifeSupport).get_status() {
            return String::from("Life support is broken");
        };

        return String::from("");
    }

    fn calculate_mission_score(facility: &Facility) -> u8 {

        const EVACUATE_ALL_OCCUPANTS_SCORE:                 u8 = 40;
        const COMPLETE_IN_20_MINS_OR_LESS_SCORE:            u8 = 20;
        const FINISH_WITH_AT_LEAST_30_FUEL_UNITS_SCORE:     u8 = 15;
        const FINISH_WITH_AT_LEAST_30_POWER_UNITS_SCORE:    u8 = 15;
        const COMPLETE_MISSION_WITHOUT_SYSTEM_FAULT:        u8 = 10;

        let mut mission_score = 0;

        if facility.get_n_evacuated_occupants() == INITIAL_FACILITY.get_n_remaining_occupants() {
            mission_score += EVACUATE_ALL_OCCUPANTS_SCORE;
        }

        if facility.get_time_elapsed() <= 20 {
            mission_score += COMPLETE_IN_20_MINS_OR_LESS_SCORE;
        }

        if facility.get_resource_unit(ResourceType::Fuel) >= 30 {
            mission_score += FINISH_WITH_AT_LEAST_30_FUEL_UNITS_SCORE;
        }

        if facility.get_resource_unit(ResourceType::EmergencyPower) >= 30 {
            mission_score += FINISH_WITH_AT_LEAST_30_POWER_UNITS_SCORE;
        }

        // TODO: Track system fault during game play!

        mission_score += COMPLETE_MISSION_WITHOUT_SYSTEM_FAULT;

        return mission_score;
    }

    fn get_mission_score_grade(mission_score: u8) -> String {
        if mission_score >= 90 { return String::from("Exceptional") }
        if mission_score >= 70 { return String::from("Successful"); }
        if mission_score >= 50 { return String::from("Marginal") }

        String::from("Poor")
    }

}
