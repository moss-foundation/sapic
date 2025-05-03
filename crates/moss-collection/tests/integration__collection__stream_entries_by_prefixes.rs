mod shared;

use moss_collection::models::operations::{
    StreamEntriesByPrefixesEvent, StreamEntriesByPrefixesInput,
};

use crate::shared::set_up_test_collection;

// TODO:
// #[tokio::test]
// async fn test() {
//     let (collection_path, collection) = set_up_test_collection().await;
//     // let (evt_tx, mut evt_rx) = mpsc::unbounded_channel::<ListEntriesEvent>();
//     // let on_event = TestChannel::new(evt_tx.clone());
//     // let on_event: tauri::ipc::Channel<ListEntriesEvent> = unsafe { std::mem::transmute(on_event) };

//     let variants = vec!["endpoints", "components", "schemas", "cases", "requests"];

//     let on_event = tauri::ipc::Channel::<StreamEntriesByPrefixesEvent>::new(|event| {
//         dbg!(&event);

//         Ok(())
//     });

//     let _ = collection
//         .stream_entries_by_prefixes(on_event, StreamEntriesByPrefixesInput(variants))
//         .await;

//     // let mut all: Vec<EntryInfo> = Vec::new();

//     // while let Some(ListEntriesEvent(batch)) = evt_rx.recv().await {
//     //     dbg!(&batch);

//     //     if batch.is_empty() {
//     //         break;
//     //     }

//     //     all.extend(batch);
//     // }
// }
