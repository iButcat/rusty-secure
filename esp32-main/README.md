## Configuration

This project requires configuration for WiFi and API credentials. To set up:

1. Copy `src/config/config.template.rs` to `src/config/secrets.rs`
2. Update the values in `secrets.rs` with your actual credentials
3. Build with the `use_secrets` feature to use your configuration:
   ```bash
   cargo build --feature use_secrets
   ```

⚠️ Never commit `secrets.rs` to version control!