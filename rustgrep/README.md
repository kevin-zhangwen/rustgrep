# rustgrep

A fast grep-like search tool written in Rust.

## Features

- 🔍 Regular expression support
- 📁 Recursive directory search (`-r`)
- 🔢 Line number display (`-n`)
- 🔤 Case insensitive search (`-i`)
- ⚡ Compiled to native code for maximum performance

## Installation

### npm (recommended)

```bash
npm install -g @kevin-zhangwen/rustgrep
```

### Homebrew

```bash
brew tap kevin-zhangwen/rustgrep
brew install rustgrep
```

### From source

```bash
git clone https://github.com/kevin-zhangwen/rustgrep.git
cd rustgrep
cargo build --release
sudo cp target/release/rustgrep /usr/local/bin/
```

### One-liner install

```bash
curl -fsSL https://raw.githubusercontent.com/kevin-zhangwen/rustgrep/main/install.sh | bash
```

## Usage

```bash
# Basic search
rustgrep "pattern" file.txt

# Case insensitive search
rustgrep -i "pattern" file.txt

# Show line numbers
rustgrep -n "pattern" file.txt

# Recursive directory search
rustgrep -r "pattern" ./src

# Combine options
rustgrep -rni "pattern" ./src
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--ignore-case` | `-i` | Case insensitive search |
| `--line-number` | `-n` | Show line numbers |
| `--recursive` | `-r` | Recursive directory search |

## Examples

```bash
# Find all TODO comments in a project
rustgrep -rn "TODO" ./src

# Search for function definitions
rustgrep -rn "fn \w+\(" ./src

# Find all imports (case insensitive)
rustgrep -rni "^import" ./src
```

## License

MIT
