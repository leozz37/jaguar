# Jaguar 🐆

![Build](https://github.com/leozz37/jaguar/workflows/Build/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Jaguar is a simple tool for testing socket connections. You can send, listen and ping a port.

## Running

You can use the --help option to check all the options:

| Argument       | Description                    | Required | Default   |
| -------------- | ------------------------------ | -------- | --------- |
| -l, --listen   | Listen to a socket port        | No       | False     |
| -s, --send     | Send payload to a socket port  | No       | False     |
| -a, --alive    | Ping a port                    | No       | True      |
| -p, --port     | Port to be interacted with     | Yes      | -         |
| -h, --hostname | Hostname to be interacted with | No       | 127.0.0.1 |
| -d, --data     | Payload to be sent             | No       | -         |


You can run it with `cargo` to install all its dependencies:

```shell
$ cargo run -- -p 3000
```

### Listen

To listen to a socket connection, run the following command:

```shell
$ cargo run -- -l -p 3000
```

To listen to a custom hostname, use the `-h` flag:

```shell
$ cargo run -- -l -p 3000 -h "127.0.0.1"
```

### Send

To send a payload to a socket connection, run the following command:

```shell
$ cargo run -- -s -p 3000 -d "Hello World"
```

To listen to a custom hostname, use the `-h` flag:

```shell
$ cargo run -- -s -p 3000 -d "Hello World" -h "127.0.0.1"
```

### Ping

To ping a socket connection, run the following command:

```shell
$ cargo run -- -p 3000
```

The `-a` flag is optional, ping is the default action of Jaguar.
