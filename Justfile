# Run ALL tests with `--nocapture` (e.g., `just test-all`)
test-all:
  cargo test -- --nocapture

# Run a SPECIFIC test file with `--nocapture` (e.g., `just test-feed test_feed`)
test TEST_NAME:
  cargo test --test {{ TEST_NAME }} -- --nocapture