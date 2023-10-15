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

