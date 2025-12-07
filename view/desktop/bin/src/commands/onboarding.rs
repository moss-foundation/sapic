use crate::commands::primitives::{App, AsyncContext, Options};
use joinerror::ResultExt;
use tauri::Window as TauriWindow;

#[allow(unused)]
#[allow(non_snake_case)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn onboarding__complete_onboarding<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: TauriWindow<R>,
    options: Options,
) -> joinerror::Result<()> {
    super::with_onboarding_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |_, app, app_delegate, _| async move {
            app.ensure_welcome(&app_delegate)
                .await
                .join_err::<()>("failed to ensure welcome window")?;

            app.close_onboarding_window()
                .await
                .join_err::<()>("failed to close onboarding window")?;

            Ok(())
        },
    )
    .await
}
