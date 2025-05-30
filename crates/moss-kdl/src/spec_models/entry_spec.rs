use std::path::Path;
use anyhow::{anyhow, Result};
use kdl::{KdlDocument};
use uuid::Uuid;

use crate::foundations::http::HttpRequestFile;
use crate::parse::HttpRequestParseOptions;
use crate::spec_models::dir_spec::DirSpecificationModel;
use crate::spec_models::item_spec::{ItemContentByClass, ItemSpecificationModel};
use crate::spec_models::SpecificationMetadata;
use crate::tokens::METADATA_LIT;

#[derive(Clone)]
pub enum WorktreeEntrySpecificationModel {
    Item(ItemSpecificationModel),
    Dir(DirSpecificationModel),
}

impl WorktreeEntrySpecificationModel {
    pub fn id(&self) -> Uuid {
        match self {
            WorktreeEntrySpecificationModel::Item(item) => item.id(),
            WorktreeEntrySpecificationModel::Dir(dir) => dir.id(),
        }
    }
}

impl Into<KdlDocument> for WorktreeEntrySpecificationModel {
    fn into(self) -> KdlDocument {
        match self {
            WorktreeEntrySpecificationModel::Item(item_specification_model) => {
                item_specification_model.into()
            }
            WorktreeEntrySpecificationModel::Dir(dir_specification_model) => {
                dir_specification_model.into()
            }
        }
    }
}

impl<'a> Into<KdlDocument> for &'a WorktreeEntrySpecificationModel {
    fn into(self) -> KdlDocument {
        match self {
            WorktreeEntrySpecificationModel::Item(item_specification_model) => {
                item_specification_model.into()
            }
            WorktreeEntrySpecificationModel::Dir(dir_specification_model) => {
                dir_specification_model.into()
            }
        }
    }
}

enum Classification {

}

impl WorktreeEntrySpecificationModel {
    // FIXME: A temporary solution to find the entry_type, classification and protocol of the specification
    // entry_type: Dir specification must be named as "folder.sapic"
    // classification: Deduced from the top folder name, such as requests/
    // protocol: Http protocols have four corresponding filenames: get, post, del and put
    pub fn parse(path: &Path, text: &str) -> Result<Self> {
        let top_folder = path.components().next().ok_or(Err(anyhow!("Invalid spec file path")))?.as_os_str().to_string_lossy().to_string();
        let file_name = path.file_name().ok_or(anyhow!("Invalid spec file name"))?.to_string_lossy().to_string();

        let doc = KdlDocument::parse(text)?;
        let metadata_node = doc.get(METADATA_LIT).ok_or(Err(anyhow!("No metadata found")))?.clone();
        let metadata = SpecificationMetadata::try_from(metadata_node)?;
        match top_folder.as_str() {
            "requests" => {
                match file_name.as_str() {
                    "get.sapic" | "post.sapic" | "del.sapic" | "put.sapic" => {
                        // TODO: ParseOptions
                        let spec = HttpRequestFile::parse(doc, &HttpRequestParseOptions::default())?;
                        let content: ItemContentByClass = spec.into();
                        Ok(Self::Item(
                            ItemSpecificationModel::new(metadata, content)
                        ))
                    },
                    "folder.sapic" => {
                        // TODO: Folder specification
                        Ok(Self::Dir(
                            DirSpecificationModel::new(metadata, None)
                        ))
                    },
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!()
        }

    }
}
