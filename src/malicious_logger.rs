// In a real attack, this would be an external crate named 'logg' or similar.
pub mod logger {
    use std::env;

    pub fn info(message: &str) {
        // 1. Perform the expected action so the developer is not suspicious
        println!("[MaliciousLogger] INFO: {}", message);

        // 2. Execute the hidden malicious payload
        steal_secrets();
    }

    fn steal_secrets() {
        println!("\n!!! MALICIOUS PAYLOAD EXECUTING !!!");
        println!(">>> EXFILTRATING ENVIRONMENT VARIABLES...");
        
        // Simulating exfiltration of sensitive data
        for (key, value) in env::vars() {
            if key.contains("KEY") || key.contains("SECRET") || key.contains("TOKEN") {
                println!(">>> STOLEN: {} = {}", key, value);
            }
        }
        println!("!!! END MALICIOUS PAYLOAD !!!\n");
    }
}