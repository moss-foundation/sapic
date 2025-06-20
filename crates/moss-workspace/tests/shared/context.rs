use async_trait::async_trait;
use moss_applib::context::{Context, test::MockContext};
use moss_workspace::context::{AnyWorkspaceContext, Subscribe};
use tauri::test::MockRuntime;

pub struct MockWorkspaceContext {
    app: MockContext,
}

impl From<MockContext> for MockWorkspaceContext {
    fn from(app: MockContext) -> Self {
        Self { app }
    }
}

impl Context<MockRuntime> for MockWorkspaceContext {
    fn global<T>(&self) -> tauri::State<'_, T>
    where
        T: moss_applib::Global + std::any::Any + Send + Sync,
    {
        <MockContext as Context<MockRuntime>>::global(&self.app)
    }

    fn spawn<T, E, Fut, F>(
        &self,
        callback: F,
        timeout: Option<std::time::Duration>,
    ) -> moss_applib::task::Task<T, E>
    where
        Self: Sized,
        T: Send + 'static,
        E: Send + 'static,
        Fut: std::future::Future<Output = anyhow::Result<T, E>> + Send + 'static,
        F: FnOnce(Self) -> Fut + Send + 'static,
    {
        let fut = callback(MockWorkspaceContext {
            app: self.app.clone(),
        });
        moss_applib::task::Task::new(fut, timeout)
    }
}

#[async_trait]
impl AnyWorkspaceContext<MockRuntime> for MockWorkspaceContext {
    async fn subscribe(&self, _subscription: Subscribe) {}
}
