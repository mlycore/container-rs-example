use tracing::info;
pub struct CreateCommand {
    pub id: String,
    pub oci_config: String,
}

impl CreateCommand {
    pub fn exec(&self) {
        info!("create command: {:?}, {:?}", self.id, self.oci_config);
    }
}

