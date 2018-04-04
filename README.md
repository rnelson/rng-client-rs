# RNG Client

## Introduction

I like learning new things. I'm playing with both Rust and Kotlin at the same time, so after building [a service](https://github.com/rnelson/rng-service) in Kotlin, I decided to build a client in Rust.

## Running

First, build the code with `cargo build`.

Once that's done, run a copy of [the RNG service](https://github.com/rnelson/rng-service) and connect to it with the client:

```
target/debug/rngclient -s localhost -t int
```

```
target/debug/rngclient -s localhost -t float
```

```
target/debug/rngclient -s localhost -t double
```

## License

Released under the [MIT License](http://rnelson.mit-license.org).
