# udpiss

udp server and client test written in rust with tokio

## quickstart

```bash
cargo build --workspace

cargo run -p server
cargo run -p client
```

## udp server

```bash
Usage: server.exe --server-addr <SERVER_ADDR>

Options:
  -s, --server-addr <SERVER_ADDR>
  -h, --help                       Print help
```

## udp client

```bash
Usage: client.exe [OPTIONS] --server-addr <SERVER_ADDR>

Options:
  -c, --client-addr <CLIENT_ADDR>  [default: 0.0.0.0:0]
  -s, --server-addr <SERVER_ADDR>
  -h, --help                       Print help
```
