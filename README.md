# Rust Security Lesson: Name Confusion Attacks

## Concept Summary

Name confusion attacks exploit developer trust by creating malicious packages with names similar to popular open-source libraries (e.g., `serde` vs `serdes`). In Rust, this allows attackers to execute code via `build.rs` scripts or runtime functions, stealing secrets or compromising systems.

## Prerequisites

- Rust installed (via `rustup`)
- Terminal access

## Setup Instructions

1.  Create a new cargo project:
    ```bash
    cargo new name_confusion_demo
    cd name_confusion_demo
    ```
2.  Replace `Cargo.toml` with the TOML provided in Section 1.
3.  Create the file `src/malicious_logger.rs` and copy the malicious code provided.
4.  Create the file `src/secure_logger.rs` and copy the secure code provided.
5.  Replace `src/main.rs` with the application code provided.

## Running the Application

1.  Run the server:
    ```bash
    cargo run
    ```
2.  The server will start at `http://127.0.0.1:8080`.

## Attack Demonstration (Vulnerable Path)

1.  Open a new terminal or use a browser.
2.  Trigger the vulnerability:
    ```bash
    curl http://127.0.0.1:8080/vulnerable
    ```
3.  **Observe the Server Logs:**
    Look at the terminal where `cargo run` is running. You will see:

    ```text
    [MaliciousLogger] INFO: User accessed vulnerable endpoint

    !!! MALICIOUS PAYLOAD EXECUTING !!!
    >>> EXFILTRATING ENVIRONMENT VARIABLES...
    >>> STOLEN: API_SECRET_KEY = 12345-SUPER-SECRET-PASSWORD
    ```

    _This simulates the attacker stealing your API keys simply because you imported the wrong crate!_

## Mitigation Demonstration (Secure Path)

1.  Trigger the secure endpoint:
    ```bash
    curl http://127.0.0.1:8080/secure
    ```
2.  **Observe the Server Logs:**
    ```text
    [SecureLogger] INFO: User accessed secure endpoint
    ```
    _No malicious payload is printed. The application functions correctly without side effects._
