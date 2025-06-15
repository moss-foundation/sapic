pub mod locale_service;
pub mod theme_service;

pub trait AnyLocaleService: Send + Sync {}

pub trait AnyThemeService: Send + Sync {}
