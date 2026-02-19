pub struct TrustedContent(String);

impl TrustedContent {
    pub fn new(content: String) -> Self {
        Self(content)
    }

    pub fn as_inner(&self) -> &str {
        &self.0
    }
}
