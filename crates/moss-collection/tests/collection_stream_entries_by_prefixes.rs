mod shared;

use crate::shared::set_up_test_collection;
use moss_collection::models::operations::CreateEntryInput;
use moss_collection::models::types::Classification;
use moss_collection::models::{
    events::StreamEntriesByPrefixesEvent, operations::StreamEntriesByPrefixesInput,
};
use serde_json::Value as JsonValue;
use std::path::Path;
use tauri::ipc::InvokeResponseBody;
#[tokio::test]
async fn stream_entries_by_prefixes() {
    let (collection_path, collection) = set_up_test_collection().await;

    collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join("group"),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join("group").join("test"),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let variants = vec!["requests"];

    let on_event = tauri::ipc::Channel::<StreamEntriesByPrefixesEvent>::new(|event| {
        match event {
            InvokeResponseBody::Json(s) => {
                let value: JsonValue = serde_json::from_str(&s).unwrap();
                println!("{}", serde_json::to_string_pretty(&value).unwrap())
            }
            InvokeResponseBody::Raw(s) => {
                dbg!(s);
            }
        }

        Ok(())
    });

    let _ = collection
        .stream_entries_by_prefixes(on_event, StreamEntriesByPrefixesInput(variants))
        .await;

    // let mut all: Vec<EntryInfo> = Vec::new();

    // while let Some(ListEntriesEvent(batch)) = evt_rx.recv().await {
    //     dbg!(&batch);

    //     if batch.is_empty() {
    //         break;
    //     }

    //     all.extend(batch);
    // }
}
