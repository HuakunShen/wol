# WOL

## Install for Golang

```bash
go get github.com/HuakunShen/wol/wol-go
```

## Usage
```go
import {
  wol "github.com/HuakunShen/wol/wol-go"
}
...
wol.WakeOnLan(mac, ip, port)
```

## CLI Demo
- [main.go](./main.go)
- executable: [wol](./wol)
