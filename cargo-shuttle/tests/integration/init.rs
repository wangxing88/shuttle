use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

use cargo_shuttle::{Shuttle, ShuttleArgs};
use clap::Parser;
use indoc::indoc;
use tempfile::Builder;

// quite high timeout since the template is being cloned over network
const EXPECT_TIMEOUT_MS: u64 = 10000;

#[tokio::test]
async fn non_interactive_basic_init() {
    let temp_dir = Builder::new().prefix("basic-init").tempdir().unwrap();
    // Sleep to give time for the directory to finish creating
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    let temp_dir_path = temp_dir.path().to_owned();

    let args = ShuttleArgs::parse_from([
        "cargo-shuttle",
        "--api-url",
        "http://shuttle.invalid:80",
        "init",
        "--api-key",
        "dh9z58jttoes3qvt",
        "--name",
        "my-project",
        "--template",
        "none",
        temp_dir_path.to_str().unwrap(),
    ]);
    Shuttle::new().unwrap().run(args).await.unwrap();

    let cargo_toml = read_to_string(temp_dir_path.join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains("name = \"my-project\""));
    assert!(cargo_toml.contains("shuttle-runtime = "));

    // CI DEBUG: Dropping the underlying cargo generate `ScopedWorkingDirectory`
    // attempts to enter the original (temp) working directory again.
    // If both are dropped at the same time, the test can sometimes crash.
    // Added sleep here to prevent.
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    drop(temp_dir);
}

#[tokio::test]
async fn non_interactive_rocket_init() {
    let temp_dir = Builder::new().prefix("rocket-init").tempdir().unwrap();
    // Sleep to give time for the directory to finish creating
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    let temp_dir_path = temp_dir.path().to_owned();

    let args = ShuttleArgs::parse_from([
        "cargo-shuttle",
        "--api-url",
        "http://shuttle.invalid:80",
        "init",
        "--api-key",
        "dh9z58jttoes3qvt",
        "--name",
        "my-project",
        "--template",
        "rocket",
        temp_dir_path.to_str().unwrap(),
    ]);
    Shuttle::new().unwrap().run(args).await.unwrap();

    assert_valid_rocket_project(temp_dir_path.as_path(), "my-project");

    // CI DEBUG: Dropping the underlying cargo generate `ScopedWorkingDirectory`
    // attempts to enter the original (temp) working directory again.
    // If both are dropped at the same time, the test can sometimes crash.
    // Added sleep here to prevent.
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    drop(temp_dir);
}

#[tokio::test]
async fn non_interactive_init_with_from_url() {
    let temp_dir = Builder::new().prefix("basic-init-from").tempdir().unwrap();
    // Sleep to give time for the directory to finish creating
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    let temp_dir_path = temp_dir.path().to_owned();

    let args = ShuttleArgs::parse_from([
        "cargo-shuttle",
        "--api-url",
        "http://shuttle.invalid:80",
        "init",
        "--api-key",
        "dh9z58jttoes3qvt",
        "--name",
        "my-project",
        "--from",
        "https://github.com/shuttle-hq/shuttle-examples",
        "--subfolder",
        "tower/hello-world",
        temp_dir_path.to_str().unwrap(),
    ]);
    Shuttle::new().unwrap().run(args).await.unwrap();

    let cargo_toml = read_to_string(temp_dir_path.join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains("name = \"my-project\""));
    assert!(cargo_toml.contains("shuttle-tower ="));

    // CI DEBUG: Dropping the underlying cargo generate `ScopedWorkingDirectory`
    // attempts to enter the original (temp) working directory again.
    // If both are dropped at the same time, the test can sometimes crash.
    // Added sleep here to prevent.
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    drop(temp_dir);
}

#[tokio::test]
async fn non_interactive_init_with_from_gh() {
    let temp_dir = Builder::new().prefix("basic-init-from").tempdir().unwrap();
    // Sleep to give time for the directory to finish creating
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    let temp_dir_path = temp_dir.path().to_owned();

    let args = ShuttleArgs::parse_from([
        "cargo-shuttle",
        "--api-url",
        "http://shuttle.invalid:80",
        "init",
        "--api-key",
        "dh9z58jttoes3qvt",
        "--name",
        "my-project",
        "--from",
        "gh:shuttle-hq/shuttle-examples",
        "--subfolder",
        "tower/hello-world",
        temp_dir_path.to_str().unwrap(),
    ]);
    Shuttle::new().unwrap().run(args).await.unwrap();

    let cargo_toml = read_to_string(temp_dir_path.join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains("name = \"my-project\""));
    assert!(cargo_toml.contains("shuttle-tower ="));

    // CI DEBUG: Dropping the underlying cargo generate `ScopedWorkingDirectory`
    // attempts to enter the original (temp) working directory again.
    // If both are dropped at the same time, the test can sometimes crash.
    // Added sleep here to prevent.
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    drop(temp_dir);
}

