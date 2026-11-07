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
