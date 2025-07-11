# cli-type

A blazing fast, minimal, and fun **terminal typing speed test**, built in Rust.  
Measures **WPM**, and gives you real-time feedback — all in your terminal.

---

## Install via Cargo

```bash
cargo install cli-type
```

---

## Usage

Once installed, just run:

```bash
cli-type
```

You’ll see something like:

```
Type the following:

> let boxed: Box<dyn Fn()> = Box::new(|| println!("hello from box"));

Start typing:
> 
```

And once you finish typing the sentence correctly, it will show:

```
Time taken: 7.32 seconds
WPM: 42.83
```

---

## Features

* Terminal-based typing test
* Random Rust syntax on each run
* Calculates WPM
* Supports backspace + ESC to quit
* Built using `crossterm` and `rand`

---

## Developer Notes

Clone and build locally:

```bash
git clone https://github.com/yourname/cli-type.git
cd cli-type
cargo run
```
---

## Crate Info

* Crates.io: [cli-type](https://crates.io/crates/cli-type)
* Built with: Rust 2021
* License: MIT

---

## Contribute

Pull requests welcome! To run locally:

```bash
cargo run
```

---
