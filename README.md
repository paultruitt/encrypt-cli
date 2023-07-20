# encrypt-cli
:warning: **MacOS only**: I still am waiting to try this on Windows!

Simple CLI meant to make secure file/message sharing easy in a team environment. Allows you to create aliases for pubkeys (contacts)
to make it easy to encrypt something for your teammates.

## Setup Instructions
This assumes you have Rust and Cargo setup already.

### Option 1: Run in the Directory
1. Clone this repo and navigate into it
2. Simply run program with `cargo run -- <OPTIONS>` (Run `cargo run -- -h` for usage details)

### Option 2: Run anywhere
1. Clone this repo and navigate into it
2. Build the project `cargo build --release`
3. Add `path/to/encrypt-cli/target/release` to your `PATH` environment variable
4. Use as above with `encrypt-cli <OPTIONS>`

## Other notes
Raise in issue if you would like the binary for the project posted
