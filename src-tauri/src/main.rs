// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use serde::{Deserialize, Serialize};
use tauri::{
  api::dialog::ask,
  http::ResponseBuilder, 
  RunEvent, WindowEvent, 
  GlobalShortcutManager,
  Manager,
  // 自定义菜单相关
  CustomMenuItem,
  Menu,
  MenuItem,
  Submenu
};

#[derive(Serialize)]
struct Reply {
  data: String,
}

#[derive(Serialize, Deserialize)]
struct HttpPost {
  foo: String,
  bar: String,
}

#[derive(Serialize)]
struct HttpReply {
  msg: String,
  request: HttpPost,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
  #[allow(unused_mut)]
  let mut app = tauri::Builder::default()
    .setup(|app| {
      app.listen_global("clickReq", |event| {
        let msg = match event.payload() {
            Some(msg) => msg,
            None => ""
        };
        println!("=========got click with payload {:?}========", msg);
      });
      Ok(())
    })
    .on_page_load(|window, _| {
      let window_ = window.clone();
      window.listen("js-event", move |event| {
        println!("got js-event with message '{:?}'", event.payload());
        let reply = Reply {
          data: "something else".to_string(),
        };

        window_
          .emit("rust-event", Some(&reply))
          .expect("failed to emit");
      });
    })
    .register_uri_scheme_protocol("customprotocol", move |_app_handle, request| {
      if request.method() == "POST" {
        let request: HttpPost = serde_json::from_slice(request.body()).unwrap();
        return ResponseBuilder::new()
          .mimetype("application/json")
          .header("Access-Control-Allow-Origin", "*")
          .status(200)
          .body(serde_json::to_vec(&HttpReply {
            request,
            msg: "Hello from rust!".to_string(),
          })?);
      }

      ResponseBuilder::new()
        .mimetype("text/html")
        .status(404)
        .body(Vec::new())
    })
    .menu(get_menu())
    .on_menu_event(|event| {
      // 自定义菜单的点击事件
      println!("你刚才点击了:{:?}", event.menu_item_id());
    })
    // 监听来自于渲染进程的数据通信
    .invoke_handler(tauri::generate_handler![
      cmd::hello_world_test,
      cmd::my_custom_command,
      cmd::menu_toggle,
      cmd::close_splashscreen,
      cmd::store_msg,
      cmd::get_history
    ])
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

  #[cfg(target_os = "macos")]
  app.set_activation_policy(tauri::ActivationPolicy::Regular);

  app.run(|app_handle, e| match e {
    // Application is ready (triggered only once)
    RunEvent::Ready => {
      let app_handle = app_handle.clone();
      app_handle
        .global_shortcut_manager()
        .register("CmdOrCtrl+1", move || {
          let app_handle = app_handle.clone();
          let window = app_handle.get_window("main").unwrap();
          window.set_title("New title!").unwrap();
        })
        .unwrap();
    }

    // Triggered when a window is trying to close
    RunEvent::WindowEvent {
      label,
      event: WindowEvent::CloseRequested { api, ..},
      ..
    } => {
      let app_handle = app_handle.clone();
      let window = app_handle.get_window(&label).unwrap();
      // use the exposed close api, and prevent the event loop to close
      api.prevent_close();
      // ask the user if he wants to quit
      ask(
        Some(&window),
        "Tauri API",
        "Are you sure that you want to close this window?",
        move |answer| {
          if answer {
            // .close() cannot be called on the main thread
            std::thread::spawn(move || {
              app_handle.get_window(&label).unwrap().close().unwrap();
            });
          }
        },
      );
    }

    // Keep the event loop running even if all windows are closed
    // This allow us to catch system tray events when there is no window
    RunEvent::ExitRequested { api, .. } => {
      api.prevent_exit();
    }
    _ => {}
  })
}

// 自定义菜单栏
pub fn get_menu() -> Menu {
  // 创建自定义的菜单项
  #[allow(unused_mut)]
  let mut disable_item =
    CustomMenuItem::new("disable-menu", "Disable menu").accelerator("CmdOrControl+D");
  #[allow(unused_mut)]
  let mut test_item = CustomMenuItem::new("test", "Test").accelerator("CmdOrControl+T");
  
  #[allow(unused_mut)]
  let mut demo_item = CustomMenuItem::new("demo", "Demo").accelerator("CmdOrControl+E");
  
  #[cfg(target_os = "macos")]
  {
    // 可以使用由tauri提供了图标
    disable_item = disable_item.native_image(tauri::NativeImage::MenuOnState);
    test_item = test_item.native_image(tauri::NativeImage::Add);
    demo_item = demo_item.native_image(tauri::NativeImage::Add);
  }

  // create a submenu
  let my_sub_menu = Menu::new()
    .add_item(disable_item)
    .add_item(demo_item);

  let my_app_menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_submenu(Submenu::new("Sub menu", my_sub_menu));

  let test_menu = Menu::new()
    .add_item(CustomMenuItem::new(
      "selected/disabled",
      "Selected and disabled",
    ))
    .add_native_item(MenuItem::Separator)
    .add_item(test_item);

  // add all our childs to the menu (order is how they'll appear)
  Menu::new()
    .add_submenu(Submenu::new("My app", my_app_menu)) // 第一个菜单项代表当前应用，这里的title字段无效
    .add_submenu(Submenu::new("Custom menu", test_menu))
}