# WOL

## Golang

```bash
go get github.com/HuakunShen/wol/wol-go
```

### Installation As Executable

```bash
go install github.com/HuakunShen/wol@latest
# make sure your $GOPATH/bin is in PATH, for me, the executable is installed in ~/go/bin/wol
~/go/bin/wol --help
wol --help
```

### Usage
```go
import {
  wol "github.com/HuakunShen/wol/wol-go"
}
...
wol.WakeOnLan(mac, ip, port)
```

### CLI Demo
- [main.go](./main.go)
- executable: [wol](./wol)

## Python

- [wol.py](./wol-py/wol.py)

## JavaScript / TypeScript

- [wol-js](./wol-js) — Node, Bun and Deno implementations plus a CLI.

```bash
npm i wol-js          # Node / Bun
deno add jsr:@hk/wol  # Deno
```

See [wol-js/README.md](./wol-js/README.md) for usage; the Deno entrypoint needs
`--unstable-net`.

## Rust

- [wol-rust](./wol-rust)

```bash
cd wol-rust && cargo run -- aa:bb:cc:dd:ee:ff 255.255.255.255 9
```
