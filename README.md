# AI-Powered CLI Tool

Convert natural language into executable commands directly from the terminal!

Open source CLI tool powered by [OpenAI](https://platform.openai.com/) (bring your own key).

Feeling rusty on your bash skills? This might help! Built using Rust 🦀

## Installation

1. Clone this repo.

2. Ensure your OpenAI API key is set.
```bash
OPENAI_API_KEY = "your-api-key-here"
```

3. Build the program.
```bash
cargo build --release
```

4. Run the program
```
cargo run --release -- how can i revert my github last commit?
```

OR

4. Add the binary `tellme` to your path. It's located in the `rusty/target/release` directory.

## Usage

```bash
tellme <argument>
```