use moss_applib::AppRuntime;

use crate::{
    Project,
    models::{
        operations::{
            BatchCreateResourceInput, BatchCreateResourceKind, BatchCreateResourceOutput,
        },
        primitives::FrontendResourcePath,
        types::AfterCreateResourceDescription,
    },
};

impl Project {
    pub async fn batch_create_resource<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        input: BatchCreateResourceInput,
    ) -> joinerror::Result<BatchCreateResourceOutput> {
        // Split directories from items and create directories first
        let mut items = Vec::new();
        let mut dirs = Vec::new();
        let mut resources = Vec::new();
        for entry in input.resources {
            match entry {
                BatchCreateResourceKind::Item(item) => {
                    items.push(item);
                }
                BatchCreateResourceKind::Dir(dir) => {
                    dirs.push(dir);
                }
            }
        }

        // Sort dir creation input by depth and create from shallow to deep
        dirs.sort_by(|a, b| {
            let depth_a = a.path.components().count();
            let depth_b = b.path.components().count();
            match depth_a.cmp(&depth_b) {
                std::cmp::Ordering::Equal => a.path.cmp(&b.path),
                other => other,
            }
        });

        for dir in dirs {
            let path = dir.path.clone();
            let output = self.create_dir_resource::<R>(ctx, dir).await?;
            resources.push(AfterCreateResourceDescription {
                id: output.id,
                path: FrontendResourcePath::new(path.to_path_buf()),
            });
        }

        for item in items {
            let path = item.path.clone();
            let output = self.create_item_resource::<R>(ctx, item).await?;
            resources.push(AfterCreateResourceDescription {
                id: output.id,
                path: FrontendResourcePath::new(path.to_path_buf()),
            });
        }

        Ok(BatchCreateResourceOutput { resources })
    }
}
