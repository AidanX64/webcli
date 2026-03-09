# WebCLI

A terminal-based CLI application for browsing an outdoor clothing product catalog. Built with Rust using crossterm and ratatui.

## Features

- Browse products by brand (The North Face, Arc'teryx, Colombia)
- Filter by category (Rain Shell, Mid Layer, Base Layer)
- View detailed product information including price, fit, weight, and sizes
- Stock availability tracking

## Running

```bash
cargo run
```

## Controls

| Key | Action |
|-----|--------|
| `j` / `k` or `↓` / `↑` | Navigate |
| `Enter` | Select / Open |
| `Esc` / `Backspace` | Go back |
| `q` | Quit |

## Project Structure

- `src/main.rs` - Application entry point and event loop
- `src/app.rs` - Application state and navigation logic
- `src/ui.rs` - TUI rendering with ratatui
- `src/catalog.rs` - Hardcoded product data

## Theme

The UI uses a warm, earthy color scheme inspired by Claude Code.
