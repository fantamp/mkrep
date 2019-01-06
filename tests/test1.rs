use std::env;
use std::process::Command;

// https://github.com/rust-lang/cargo/issues/5758
fn get_exe() -> std::path::PathBuf {
    let dir = env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        })
        .unwrap();
    dir.join(format!("mkrep{}", env::consts::EXE_SUFFIX))
}

#[test]
fn without_params() {
    let exe = get_exe();
    let out = Command::new(exe).output().unwrap();
    assert!(!out.status.success());
}

#[test]
fn normal_run() {
    let exe = get_exe();
    let out = Command::new(exe)
        .args(&["tests/data/Book1.xlsx", "Sheet1", "Buddy1"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.find("<meta charset=\"utf-8\">").is_some());
    assert!(stdout.find("Ticket to the Moon").is_none());
    assert!(out.status.success());
}

#[test]
fn text_format() {
    let exe = get_exe();
    let out = Command::new(exe)
        .args(&[
            "tests/data/Book1.xlsx",
            "Sheet1",
            "Buddy1",
            "--format",
            "text",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.find("<meta charset=\"utf-8\">").is_none());
    assert!(out.status.success());
}

#[test]
fn order_by_rank() {
    let exe = get_exe();
    let out = Command::new(exe)
        .args(&[
            "tests/data/Book1.xlsx",
            "Sheet1",
            "Buddy1",
            "--format",
            "text",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let p1 = stdout
        .find("Глобальное потепление")
        .unwrap();
    let p2 = stdout.find("Rank 50").unwrap();
    let p3 = stdout.find("Rank 25").unwrap();
    let p4 = stdout.find("Очистка от снего").unwrap();
    assert!(p1 < p2);
    assert!(p2 < p3);
    assert!(p3 < p4);
    assert!(out.status.success());
}

#[test]
fn no_rank_no_output() {
    let exe = get_exe();
    let out = Command::new(exe)
        .args(&["tests/data/Book1.xlsx", "Sheet1", "Team"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.find("Rank 25").is_none());
    assert!(stdout.find("Очистка от снего").is_none());
    assert!(stdout.find("Rank 50").is_none());
    assert!(stdout
        .find("Глобальное потепление")
        .is_some());
    assert!(out.status.success());
}

#[test]
fn specific_texts_are_really_specific() {
    let out = Command::new(get_exe())
        .args(&["tests/data/Book1.xlsx", "Sheet1", "Buddy1"])
        .output()
        .unwrap();
    let stdout1 = String::from_utf8_lossy(&out.stdout);
    let out = Command::new(get_exe())
        .args(&["tests/data/Book1.xlsx", "Sheet1", "Team"])
        .output()
        .unwrap();
    let stdout2 = String::from_utf8_lossy(&out.stdout);
    // test text for Buddy1
    assert!(stdout1
        .find("Глобальное потепление")
        .is_some());
    assert!(stdout1.find("Судя по морозу, полного потепления еще не наступило").is_some());
    assert!(stdout1
        .find("Увеличить закупки валенок")
        .is_some());
    assert!(stdout1
        .find("Валенки в обмен на продовольствие")
        .is_none());
    // test text for Team
    assert!(stdout2
        .find("Глобальное потепление")
        .is_some());
    assert!(stdout2.find("Судя по морозу, полного потепления еще не наступило").is_some());
    assert!(stdout2
        .find("Увеличить закупки валенок")
        .is_none());
    assert!(stdout2
        .find("Валенки в обмен на продовольствие")
        .is_some());
}

#[test]
fn x_in_rank_is_ok() {
    let exe = get_exe();
    let out = Command::new(exe)
        .args(&["tests/data/Book1.xlsx", "Sheet1", "Team"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.find("Xrank title").is_some());
    assert!(out.status.success());
}

#[test]
fn week_52() {
    let exe = get_exe();
    let out = Command::new(exe)
        .args(&[
            "tests/data/Book1.xlsx",
            "Sheet1",
            "Buddy1",
            "--year",
            "2018",
            "--week",
            "52",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.find("Ticket to the Moon").is_some());
    assert!(out.status.success());
}

#[test]
fn absent_rcpt_specific_fields() {
    let exe = get_exe();
    let out = Command::new(exe)
        .args(&[
            "tests/data/no_rcpt_specific_fields.xlsx",
            "Sheet1",
            "Buddy1",
        ])
        .output()
        .unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.find("records must have 'Buddy1:Rank' field").is_some());
    assert!(!out.status.success());
}

#[test]
fn bad_rank() {
    let exe = get_exe();
    let out = Command::new(exe)
        .args(&["tests/data/bad_rank.xlsx", "Sheet1", "Buddy1"])
        .output()
        .unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err
        .find("failed to parse 'zz' value in Rank field: invalid digit found in string")
        .is_some());
    assert!(!out.status.success());
}
