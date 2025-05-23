use moss_common::api::{OperationError, OperationResult};
use moss_db::common::DatabaseError;
use moss_db::primitives::AnyValue;
use moss_storage::storage::operations::TransactionalGetItem;
use moss_storage::workspace_storage::entities::state_store_entities::{
    EditorPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity,
};
use serde::de::DeserializeOwned;
use tauri::Runtime as TauriRuntime;

use crate::models::types::{EditorPartState, PanelPartState, SidebarPartState};
use crate::storage::segments::{PART_EDITOR_SEGKEY, PART_PANEL_SEGKEY, PART_SIDEBAR_SEGKEY};
use crate::{models::operations::DescribeStateOutput, workspace::Workspace};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn describe_state(&self) -> OperationResult<DescribeStateOutput> {
        let item_store = self.workspace_storage.item_store();

        fn to_option<E, S>(
            result: Result<AnyValue, DatabaseError>,
            _: std::marker::PhantomData<E>,
            convert_fn: impl FnOnce(E) -> S,
        ) -> Result<Option<S>, OperationError>
        where
            E: DeserializeOwned,
        {
            match result {
                Ok(value) => {
                    let entity: E = value.deserialize()?;
                    Ok(Some(convert_fn(entity)))
                }
                Err(DatabaseError::NotFound { .. }) => Ok(None),
                Err(err) => Err(err.into()),
            }
        }

        let mut txn = self.workspace_storage.begin_read()?;

        // Get editor state
        let editor_result = TransactionalGetItem::get_item(
            item_store.as_ref(),
            &mut txn,
            PART_EDITOR_SEGKEY.to_segkey_buf(),
        );
        let editor = to_option(
            editor_result,
            std::marker::PhantomData::<EditorPartStateEntity>,
            EditorPartState::from,
        )?;

        // Get sidebar state
        let sidebar_result = TransactionalGetItem::get_item(
            item_store.as_ref(),
            &mut txn,
            PART_SIDEBAR_SEGKEY.to_segkey_buf(),
        );
        let sidebar = to_option(
            sidebar_result,
            std::marker::PhantomData::<SidebarPartStateEntity>,
            SidebarPartState::from,
        )?;

        // Get panel state
        let panel_result = TransactionalGetItem::get_item(
            item_store.as_ref(),
            &mut txn,
            PART_PANEL_SEGKEY.to_segkey_buf(),
        );
        let panel = to_option(
            panel_result,
            std::marker::PhantomData::<PanelPartStateEntity>,
            PanelPartState::from,
        )?;

        Ok(DescribeStateOutput {
            editor,
            sidebar,
            panel,
        })
    }
}
