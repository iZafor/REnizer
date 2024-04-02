<picture>
    <img src="public/images/logo/vector/default-monochrome-white.svg" alt="renizer-logo">
</picture>

This project uses [Leptos](https://github.com/leptos-rs/leptos) for the front-end and [Axum](https://github.com/tokio-rs/axum) for the backend. For styling [TailwindCSS](https://tailwindcss.com/) is used.


## Pre-requites (skip if you're all set)
1. First install [rust](https://www.rust-lang.org/tools/install) if you haven't yet 
2. For building & running the project use [cargo leptos](https://github.com/leptos-rs/cargo-leptos). It has built-in support for things like Tailwind, SASS, and testing.
3. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
4. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
5. Install [Node.js](https://nodejs.org/en/download) to get the tailwindcss properly working. Currently v20.12.0 is being used at the development side

## Running the project

1. Clone the repository
```bash
git clone https://github.com/iZafor/REnizer/
```
2. Change the current directory
 ```bash
 cd REnizer/renizer-web
```
3. Install js dependencies
```bash
npm install
```
4. Finally start the site
```bash
cargo leptos watch
```
