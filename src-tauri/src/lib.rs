use serde::Deserialize;
use tauri::async_runtime::Mutex;
use tauri::{AppHandle, Emitter, Manager, Window};
use tauri_plugin_store::StoreExt;

#[derive(Clone)]
struct State {
    names: Vec<String>,
    mounted: bool,
    win: Window,
}

impl State {
    fn greet(&self, handle: &AppHandle) {
        let names = self.names.join(", ");
        self.win
            .set_title(format!("greeting {names}").as_str())
            .unwrap();
        handle
            .emit("greet", names)
            .map_err(|e| e.to_string())
            .unwrap();
    }
}

#[tauri::command]
async fn greet(
    app: AppHandle,
    state: tauri::State<'_, Mutex<State>>,
    name: String,
) -> Result<(), String> {
    let mut app_state = state.lock().await;

    app_state.names.push(name);

    let store = app.store("names.json").map_err(|e| e.to_string())?;

    store.set("names", app_state.names.clone());

    app_state.greet(app.app_handle());

    Ok(())
}

#[tauri::command]
async fn mount(app: AppHandle, state: tauri::State<'_, Mutex<State>>) -> Result<(), String> {
    let mut app_state = state.lock().await;

    app_state.mounted = true;

    let splash_window = app
        .get_webview_window("splashscreen")
        .ok_or("no splashscreen window".to_string())?;
    let main_window = app
        .get_webview_window("main")
        .ok_or("no main window".to_string())?;

    splash_window.close().map_err(|e| e.to_string())?;
    main_window.show().map_err(|e| e.to_string())?;

    app.emit("navigate", "greet").map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            let win = app.get_window("main").unwrap();

            let store = app.store("names.json")?;

            let names = store
                .get("names")
                .map(<Vec<String>>::deserialize)
                .transpose()?
                .unwrap_or(Vec::new());

            app.manage(Mutex::new(State {
                names,
                mounted: false,
                win,
            }));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, mount])
        .run(tauri::generate_context!("./tauri.conf.json"))
        .expect("error while running tauri application");
}
