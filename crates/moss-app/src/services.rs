pub mod locale_service;
pub mod log_service;
pub mod session_service;
pub mod theme_service;
pub mod workspace_service;

pub trait AnyLocaleService: Send + Sync {}

pub trait AnyThemeService: Send + Sync {}
