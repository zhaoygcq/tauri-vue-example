use std::process::{Command};
use tauri::{command};

#[command]
pub async fn menu_toggle(window: tauri::Window) {
  window.menu_handle().toggle().unwrap();
}


#[command]
pub fn hello_world_test(event: String) -> Option<String> {
  let stdout = hello_world(event);
  println!("{}, =======data on rust", stdout);
  Some(stdout)
}

#[command]
pub fn my_custom_command(event: String) -> Option<String> {
  let stdout = String::from(event + " Welcome");
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
