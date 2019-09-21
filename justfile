wasm:
    wasm-pack build -t web -d ../frontend/wasm/ --no-typescript 

histogram:
    cargo run --example histogram_image --features "display-window"

blur:
    cargo run --example test_blur --features "display-window"

blur-release:
    cargo run --release --example test_blur --features "display-window"

edge:
    cargo run --example edge_detection --features "display-window"

edge-release:
    cargo run --release --example edge_detection --features "display-window"

bench-sobel:
    cargo bench --bench sobel -- --baseline sobel_naive

bench-blur:
    cargo bench --bench blur -- --baseline blur_naive