// use anyhow::Result;
// use moss_fs::FileSystem;
// use std::{path::PathBuf, sync::Arc};

// // pub struct CollectionReader {}

// // pub struct RequestSubFolderFile {}

// // pub struct RequestFile {}

// // pub enum CollectionItem {
// //     Request {
// //         name: String,
// //         file: RequestFile,
// //     },
// //     RequestSubFolder {
// //         name: String,
// //         file: Option<RequestSubFolderFile>,
// //     },
// // }

// // pub struct ReadCollection {
// //     endpoints: i128,
// //     requests: i128,
// //     components: i128,
// //     schemas: i128,
// // }

// // impl CollectionReader {
// //     pub fn new(path: &PathBuf) -> Self {
// //         Self {}
// //     }

// //     pub async fn read(&self) -> Result<ReadCollection> {
// //         todo!()
// //     }
// // }

// const REQUESTS_DIR: &'static str = "requests";

// pub enum RequestEntry {
//     Request {},
//     Folder {},
// }

// // pub trait Visit {
// //     fn visit_requests_dir(&mut self, path: &PathBuf);
// //     fn visit_request_sub_dir(&mut self, path: &PathBuf);
// //     fn visit_request_dir(&mut self, path: &PathBuf);
// // }

// pub struct Visitor {
//     fs: Arc<dyn FileSystem>,
//     abs_path: PathBuf,
// }

// impl Visitor {
//     pub fn new(fs: Arc<dyn FileSystem>, abs_path: PathBuf) -> Self {
//         Self { fs, abs_path }
//     }

//     pub async fn visit(&self) -> Result<()> {
//         Ok(())
//     }

//     async fn visit_requests_dir(&self) -> Result<Vec<RequestEntry>> {
//         let requests_dir = self.abs_path.join(REQUESTS_DIR);
//         if !requests_dir.exists() {
//             return Ok(vec![]);
//         }

//         // let mut dir = self.fs.read_dir(&requests_dir).await?;

//         todo!()
//     }
// }
