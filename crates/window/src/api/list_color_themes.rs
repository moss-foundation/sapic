// use moss_applib::AppRuntime;

// use crate::{models::operations::ListColorThemesOutput, window::Window};

// impl<R: AppRuntime> Window<R> {
//     pub async fn list_color_themes(
//         &self,
//         _ctx: &R::AsyncContext,
//     ) -> joinerror::Result<ListColorThemesOutput> {
//         let themes = self.theme_service.themes().await;

//         Ok(ListColorThemesOutput(
//             themes.values().cloned().collect::<Vec<_>>(),
//         ))
//     }
// }
