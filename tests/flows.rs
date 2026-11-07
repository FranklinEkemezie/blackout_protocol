use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn run(script: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_blackout_protocol"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start blackout_protocol");

    {
        let mut stdin = child.stdin.take().expect("missing stdin");
        stdin.write_all(script.as_bytes()).expect("failed to write stdin");
    }

    let started = Instant::now();
    loop {
        if child.try_wait().expect("failed to poll child").is_some() {
            break;
        }
        if started.elapsed() > Duration::from_secs(5) {
            let _ = child.kill();
            break;
        }
        sleep(Duration::from_millis(10));
    }

    child.wait_with_output().expect("failed to collect output")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn assert_contains(output: &Output, needle: &str) {
    let text = stdout(output);
    assert!(
        text.contains(needle),
        "missing {needle:?}\n--- stdout ---\n{text}\n--- stderr ---\n{}",
        stderr(output)
    );
}

#[test]
fn main_menu_lists_required_options_and_exits() {
    let output = run("4\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    for needle in [
        "BLACKOUT PROTOCOL - MAIN MENU",
        "1. Start New Mission",
        "2. View Mission Briefing",
        "3. View Instructions",
        "4. Exit",
        "Exiting application. Bye",
    ] {
        assert_contains(&output, needle);
    }
}

#[test]
fn briefing_and_instructions_return_to_the_main_menu() {
    let output = run("2\n3\n4\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    for needle in [
        "MISSION BRIEFING",
        "OPERATION: BLACKOUT PROTOCOL",
        "HOW TO PLAY",
        "1. NAVIGATING THE MENUS",
        "Exiting application. Bye",
    ] {
        assert_contains(&output, needle);
    }
}

#[test]
fn invalid_main_menu_input_is_rejected() {
    let output = run("abc\n99\n-1\n4\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let text = stdout(&output);
    assert!(
        text.matches("[ERROR]").count() >= 3,
        "expected three rejected inputs\n{text}"
    );
    assert_contains(&output, "Exiting application. Bye");
}

#[test]
fn new_mission_starts_with_the_specified_initial_state() {
    let output = run("1\n6\n4\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    for needle in [
        "MISSION INITIAL STATUS",
        "Elapsed Time:       0 minutes",
        "Fuel:               100 units",
        "Emergency Power:    100 units",
        "Life Support:       Offline",
        "Communications:     Offline",
        "Security:           Online",
        "Evacuation Lift:    Offline",
        "Occupants Remaining: 12",
        "Occupants Evacuated: 0",
    ] {
        assert_contains(&output, needle);
    }
}

#[test]
fn mission_clock_starts_with_thirty_minutes() {
    let output = run("1\n6\n4\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_contains(&output, "30 minutes remaining");
}

#[test]
fn ending_the_mission_manually_shows_a_debrief_without_a_score() {
    let output = run("1\n6\n4\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_contains(&output, "YOU ENDED THE MISSION");
    assert_contains(&output, "Occupants Evacuated: 0");
    assert!(
        !stdout(&output).contains("You scored"),
        "a manually ended mission must not receive a completion score"
    );
}

#[test]
fn inspection_consumes_two_minutes_and_reports_readiness() {
    let output = run("1\n1\ny\n6\n4\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    for needle in [
        "FACILITY INSPECTION REPORT",
        "Elapsed Time:        2 minutes",
        "Evacuation Possible:    false",
        "Communication Restored: false",
        "Remaining Time:         28 minutes",
    ] {
        assert_contains(&output, needle);
    }
}

#[test]
fn restoring_communications_uses_the_specified_costs() {
    let output = run("1\n2\nC\n6\n4\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    for needle in [
        "communication system is now restored and fully online",
        "Elapsed Time:       3 minutes",
        "Fuel:               95 units",
        "Emergency Power:    92 units",
        "Communications:     Online",
    ] {
        assert_contains(&output, needle);
    }
}

#[test]
fn restoring_life_support_uses_the_specified_costs() {
    let output = run("1\n2\nL\n6\n4\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    for needle in [
        "life support system is now restored and fully online",
        "Elapsed Time:       5 minutes",
        "Fuel:               92 units",
        "Emergency Power:    88 units",
        "Life Support:       Online",
    ] {
        assert_contains(&output, needle);
    }
}

#[test]
fn restoring_an_already_online_system_is_rejected_without_crashing() {
    let output = run("1\n2\nS\n6\n4\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_contains(&output, "Security:           Online");
    assert!(
        !stderr(&output).contains("panicked"),
        "restoring an online system must be rejected, not panic"
    );
}

#[test]
fn repair_with_no_damaged_systems_is_rejected() {
    let output = run("1\n3\n6\n4\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_contains(&output, "No damaged system found");
}

#[test]
fn fault_damages_a_system_without_deducting_resources() {
    let script = format!("1\n{}2\nC\n6\n4\n", "1\ny\n".repeat(6));
    let output = run(&script);

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    for needle in [
        "damaged during restoration",
        "Elapsed Time:       12 minutes",
        "Fuel:               100 units",
        "Emergency Power:    100 units",
        "Communications:     Damaged",
    ] {
        assert_contains(&output, needle);
    }
}

#[test]
fn a_damaged_system_can_be_repaired_then_restored() {
    let script = format!(
        "1\n{}2\nC\n3\nC\ny\n2\nC\n6\n4\n",
        "1\ny\n".repeat(6)
    );
    let output = run(&script);

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    for needle in [
        "repair",
        "Elapsed Time:       18 minutes",
        "Fuel:               91 units",
        "Emergency Power:    87 units",
        "Communications:     Online",
    ] {
        assert_contains(&output, needle);
    }
}
