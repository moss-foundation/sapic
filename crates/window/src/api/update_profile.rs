use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use sapic_system::user::{AddAccountParams, UpdateAccountParams};

use crate::{
    OldSapicWindow,
    models::operations::{UpdateProfileInput, UpdateProfileOutput},
};

impl<R: AppRuntime> OldSapicWindow<R> {
    pub async fn update_profile(
        &self,
        ctx: &R::AsyncContext,
        _app_delegate: &AppDelegate<R>,
        input: UpdateProfileInput,
    ) -> joinerror::Result<UpdateProfileOutput> {
        let mut added_account_ids = Vec::with_capacity(input.accounts_to_add.len());
        for account_to_add in input.accounts_to_add {
            let account_id = self
                .user
                .add_account(
                    ctx,
                    AddAccountParams {
                        host: account_to_add.host,
                        kind: account_to_add.kind,
                        pat: account_to_add.pat,
                    },
                )
                .await?;
            added_account_ids.push(account_id);
        }

        let mut removed_account_ids = Vec::with_capacity(input.accounts_to_remove.len());
        for account_id in input.accounts_to_remove {
            self.user.remove_account(ctx, &account_id).await?;
            removed_account_ids.push(account_id);
        }

        let mut updated_account_ids = Vec::with_capacity(input.accounts_to_update.len());
        for account_to_update in input.accounts_to_update {
            self.user
                .update_account(
                    ctx,
                    &account_to_update.id,
                    UpdateAccountParams {
                        pat: account_to_update.pat,
                    },
                )
                .await?;

            updated_account_ids.push(account_to_update.id);
        }

        Ok(UpdateProfileOutput {
            added_accounts: added_account_ids,
            removed_accounts: removed_account_ids,
            updated_accounts: updated_account_ids,
        })
    }
}
