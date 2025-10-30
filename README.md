# ANSI Square

A terminal animation that draws and decays colored squares in a grid pattern with customizable parameters.

## Features

- Customizable grid size and animation timing
- Multiple color options including random colors
- Optional decay (disappearing) animation
- Reproducible patterns with RNG seeding
- Infinite or finite animation cycles

## Installation

### Using Cargo
```bash
cargo install --path .
```

### Using Nix Flake

```bash
nix run github:FlorianNAdam/ansi-square
```

## Usage

```bash
# Default animation (16x16 green grid)
ansi-square

# Custom grid size with random colors
ansi-square -W 20 -H 20 -C random

# Fast animation with no decay
ansi-square -d 5 --no-decay

# Reproducible pattern with seed
ansi-square -s 12345 -C blue

# Infinite red animation
ansi-square -C red -n 0
```

## Options

- `-W, --width` - Grid width (default: 16)
- `-H, --height` - Grid height (default: 16)
- `-d, --draw-delay` - Delay between drawing cells in ms (default: 10)
- `-i, --interval-delay` - Delay between animation cycles in ms (default: 200)
- `-C, --color` - Cell color: red, green, blue, yellow, magenta, cyan, white, black, random (default: green)
- `--no-decay` - Disable the disappearing animation
- `-n, --cycles` - Number of animation cycles (0 for infinite, default: 1)
- `-s, --seed` - RNG seed for reproducible patterns

## License

MIT
