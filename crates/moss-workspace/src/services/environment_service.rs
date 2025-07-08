use std::sync::Arc;

use derive_more::{Deref, DerefMut};
use moss_common::NanoId;
use moss_environment::environment::Environment as EnvironmentHandle;

// EnvironmentId: length-10

#[derive(Deref, DerefMut)]
pub struct EnvironmentItem {
    pub id: NanoId,
    pub name: String,
    pub display_name: String,

    #[deref]
    #[deref_mut]
    pub inner: Arc<EnvironmentHandle>,
}

pub struct EnvironmentService {}

// pub async fn environments<C: Context<R>>(&self, ctx: &C) -> Result<&EnvironmentMap> {
//     let fs = <dyn FileSystem>::global::<R, C>(ctx);
//     let result = self
//         .environments
//         .get_or_try_init(|| async move {
//             let mut environments = HashMap::new();

//             let abs_path = self.abs_path.join(dirs::ENVIRONMENTS_DIR);
//             if !abs_path.exists() {
//                 return Ok(environments);
//             }

//             // TODO: restore environments cache from the database
//             let mut read_dir = fs.read_dir(&abs_path).await?;
//             while let Some(entry) = read_dir.next_entry().await? {
//                 if entry.file_type().await?.is_dir() {
//                     continue;
//                 }

//                 let entry_abs_path = entry.path();
//                 let name = entry_abs_path
//                     .file_name()
//                     .unwrap()
//                     .to_string_lossy()
//                     .to_string();
//                 let decoded_name = desanitize(&name);

//                 let environment = Environment::load(
//                     &entry_abs_path,
//                     fs.clone(),
//                     self.storage.variable_store().clone(),
//                     self.next_variable_id.clone(),
//                     environment::LoadParams {
//                         create_if_not_exists: false,
//                     },
//                 )
//                 .await?;

//                 let id = environment.id().await;
//                 let entry = EnvironmentItem {
//                     id,
//                     name,
//                     display_name: decoded_name,
//                     inner: environment,
//                 };

//                 environments.insert(id, Arc::new(entry));
//             }

//             Ok::<_, anyhow::Error>(environments)
//         })
//         .await?;

//     Ok(result)
// }
