# Log nav rs

Tool for parsing logs. Currently only reads from stdin

## Testing

Read from file continuously
```sh
 tail -f log.in | cargo run --
```

Generate a stream of random characters
```sh
tr -dc A-Za-z0-9 </dev/urandom | head -c 13; echo
```