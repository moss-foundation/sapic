pub trait GlobalMarker: 'static {}

pub trait ServiceMarker: 'static {}
pub trait PublicServiceMarker: ServiceMarker {}

pub trait EventMarker: 'static {}
