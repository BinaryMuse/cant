# `cant` - A Terminal Log Viewer

> cant (`/kænt/`), noun - a partly trimmed log

Cant is a powerful terminal-based log viewer built in Rust, designed to make log file analysis more efficient and intuitive. It provides a modern, interactive interface for viewing, searching, and filtering log files.

## Features

- [x] **Real-time Log Viewing**: View logs in real-time from both files and stdin
- **Interactive Navigation**:
  - [x] Scroll through logs using arrow keys
  - [x] Page up/down with spacebar
  - [x] Jump to specific line numbers
  - [x] Attach to bottom of logs for real-time monitoring
- [ ] **Search Capabilities**: Search through logs with intuitive keyboard shortcuts
- [ ] **Customizable Display**:
  - [x] Toggle line numbers
- [ ] **Vim-style Navigation**: Familiar keybindings for efficient navigation

## Installation

```bash
cargo install cant
```

## Usage

```bash
# View logs from a file
cant path/to/logfile.log

# View logs from stdin; you can also use `cant -` to read from stdin
tail -f /var/log/system.log | cant
```

## Keyboard Shortcuts

- `↑`/`↓`: Scroll up/down one line
- `Space`: Page down
- `b`: Attach to bottom of logs
- `N`: Toggle line numbers
- `g`: Go to specific line number
- `/`: Open search
- `q` or `Esc`: Quit
- `Ctrl+c`: Cancel current operation

## License

MIT License

## Author

Michelle Tilley <michelle@michelletilley.net> 
