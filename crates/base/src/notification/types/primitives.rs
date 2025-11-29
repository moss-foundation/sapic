use serde::Serialize;
use ts_rs::TS;

/// @category Primitive
///
/// Represents where progress or transient messages are displayed.
///
/// Use `Location` to route UI messages to a specific surface within the
/// application shell. Variants are ordered from most global to most
/// ephemeral.
///
/// - `Window`: Application-wide surface (e.g., status area).
/// - `Notification`: Persistent, actionable message in the notifications UI.
/// - `Toast`: Short-lived, non-blocking overlay near the edge of the window.
///
/// # Semantics
/// * `Window` is suitable for low-noise, ambient progress (spinners, brief text).
/// * `Notification` is used for items that may require user attention or actions.
/// * `Toast` is for ephemeral hints that disappear automatically.
#[derive(Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "notification/primitives.ts")]
pub enum Location {
    /// Application-wide surface for ambient status and progress.
    ///
    /// Typical use: long-running background tasks, indexing spinners,
    /// or brief text updates tied to the whole window.
    Window,

    /// Persistent, potentially actionable message shown in the notifications UI.
    ///
    /// Messages here can include buttons and remain visible until dismissed
    /// by the user or resolved by the system.
    Notification,

    /// Short-lived, non-blocking overlay.
    ///
    /// Ideal for transient feedback (e.g., "Saved", "Copied", or quick errors).
    /// Should avoid requiring user interaction.
    Toast,
}