#[tokio::test]
async fn non_interactive_init_with_from_repo_name() {
    let temp_dir = Builder::new().prefix("basic-init-from").tempdir().unwrap();
    // Sleep to give time for the directory to finish creating
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    let temp_dir_path = temp_dir.path().to_owned();

    let args = ShuttleArgs::parse_from([
        "cargo-shuttle",
        "--api-url",
        "http://shuttle.invalid:80",
        "init",
        "--api-key",
        "dh9z58jttoes3qvt",
        "--name",
        "my-project",
        "--from",
        "shuttle-hq/shuttle-examples",
        "--subfolder",
        "tower/hello-world",
        temp_dir_path.to_str().unwrap(),
    ]);
    Shuttle::new().unwrap().run(args).await.unwrap();

    let cargo_toml = read_to_string(temp_dir_path.join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains("name = \"my-project\""));
    assert!(cargo_toml.contains("shuttle-tower ="));

    // CI DEBUG: Dropping the underlying cargo generate `ScopedWorkingDirectory`
    // attempts to enter the original (temp) working directory again.
    // If both are dropped at the same time, the test can sometimes crash.
    // Added sleep here to prevent.
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    drop(temp_dir);
}

#[tokio::test]
async fn non_interactive_init_with_from_local_path() {
    let temp_dir = Builder::new().prefix("basic-init-from").tempdir().unwrap();
    // Sleep to give time for the directory to finish creating
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    let temp_dir_path = temp_dir.path().to_owned();

    let args = ShuttleArgs::parse_from([
        "cargo-shuttle",
        "--api-url",
        "http://shuttle.invalid:80",
        "init",
        "--api-key",
        "dh9z58jttoes3qvt",
        "--name",
        "my-project",
        "--from",
        "../examples", // for some reason, cargo runs the test from the cargo-shuttle folder.
        "--subfolder",
        "tower/hello-world",
        temp_dir_path.to_str().unwrap(),
    ]);
    Shuttle::new().unwrap().run(args).await.unwrap();

    let cargo_toml = read_to_string(temp_dir_path.join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains("name = \"my-project\""));
    assert!(cargo_toml.contains("shuttle-tower ="));

    // CI DEBUG: Dropping the underlying cargo generate `ScopedWorkingDirectory`
    // attempts to enter the original (temp) working directory again.
    // If both are dropped at the same time, the test can sometimes crash.
    // Added sleep here to prevent.
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    drop(temp_dir);
}

#[test]
fn interactive_rocket_init() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = Builder::new().prefix("rocket-init").tempdir().unwrap();
    // Sleep to give time for the directory to finish creating
    std::thread::sleep(std::time::Duration::from_millis(500));
    let temp_dir_path = temp_dir.path().to_owned();

    let bin_path = assert_cmd::cargo::cargo_bin("cargo-shuttle");
    let mut command = Command::new(bin_path);
    command.args([
        "--api-url",
        "http://shuttle.invalid:80",
        "init",
        "--api-key",
        "dh9z58jttoes3qvt",
    ]);
    let mut session = rexpect::session::spawn_command(command, Some(EXPECT_TIMEOUT_MS))?;

    session.exp_string("What do you want to name your project?")?;
    session.exp_string("Project name")?;
    session.send_line("my-project")?;
    session.exp_string("Where should we create this project?")?;
    session.exp_string("Directory")?;
    session.send_line(temp_dir_path.to_str().unwrap())?;
    session.exp_string(
        "Shuttle works with a range of web frameworks. Which one do you want to use?",
    )?;
    session.exp_string("Framework")?;
    // Partial input should be enough to match "rocket"
    session.send_line("roc")?;
    session.exp_string("Creating project")?;
    session.exp_string("Do you want to create the project environment on Shuttle?")?;
    session.send("n")?;
    session.flush()?;
    session.exp_string("no")?;

    assert_valid_rocket_project(temp_dir_path.as_path(), "my-project");

    Ok(())
}

