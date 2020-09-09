# Rust Chat

## Dev

```bash
cd frontend
npm run dev     # run & watch frontend

cd ..
cargo run     # run backend
```


## Compile

```bash
cd frontend
# change ws url in src/tools.js#getWsUrl
npm run build     # compile frontend

cd ..
cargo build --release     # compile backend

serve frontend/build     # serve frontend
./target/release/rust_chat     # run backend
```
