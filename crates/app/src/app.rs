mod language;
mod profile;
mod theme;

pub mod types;

use moss_app::Window;
use moss_applib::AppRuntime;
use rustc_hash::FxHashMap;
use tokio::sync::RwLock;

pub struct App<R: AppRuntime> {
    pub(crate) windows: RwLock<FxHashMap<String, Window<R>>>,
}
