# rust-echo-server

### サーバの起動

TCPの場合

```bash
$ cargo run
```

UDPの場合

```bash
$ cargo run udp
```

### テスト用クライアントの起動

```bash
node bin/test_tcp 8080 'Hello World'
```

```bash
node bin/test_udp
```
