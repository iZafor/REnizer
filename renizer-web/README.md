<picture>
    <img src="public/images/logo/vector/default-monochrome-white.svg" alt="renizer-logo">
</picture>

This project uses [Leptos](https://github.com/leptos-rs/leptos) for the front-end and [Axum](https://github.com/tokio-rs/axum) for the backend. For styling [TailwindCSS](https://tailwindcss.com/) is used.


## Pre-requites
1. First install [rust](https://www.rust-lang.org/tools/install) if you haven't yet 
2. For building & running the project use [cargo leptos](https://github.com/leptos-rs/cargo-leptos). It has built-in support for things like Tailwind, SASS, and testing.
3. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
4. Install [Node.js](https://nodejs.org/en/download) to get the tailwindcss properly working. Currently v20.12.0 is being used at the development side
5. Create a database named `REnizer`
6. Optionally populate the database with [REnizer.sql](REnizer.sql)

## Running the project

1. Clone the repository
```bash
git clone https://github.com/iZafor/REnizer/
```
2. Change the current directory
 ```bash
 cd REnizer/renizer-web
```
3. Add the ability to compile Rust to WebAssembly
```bash
rustup target add wasm32-unknown-unknown
```
4. Install js dependencies
```bash
npm install
```
5. Make sure css file is updated
```bash
npx tailwindcss -i style/input.css -o style/output.css
```
6. Set required environment variables
```bash
REnizer_DB_HOST=db_host_ip_address
REnizer_DB_PORT=port_number
REnizer_DB_USER_NAME=username
REnizer_DB_USER_PASSWORD=password_for_the_user
LEPTOS_TAILWIND_VERSION=v3.4.3 (Optional)
```
6. Finally start the server
```bash
cargo leptos watch
```
