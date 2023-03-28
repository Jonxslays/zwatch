# Zwatch

A ziglings hot reloader.

## What is it?

I was learning [Zig](https://ziglang.org/), and using the
[ziglings](https://github.com/ratfactor/ziglings) repository as a fast track.

Although, at the time of publishing this repo there is no file watch/hot reload
functionality built into ziglings. Due to that I created this binary crate for
use with ziglings to enable hot reloading on the exercises.

## Installation

Zwatch has a minimum supported rust version of 1.64.0.

Currently the only way to utilize this tool is to clone and install locally:

```bash
# Clone the repo
git clone https://github.com/Jonxslays/zwatch

# Install from source
cargo install --path zwatch
```

## Usage

```bash
# Get help
zwatch -h

# Watch your ziglings directory
zwatch ~/projects/ziglings
```

## License

Zwatch is licensed under the
[MIT license](https://github.com/Jonxslays/zwatch/blob/master/LICENSE).
