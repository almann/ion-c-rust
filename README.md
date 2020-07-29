# Ion C Bindings for Rust

This is an experimental bindings for Amazon Ion in Rust using [Ion C][ion-c]

## Development

This project uses a submodule to pull in [Ion C][ion-c].  The easiest way to pull everything in is to clone
the repository recursively, but you can also initialize the submodules as follows:

```bash
$ git submodule update --init --recursive
```

[ion-c]: https://github.com/amzn/ion-c