#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod tray;

use serde_json::json;
use tauri::{AppHandle, Listener, Manager, Url, Wry};
use tauri_plugin_store::{StoreBuilder, StoreCollection};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn change_route(app: AppHandle, href: &str) -> String {
  let stores = app.state::<StoreCollection<Wry>>();
  let path = app.path().data_dir().unwrap().join("settings.json");
  let mut store = StoreBuilder::new(".settings.json").build(app.clone());
  let _ = store.load();
  let _ = store.insert("href".to_string(), json!(href));
  let _ = store.save();

  // let _ = with_store(app, stores, path, |store| {
  //   let _ = store.insert("href".to_string(), json!(href));
  //   store.save()
  // });

  format!("{}", href)
}

#[tauri::command]
async fn create_setting(handle: AppHandle) {
  if let Some(win) = handle.get_window("setting") {
    let _ = win.show();
  } else {
    let _ = tauri::WebviewWindowBuilder::new(&handle, "setting", tauri::WebviewUrl::default())
      .title("设置")
      .build()
      .unwrap();
  }
}

#[tauri::command]
fn get_store(handle: AppHandle, key: &str) -> Option<String> {
  let mut store = StoreBuilder::new(".settings.json").build(handle.clone());
  let _ = store.load();
  store.get(key).map(|t| t.to_string())
}

fn inject_style(css: &str) -> String {
  format!(
    r#"
      document.addEventListener('DOMContentLoaded', _event => {{
          const weReadStyle = `\{}`;
          const weReadStyleElement = document.createElement('style');
          weReadStyleElement.innerHTML = weReadStyle;
          document.head.appendChild(weReadStyleElement);
          console.log("inject style");
      }})
      "#,
    css
  )
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Payload {
  href: String,
}

static BASE_URL: &str = "https://weread.qq.com";

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_store::Builder::default().build())
    .setup(|app| {
      #[cfg(all(desktop, not(test)))]
      {
        let handle = app.handle();
        tray::create_tray(handle)?;
      }

      let _id = app.listen_any("location", |event| {
        let raw = event.payload();
        let payload: Result<Payload, _> = serde_json::from_str(raw);
        println!("got event-name with payload {:?}", payload);
      });

      let mut store = StoreBuilder::new(".settings.json").build(app.handle().clone());
      let _ = app
        .handle()
        .plugin(tauri_plugin_store::Builder::default().build());
      let _ = store.load();

      let href = store
        .get("href")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or("https://weread.qq.com".to_string());

      let url = Url::parse(&href).unwrap_or(BASE_URL.parse().unwrap());

      let win = tauri::WebviewWindowBuilder::new(app, "weread", tauri::WebviewUrl::External(url))
        .title("微信读书")
        .visible(false)
        .initialization_script(include_str!("../inject/preload.js"))
        .initialization_script(include_str!("../inject/event.js"))
        .initialization_script(&inject_style(include_str!("../inject/style.css")))
        .build()?;

      win.show().unwrap();

      #[cfg(debug_assertions)]
      win.open_devtools();

      let id = win.listen_any("location", |event| {
        println!("got location with payload {:?}", event.payload());
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      change_route,
      create_setting,
      get_store
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
