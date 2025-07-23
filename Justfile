# Run ALL tests with 
test-all:
  cargo test -- --nocapture

# Run a SPECIFIC test file with 
test TEST_NAME:
  cargo test --test {{ TEST_NAME }} 

bin:
  cargo run --bin main