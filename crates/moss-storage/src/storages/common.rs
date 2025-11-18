pub mod variable_store;

use crate::{
    primitives::segkey::SegKeyBuf,
    storage::operations::{
        GetItem, PutItem, RemoveItem, TransactionalGetItem, TransactionalPutItem,
        TransactionalRemoveItem,
    },
};
use moss_db::{Transaction, primitives::AnyValue};
use sapic_core::context::AnyAsyncContext;

#[async_trait::async_trait]
pub trait VariableStore<Context: AnyAsyncContext>:
    PutItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalPutItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + GetItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalGetItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + Send
    + Sync
{
    async fn begin_write(&self, ctx: &Context) -> joinerror::Result<Transaction>;
}
