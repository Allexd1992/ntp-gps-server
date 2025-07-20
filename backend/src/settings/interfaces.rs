use async_trait::async_trait;
use tokio::io::Result;

use crate::settings::store::Settings;



#[async_trait]
pub trait IStore : Sync + Send {
    async fn Backup(&self, store:Settings)->Result<()>;
    async fn Restore(&self)->Result<Settings>;
    
}
