
## komenda do generowania bindów

### Linux
```bash
bindgen metamod.h --with-derive-default -o src/metamod/abi/linux.rs -- -x c++ -target i686-unknown-linux-gnu
```

### Windows
```bash
bindgen metamod.h --with-derive-default -o src/metamod/abi/windows.rs -- -x c++ -target i686-pc-windows-msvc
```


## konfiguracja vs code

### Linux
```json
{
  "rust-analyzer.cargo.target": "i686-unknown-linux-gnu"
}
```

### Windows
```json
{
  "rust-analyzer.cargo.target": "i686-pc-windows-msvc"
}
```

## Komipacja
dla release trzeba podać jeszcze flagę --release

### Linux
```bash
cargo build-linux
```

### Windows
```bash
cargo build-win
```
