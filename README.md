# Usage
```bash
cargo run -- --stats-path=stat_sheet_real.json
```

Or with tracing

```bash
RUST_LOG='warn,cod_keeper=trace' OTEL_COLLECTOR_URL=grpc://localhost:4317 cargo run -- --cod-version=mw --stats-path=stat_sheet_test.json prompt
```
![Screenshot 2023-10-01 003754](https://github.com/pitoniak32/cod_keeper/assets/84917393/219b4ddf-82e9-4846-b115-a9114559f02c)
