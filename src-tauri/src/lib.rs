use std::sync::Mutex;
use std::time::Instant;
use tauri::{WebviewUrl, WebviewWindowBuilder};

const HOME_URL: &str = "https://sites.google.com/view/yzyplayer/yzyplayer?authuser=0";

static LAST_OPENED: Mutex<Option<(String, Instant)>> = Mutex::new(None);

fn normalize(url: &str) -> String {
    match url.parse::<tauri::Url>() {
        Ok(parsed) => format!("{}{}{}", parsed.scheme(), parsed.host_str().unwrap_or(""), parsed.path()),
        Err(_) => url.to_string(),
    }
}

fn should_open(url: &str) -> bool {
    let key = normalize(url);
    let mut last = LAST_OPENED.lock().unwrap();
    if let Some((last_key, at)) = last.as_ref() {
        if last_key == &key && at.elapsed().as_millis() < 3000 {
            return false;
        }
    }
    *last = Some((key, Instant::now()));
    true
}

fn is_google_host(host: &str) -> bool {
    const GOOGLE_SUFFIXES: [&str; 6] = [
        "google.com",
        "googleusercontent.com",
        "gstatic.com",
        "googleapis.com",
        "googleusercontent.com",
        "ggpht.com",
    ];
    GOOGLE_SUFFIXES.iter().any(|suffix| host == *suffix || host.ends_with(&format!(".{suffix}")))
}

fn open_in_app_window(app: &tauri::AppHandle, url: &str) {
    let label = format!("ext-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis());

    let _ = WebviewWindowBuilder::new(app, label, WebviewUrl::External(url.parse().unwrap()))
        .title("yzyplayer-app - Player")
        .inner_size(1000.0, 750.0)
        .position(80.0, 80.0)
        .build();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();

            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(HOME_URL.parse().unwrap()))
                .title("yzyplayer-app")
                .inner_size(1100.0, 800.0)
                .on_navigation(move |url| {
                    if url.host_str().map(is_google_host).unwrap_or(false) {
                        true
                    } else {
                        if should_open(url.as_str()) {
                            open_in_app_window(&handle, url.as_str());
                        }
                        false
                    }
                })
                .on_page_load(|webview, _payload| {
                    let home_url = HOME_URL;
                    let _ = webview.eval(&format!(
                        r#"
                        if (!window.__navShortcutsInstalled) {{
                            window.__navShortcutsInstalled = true;
                            window.addEventListener('keydown', function (e) {{
                                if (!e.metaKey) return;
                                if (e.key === 'ArrowLeft') {{ e.preventDefault(); history.back(); }}
                                else if (e.key === 'ArrowRight') {{ e.preventDefault(); history.forward(); }}
                                else if (e.key === 'Home' || (e.shiftKey && e.key.toLowerCase() === 'h')) {{
                                    e.preventDefault();
                                    window.location.href = '{home_url}';
                                }}
                            }});
                        }}
                        "#
                    ));
                })
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
