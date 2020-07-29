# Ion C Bindings for Rust

***This repository is archived and development has moved into the main [Ion Rust][ion-rust] repository.***

This is an experimental set of bindings for for [Amazon Ion C][ion-c] in Rust.

## Development

This project uses a submodule to pull in [Ion C][ion-c].  The easiest way to pull everything in is to clone
the repository recursively, but you can also initialize the submodules as follows:

```bash
$ git submodule update --init --recursive
```

[ion-rust]: https://github.com/amzn/ion-rust
[ion-c]: https://github.com/amzn/ion-c
