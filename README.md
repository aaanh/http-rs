# Very basic HTTP in Rust from "scratch"

Well, I used the provided-by-default net::{Tcp stuff} Rust package so it's not exactly from scratch, but it is lower-level enough (please verify with the OSI model), I guess.

Did it on a train from MTL -> Toronto to escape the boredom.

# Features

- Accept HTTP requests from client CLI or browser
- Serve client-side CSS and JS

# How to run

You need to install Rust and cargo, obviously.

```
cargo watch -x run
```

This utilizes the `rust-watch` package to automatically recompile the application code whenever changes occur in the source tree.

# To-dos

- [x] Asynchronous
- [ ] Server-side rendering (maybe)
- [ ] Template parsing
