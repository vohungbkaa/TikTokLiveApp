use std::collections::{HashSet, VecDeque};
use std::time::{Instant, Duration};

struct DedupeEntry {
    hash: String,
    timestamp: Instant,
}

pub struct Deduplicator {
    entries: VecDeque<DedupeEntry>,
    seen: HashSet<String>,
    window: Duration,
}

impl Deduplicator {
    pub fn new(window_secs: u64) -> Self {
        Self {
            entries: VecDeque::new(),
            seen: HashSet::new(),
            window: Duration::from_secs(window_secs),
        }
    }

    pub fn is_duplicate(&mut self, key: &str) -> bool {
        let now = Instant::now();
        
        while let Some(entry) = self.entries.front() {
            if now.duration_since(entry.timestamp) > self.window {
                self.seen.remove(&entry.hash);
                self.entries.pop_front();
            } else {
                break;
            }
        }
        
        if self.seen.contains(key) {
            true
        } else {
            self.seen.insert(key.to_string());
            self.entries.push_back(DedupeEntry {
                hash: key.to_string(),
                timestamp: now,
            });
            false
        }
    }
}
