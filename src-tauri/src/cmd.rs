use std::{process::{Command}, path, fs::{OpenOptions, self}, io::Write};
use tauri::{command, Manager, api::notification::Notification};

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

// 创建一个用于关闭loading页面的函数
#[command]
pub async fn close_splashscreen(window: tauri::Window) {
  // 关闭loading页面
  if let Some(splashscreen) = window.get_window("splash") {
    splashscreen.close().unwrap();
  }
  // 展示主页面
  window.get_window("main").unwrap().show().unwrap();
}

#[command]
pub fn store_msg(event: String) -> Option<String> {
  let event = event + "\n";
  let report_path = "../report.txt";
  println!("======path:{:?}=====", report_path);

  let exist = path::Path::new(&report_path).exists();
  println!("====path exist===={}", exist);
  if !exist {
    let file_create = fs::File::create(&report_path);

    match  file_create {
        Ok(file) => {
          println!("===file info=====: {:?}", file);

        },
        Err(msg) => {
          println!("create file error: {}", msg);
        }
    }
  }

  let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .append(true)
    .open(report_path)
    .expect("write failed");
  
  file.write_all(event.as_bytes()).expect("write failed");

  Notification::new("temp")
  .title("Success")
  .body("the message has been stored")
  .show()
  .expect("store error");
  
  Some("ok".to_string())
}

#[command]
pub fn get_history() -> Option<Vec<String>> {
  let content = fs::read_to_string("../report.txt").expect("read failed");
  let vec: Vec<String> = content.split("\n")
    .map(|x| {
      x.to_string()
    }).collect();
  println!("=====content: {:?}=====", vec);

  Some(vec)
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
