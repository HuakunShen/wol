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
import (
  wol "github.com/HuakunShen/wol/wol-go"
)
...
err := wol.WakeOnLan(mac, ip, port) // port is a string, e.g. "9"
```

### CLI Demo
- [main.go](./main.go)
- executable: [wol](./wol)

## Python

- [wol.py](./wol-py/wol.py)

```bash
python3 wol-py/wol.py aa:bb:cc:dd:ee:ff -i 255.255.255.255 -p 9
python3 -m unittest discover wol-py
```

## Zig

- [wol-zig](./wol-zig)

```bash
cd wol-zig && zig build run -- aa:bb:cc:dd:ee:ff 255.255.255.255 9
zig build test
```

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

## MAC address formats

Every implementation accepts the same two forms: 12 bare hexadecimal
characters (`aabbccddeeff`), or six hexadecimal pairs separated by `.`, `:` or
`-` (`aa:bb:cc:dd:ee:ff`). Separators are only accepted in the positions where
they belong, and anything else is rejected rather than silently reinterpreted.
All of them emit a byte-identical 102-byte packet and exit non-zero on failure.
