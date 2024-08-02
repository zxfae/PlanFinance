![Alt text](/img/planfinance.png "Home Page")

# PlanFinance
PlanFinance WebApp written in Rust.
Using Yew && tailwindCSS

## Use Rustup to install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Use apt (linux) to install Golang
```bash
sudo apt install golang
```

# Usage

## First to build this project
```bash
cd frontend/ npm run build:css
```
```bash
cd PlanFinance/ wasm-pack build frontend --target web
```

## Start server && db
```bash
cd PlanFinance/ cargo run --package backend
```
```bash
cd PlanFinance/database go run .
```

## Launch
```bash
http://localhost:8000 OR http://127.0.0.1:8000
```
## Contributing
Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## Author
```bash
ZxFae33
```
