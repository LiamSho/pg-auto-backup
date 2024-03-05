pub trait Storage {
    async fn preflight_check(&self);
    async fn save_file(&self, local_temp_file: &str, save_path: &str, file_name: String);
}
