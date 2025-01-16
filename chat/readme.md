

``` 
# start server
cargo run --bin server

# start client
cargo run --bin client
```

todos
- [ ] handle_connection function in server  with `tokio::select!`
- [ ] main function in client with `tokio::select!`
- [ ] not broadcast message to client sending it