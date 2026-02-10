use anyhow::{Context, Result};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::time::Duration;

pub enum SyncEvent {
    FileModified,
    FileDeleted,
}

pub struct SyncWatcher {
    _watcher: RecommendedWatcher,
    rx: Receiver<Result<Event, notify::Error>>,
}

impl SyncWatcher {
    pub fn new(file_path: &Path) -> Result<Self> {
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_secs(2)),
        )
        .context("Failed to create file watcher")?;

        // Watch the parent directory to catch file creation/deletion
        if let Some(parent) = file_path.parent() {
            watcher
                .watch(parent, RecursiveMode::NonRecursive)
                .context("Failed to watch progress file directory")?;
        }

        Ok(Self {
            _watcher: watcher,
            rx,
        })
    }

    pub fn poll_changes(&mut self) -> Option<SyncEvent> {
        match self.rx.try_recv() {
            Ok(Ok(event)) => {
                use notify::EventKind;
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        Some(SyncEvent::FileModified)
                    }
                    EventKind::Remove(_) => Some(SyncEvent::FileDeleted),
                    _ => None,
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watch error: {:?}", e);
                None
            }
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => None,
        }
    }
}
