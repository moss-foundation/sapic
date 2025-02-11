use std::sync::Arc;
use std::time::Duration;
use moss_cache::backend::moka::MokaBackend;
use moss_cache::Cache;

const STATE_CACHE_TTL: Duration = Duration::from_secs(60 * 3);
const STATE_MAX_CAPACITY: u64 = 100;

#[derive(Clone)]
pub struct Preferences {
    //TODO
}

#[derive(Clone)]
pub struct AppDefaults {
    //TODO
}

pub struct AppStateManager {
    pub cache: Arc<Cache<MokaBackend>>,
    pub preferences: Preferences,
    pub defaults: AppDefaults,
}

impl AppStateManager {
    pub fn new() -> Self {
        let cache = Cache::new(MokaBackend::new(STATE_MAX_CAPACITY, STATE_CACHE_TTL));

        Self {
            cache: Arc::new(cache),
            preferences: Preferences {},
            defaults: AppDefaults {},
        }
    }

    // TODO: command mechanism?
    // pub fn get_command(&self, id: &ReadOnlyStr) -> Option<CommandHandler> {
    //     self.contributions
    //         .commands
    //         .get(id)
    //         .map(|cmd| Arc::clone(&cmd))
    // }
}
