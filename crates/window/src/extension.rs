// use joinerror::OptionExt;
// use moss_app_delegate::AppDelegate;
// use moss_applib::AppRuntime;
// use moss_common::continue_if_err;
// use moss_extension::{
//     ExtensionInfo, ExtensionPoint,
//     scanner::{ExtensionScanner, ExtensionsKind},
// };
// use moss_fs::FileSystem;
// use moss_logging::session;
// use rustc_hash::FxHashMap;
// use std::sync::Arc;

// #[allow(unused)]
// pub struct ExtensionService<R: AppRuntime> {
//     scanner: ExtensionScanner,
//     points: FxHashMap<&'static str, Box<dyn ExtensionPoint<R>>>,
//     fs: Arc<dyn FileSystem>,
// }

// impl<R: AppRuntime> ExtensionService<R> {
//     pub async fn new(
//         app_delegate: &AppDelegate<R>,
//         fs: Arc<dyn FileSystem>,
//         points: Vec<Box<dyn ExtensionPoint<R>>>,
//     ) -> joinerror::Result<Self> {
//         let points: FxHashMap<&'static str, Box<dyn ExtensionPoint<R>>> =
//             points.into_iter().map(|p| (p.key().as_str(), p)).collect();

//         let scanner = ExtensionScanner::new(
//             fs.clone(),
//             vec![
//                 (
//                     app_delegate.resource_dir().join("extensions"),
//                     ExtensionsKind::BuiltIn,
//                 ),
//                 (
//                     app_delegate.user_dir().join("extensions"),
//                     ExtensionsKind::User,
//                 ),
//             ],
//         );

//         let descriptions = scanner.scan().await?;
//         for desc in descriptions {
//             let info = ExtensionInfo {
//                 source: desc.abs_path.clone(),
//             };

//             for (key, value) in desc.contributes {
//                 if !(value.is_object() || value.is_array()) {
//                     session::warn!(format!(
//                         "Invalid contribution value: {} from extension: {}",
//                         value,
//                         desc.abs_path.display()
//                     ));
//                     continue;
//                 }

//                 let point = points
//                     .get(key.as_str())
//                     // Error should never happen, if it does, it's definitely a bug.
//                     .ok_or_join_err::<()>(format!(
//                         "Failed to find extension point for key: {}",
//                         key
//                     ))?;

//                 continue_if_err!(point.handle(app_delegate, &info, value).await, |err| {
//                     session::error!(format!(
//                         "Failed to handle contribution: {} from extension: {}",
//                         err,
//                         desc.abs_path.display()
//                     ));
//                 });
//             }
//         }

//         Ok(Self {
//             fs,
//             points,
//             scanner,
//         })
//     }
// }
