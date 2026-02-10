use std::time::Duration;

use crate::sync::merge::ProgressMerger;
use crate::sync::watcher::SyncEvent;

use super::DocReaderApp;

pub(crate) fn check_sync(app: &mut DocReaderApp) {
    if let Some(watcher) = &mut app.watcher {
        if let Some(event) = watcher.poll_changes() {
            match event {
                SyncEvent::FileModified => {
                    if let Ok(remote) = app.storage.load() {
                        app.progress = ProgressMerger::merge(&app.progress, &remote);
                        app.needs_save = true;
                    }
                }
                SyncEvent::FileDeleted => {
                    app.needs_save = true;
                }
            }
        }
    }
}

pub(crate) fn maybe_save_progress(app: &mut DocReaderApp) {
    if !app.needs_save {
        return;
    }

    let save_interval = Duration::from_secs(app.settings.auto_save_interval_secs);
    if app.last_save.elapsed() >= save_interval {
        if let Err(e) = app.storage.save(&app.progress) {
            app.error_message = Some(format!("Ошибка сохранения: {}", e));
        } else {
            app.needs_save = false;
            app.last_save = std::time::Instant::now();
        }
    }
}