#[test]
fn interactive_rocket_init_manually_choose_template() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = Builder::new().prefix("rocket-init").tempdir().unwrap();
    // Sleep to give time for the directory to finish creating
    std::thread::sleep(std::time::Duration::from_millis(500));
    let temp_dir_path = temp_dir.path().to_owned();

    let bin_path = assert_cmd::cargo::cargo_bin("cargo-shuttle");
    let mut command = Command::new(bin_path);
    command.args([
        "--api-url",
        "http://shuttle.invalid:80",
        "init",
        "--api-key",
        "dh9z58jttoes3qvt",
    ]);
    let mut session = rexpect::session::spawn_command(command, Some(EXPECT_TIMEOUT_MS))?;

    session.exp_string("What do you want to name your project?")?;
    session.exp_string("Project name")?;
    session.send_line("my-project")?;
    session.exp_string("Where should we create this project?")?;
    session.exp_string("Directory")?;
    session.send_line(temp_dir_path.to_str().unwrap())?;
    session.exp_string(
        "Shuttle works with a range of web frameworks. Which one do you want to use?",
    )?;
    session.exp_string("Framework")?;
    // Partial input should be enough to match "rocket"
    session.send_line("roc")?;
    session.exp_string("Creating project")?;
    session.exp_string("Do you want to create the project environment on Shuttle?")?;
    session.send("n")?;
    session.flush()?;
    session.exp_string("no")?;

    assert_valid_rocket_project(temp_dir_path.as_path(), "my-project");

    Ok(())
}

#[test]
fn interactive_rocket_init_dont_prompt_framework() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = Builder::new().prefix("rocket-init").tempdir().unwrap();
    // Sleep to give time for the directory to finish creating
    std::thread::sleep(std::time::Duration::from_millis(500));
    let temp_dir_path = temp_dir.path().to_owned();

    let bin_path = assert_cmd::cargo::cargo_bin("cargo-shuttle");
    let mut command = Command::new(bin_path);
    command.args([
        "--api-url",
        "http://shuttle.invalid:80",
        "init",
        "--api-key",
        "dh9z58jttoes3qvt",
        "--template",
        "rocket",
    ]);
    let mut session = rexpect::session::spawn_command(command, Some(EXPECT_TIMEOUT_MS))?;

    session.exp_string("What do you want to name your project?")?;
    session.exp_string("Project name")?;
    session.send_line("my-project")?;
    session.exp_string("Where should we create this project?")?;
    session.exp_string("Directory")?;
    session.send_line(temp_dir_path.to_str().unwrap())?;
    session.exp_string("Creating project")?;
    session.exp_string("Do you want to create the project environment on Shuttle?")?;
    session.send("n")?;
    session.flush()?;
    session.exp_string("no")?;

    assert_valid_rocket_project(temp_dir_path.as_path(), "my-project");

    Ok(())
}

#[test]
fn interactive_rocket_init_dont_prompt_name() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = Builder::new().prefix("rocket-init").tempdir().unwrap();
    // Sleep to give time for the directory to finish creating
    std::thread::sleep(std::time::Duration::from_millis(500));
    let temp_dir_path = temp_dir.path().to_owned();

    let bin_path = assert_cmd::cargo::cargo_bin("cargo-shuttle");
    let mut command = Command::new(bin_path);
    command.args([
        "--api-url",
        "http://shuttle.invalid:80",
        "init",
        "--api-key",
        "dh9z58jttoes3qvt",
        "--name",
        "my-project",
    ]);
    let mut session = rexpect::session::spawn_command(command, Some(EXPECT_TIMEOUT_MS))?;

    session.exp_string("Where should we create this project?")?;
    session.exp_string("Directory")?;
    session.send_line(temp_dir_path.to_str().unwrap())?;
    session.exp_string(
        "Shuttle works with a range of web frameworks. Which one do you want to use?",
    )?;
    session.exp_string("Framework")?;
    // Partial input should be enough to match "rocket"
    session.send_line("roc")?;
    session.exp_string("Creating project")?;
    session.exp_string("Do you want to create the project environment on Shuttle?")?;
    session.send("n")?;
    session.flush()?;
    session.exp_string("no")?;

    assert_valid_rocket_project(temp_dir_path.as_path(), "my-project");

    Ok(())
}

fn assert_valid_rocket_project(path: &Path, name: &str) {
    let cargo_toml = read_to_string(path.join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains(&format!("name = \"{name}\"")));
    assert!(cargo_toml.contains("shuttle-runtime = "));
    assert!(cargo_toml.contains("shuttle-rocket = "));

    let main_file = read_to_string(path.join("src").join("main.rs")).unwrap();
    let expected = indoc! {r#"
    #[macro_use]
    extern crate rocket;

    #[get("/")]
    fn index() -> &'static str {
        "Hello, world!"
    }

    #[shuttle_runtime::main]
    async fn rocket() -> shuttle_rocket::ShuttleRocket {
        let rocket = rocket::build().mount("/", routes![index]);

        Ok(rocket.into())
    }
    "#};

    assert_eq!(main_file, expected);
}
