pub enum TaskState {
    Discovery,   // Waiting for Core.Intent.Resolve response
    Interacting, // Waiting for Service.Browser.Observed
    Muxing,      // Waiting for Service.Media.Complete
}

pub struct DownloadTask {
    pub id: String,
    pub current_state: TaskState,
    pub url: String,
}
