## wymagania do zbudowania

### Linux
dodanie bibliotek dla 32 bitowej architektury
```bash
sudo dpkg --add-architecture i386
sudo apt update
sudo apt install gcc-multilib g++-multilib
```

## komenda do generowania bindów

### Linux
```bash
bindgen metamod.h --with-derive-default -o src/metamod/adapter/abi/linux.rs -- -x c++ -target i686-unknown-linux-gnu
```

### Windows
```bash
bindgen metamod.h --with-derive-default -o src/metamod/adapter/abi/windows.rs -- -x c++ -target i686-pc-windows-msvc
```


## konfiguracja zed

### Linux
```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "cargo": {
          "target": "i686-unknown-linux-gnu",
        },
      },
    },
  },
}
```

### Windows
```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "cargo": {
          "target": "i686-pc-windows-msvc",
        },
      },
    },
  },
}
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
