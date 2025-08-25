// pub struct Profile {}

// pub struct User {}

use moss_keyring::KeyringClient;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{AccountMeta, AuthProvider, ProviderId};

pub struct UserProfile {
    // disk: RwLock<UserProfileDisk>,
    providers: RwLock<Vec<Arc<dyn AuthProvider>>>,
    keyring: Arc<dyn KeyringClient + Send + Sync>,
    // store: Arc<dyn CredentialStore>,
}

impl UserProfile {
    pub fn new(
        providers: Vec<Arc<dyn AuthProvider>>,
        keyring: Arc<dyn KeyringClient + Send + Sync>,
    ) -> Self {
        Self {
            providers: RwLock::new(providers),
            keyring,
        }
    }

    pub async fn sign_in(&self, pid: ProviderId, host: &str) -> anyhow::Result<AccountMeta> {
        let p = self.provider_for(&pid, host).await?;
        let (acc, tok) = p.login().await?;
        // self.store.save(&pid, &acc.host, &acc.id, &tok)?;
        // let mut d = self.disk.write().unwrap();
        // d.accounts.insert(acc.id.clone(), acc.clone());
        // d.host_pref
        //     .entry(acc.host.clone())
        //     .or_insert(acc.id.clone());

        Ok(acc)
    }

    // pub async fn sign_out(&self, account_id: &str) -> anyhow::Result<()> {
    //     let (acc, pid) = {
    //         let d = self.disk.read().unwrap();
    //         let acc = d.accounts.get(account_id).cloned().ok_or_else(|| anyhow::anyhow!("unknown account"))?;
    //         (acc, acc.provider.clone())
    //     };
    //     if let Some(tok) = self.store.load(&pid, &acc.host, &acc.id)? {
    //         if let Ok(p) = self.provider_for(&pid, &acc.host) {
    //             let _ = p.revoke(&acc, &tok).await;
    //         }
    //         self.store.delete(&pid, &acc.host, &acc.id)?;
    //     }
    //     let mut d = self.disk.write().unwrap();
    //     d.accounts.remove(&acc.id);
    //     d.host_pref.retain(|_h, a| a != &acc.id);
    //     Ok(())
    // }

    async fn provider_for(
        &self,
        id: &ProviderId,
        host: &str,
    ) -> anyhow::Result<Arc<dyn AuthProvider>> {
        let ps = self.providers.read().await;
        ps.iter()
            .find(|p| p.id() == *id && p.host() == host)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("no provider registered for {:?}@{}", id, host))
    }
}
