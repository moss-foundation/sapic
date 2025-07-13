pub mod ctxkeys {
    use moss_applib::context_old::ContextValue;

    use crate::models::primitives::WorkspaceId;

    /// The id of the workspace that is currently active.
    #[derive(Debug, Deref, From, PartialEq, Eq, Hash)]
    pub struct ActiveWorkspaceId(WorkspaceId);
    impl ContextValue for ActiveWorkspaceId {}

    /// The locale code that is currently active.
    #[derive(Debug, Deref, From, PartialEq, Eq, Hash)]
    pub struct ActiveLocaleCode(String);
    impl ContextValue for ActiveLocaleCode {}
}
