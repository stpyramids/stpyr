#[derive(Copy, Clone, Default)]
pub struct WizardFlags {
    pub xray:            bool,
    pub _shut_up_clippy: bool,
}

impl WizardFlags {
    pub fn toggle_xray(&mut self) {
        self.xray = !self.xray;
    }
}