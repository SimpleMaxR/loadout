use notify_debouncer_mini::{new_debouncer, DebounceEventResult, DebouncedEvent, Debouncer};
use notify::RecursiveMode;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct Watcher {
    _debouncer: Debouncer<notify::RecommendedWatcher>,
    pub rx: Receiver<Vec<DebouncedEvent>>,
}

impl Watcher {
    /// Create a new file watcher with 500ms debounce
    pub fn new() -> anyhow::Result<Self> {
        let (tx, rx) = channel::<Vec<DebouncedEvent>>();

        let debouncer = new_debouncer(Duration::from_millis(500), move |res: DebounceEventResult| {
            if let Ok(events) = res {
                let _ = tx.send(events);
            }
        })?;

        Ok(Self {
            _debouncer: debouncer,
            rx,
        })
    }

    /// Watch a directory recursively
    pub fn watch(&mut self, path: &Path) -> anyhow::Result<()> {
        self._debouncer
            .watcher()
            .watch(path, RecursiveMode::Recursive)?;
        Ok(())
    }
}
