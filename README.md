# Notify Server

When a request is received, desktop notification of the contents is sent.


## Usage

```text
Usage: notify-server.exe [PORT]

Arguments:
  [PORT]  The port to listen on [default: 12413]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Example

### Start Server (Windows)

```console
./notify-server.exe # default port: 12413
```

### Client sends notification

```console
curl -G http://localhost:12413/ --data-urlencode "summary=Hello" --data-urlencode "body=World"
```

### Server-side desktop notification

![notifycation sample](./image/notification-sample.png)
