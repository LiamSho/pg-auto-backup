pub trait PreflightCheck {
    async fn preflight_check(&self) -> Result<(), String>;
}
