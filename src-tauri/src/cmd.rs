use std::process::{Command};
use tauri::command;

#[command]
pub fn hello_world_test(event: String) -> Option<String> {
  let stdout = hello_world(event);
  println!("{}, =======data on rust", stdout);
  Some(stdout)
}

pub fn hello_world(event: String) -> String {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args([
        "/C",
        format!("echo {}", event.to_string()).as_str(),
      ])
      .output()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg(format!("echo {}", event.to_string()).as_str(),)
      .output()
      .expect("failed to execute process")
  };
  let stdout = String::from_utf8(output.stdout).unwrap();
  return stdout;  
}