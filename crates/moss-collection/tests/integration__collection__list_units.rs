mod shared;

use std::time::Duration;

use moss_collection::{
    collection::Collection,
    models::operations::{EntryInfo, ListEntriesEvent, ListUnitsInput},
};
use tokio::sync::mpsc;

use crate::shared::set_up_test_collection;

#[derive(Clone)]
struct TestChannel {
    tx: mpsc::UnboundedSender<ListEntriesEvent>,
}
impl TestChannel {
    fn new(tx: mpsc::UnboundedSender<ListEntriesEvent>) -> Self {
        Self { tx }
    }
    pub fn send(&self, event: ListEntriesEvent) -> Result<(), String> {
        self.tx.send(event).map_err(|e| e.to_string())
    }
}

#[tokio::test]
async fn test() {
    let (collection_path, collection) = set_up_test_collection().await;
    // let (evt_tx, mut evt_rx) = mpsc::unbounded_channel::<ListEntriesEvent>();
    // let on_event = TestChannel::new(evt_tx.clone());
    // let on_event: tauri::ipc::Channel<ListEntriesEvent> = unsafe { std::mem::transmute(on_event) };

    let variants = vec!["requests"];

    let on_event = tauri::ipc::Channel::<ListEntriesEvent>::new(|event| {
        dbg!(&event);

        Ok(())
    });

    let _ = collection
        .list_entries(on_event, ListUnitsInput(variants))
        .await;

    tokio::time::sleep(Duration::from_secs(3)).await;

    // let mut all: Vec<EntryInfo> = Vec::new();

    // while let Some(ListEntriesEvent(batch)) = evt_rx.recv().await {
    //     dbg!(&batch);

    //     if batch.is_empty() {
    //         break;
    //     }

    //     all.extend(batch);
    // }
}
