//! The shared browser opener: the Help menu, the About dialog's contributor credits and the
//! updater's release page route through [`open`]. This personal build ships no community or
//! project links of its own (the upstream Discord / website / GitHub links were removed, as in
//! the sibling "by Tonynehihi" apps).

use serde_json::{Value, json};

use crate::PhotocraftApp;

/// Help-menu link commands: (id, url). Labels live in `menus::UI_COMMANDS`.
/// This personal build ships none: the Help menu carries only Check for Updates,
/// System Info and About.
pub const COMMANDS: &[(&str, &str)] = &[];

pub fn url_for(id: &str) -> Option<&'static str> {
    COMMANDS.iter().find(|c| c.0 == id).map(|c| c.1)
}

/// Open `url` in the system browser (a new tab on the web) and note it in the status bar. Prefers
/// the platform `open_url` service (reliable on Windows/macOS/Linux); falls back to `ctx.open_url`.
pub fn open(app: &mut PhotocraftApp, ctx: &egui::Context, url: &str) -> Value {
    let opened = app.services.open_url.as_ref().map(|f| f(url).is_ok()).unwrap_or(false);
    if !opened {
        ctx.open_url(egui::OpenUrl::new_tab(url));
    }
    let template = "Opened {url}";
    app.ui.status = crate::i18n::fmt(tl!(template), &[("url", url)]);
    json!({"url": url})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_open_through_the_platform_service() {
        // Issue #14: links (Help menu, About credits, updater) must open in the browser.
        // They route through the `open_url` service rather than the unreliable `ctx.open_url`.
        use std::sync::{Arc, Mutex};
        let opened: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let rec = opened.clone();
        let services = crate::Services {
            open_url: Some(Box::new(move |u: &str| {
                rec.lock().unwrap().push(u.to_string());
                Ok(())
            })),
            ..Default::default()
        };
        let mut app = PhotocraftApp::new(photocraft_engine::Session::new(), services);
        let ctx = egui::Context::default();
        // Every Help-menu link id reaches the service with its URL.
        for (id, url) in COMMANDS {
            crate::menus::invoke(&mut app, &ctx, id, json!({})).unwrap();
            assert_eq!(opened.lock().unwrap().last().map(String::as_str), Some(*url), "{id}");
        }
        // The direct open() helper (About credits / updater release page) also uses it.
        open(&mut app, &ctx, "https://example.com/release");
        assert_eq!(opened.lock().unwrap().last().map(String::as_str), Some("https://example.com/release"));
    }

    #[test]
    fn help_menu_lists_check_updates_then_separator_then_system_info_and_about() {
        let app = PhotocraftApp::new(photocraft_engine::Session::new(), Default::default());
        let help: Vec<String> = crate::menus::menu_items(&app).into_iter().filter(|i| i.path == ["Help"]).map(|i| i.id).collect();
        assert_eq!(help, ["help.checkUpdates", "---", "help.systemInfo", "help.about"]);
        // The upstream community links are gone from this build's Help menu.
        for id in ["help.discord", "help.website", "help.artcraftWebsite", "help.github", "help.reportIssue"] {
            assert!(!crate::menus::is_live(id), "{id}");
        }
        assert!(crate::menus::is_live("help.checkUpdates") && crate::menus::is_enabled(&app, "help.checkUpdates"));
    }
}
