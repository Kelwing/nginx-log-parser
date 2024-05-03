# Nginx Log Parser

Parses Nginx log files and returns the following statistics about them:
- Count of each status code
- Mean, median, and p99 for
    - all requests
    - successful requests
    - failed requests
- Endpoint that returned the single largest response body
- Endpoint with the most error responses

## Building and Running

1. Install [rust](https://rustup.rs/).
2. Run the program using `cargo run -- /path/to/nginx.log`
