

``` 
# start server
cargo run --bin server

# start client
cargo run --bin client
```

todos
- [x] handle_connection function in server  with `tokio::select!`
- [x] main function in client with `tokio::select!`
- [ ] not broadcast message to client sending it

refs
- https://github.com/pretzelhammer/rust-blog/blob/master/posts/chat-server.md