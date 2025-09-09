# Url Shorten

Url shortener in Rust

# How to use it

Run

```bash
cargo run
```

And then open another terminal, and add urls to shorten

```bash
curl -X POST \
  http://127.0.0.1:3000/ \
  -H 'Content-Type: application/json' \
  -d '{
    "url": "https://example.com"
  }'
```

Should receive a response like

```
{"url":"http://127.0.0.1:3000/U2xV5a"}
```

And get the response with

```bash
curl -v http://127.0.0.1:3000/U2xV5a
```
