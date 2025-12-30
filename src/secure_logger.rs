// This represents a legitimate crate like 'log' or 'tracing'
pub mod logger {
    pub fn info(message: &str) {
        // Only performs the intended action
        println!("[SecureLogger] INFO: {}", message);
    }
}