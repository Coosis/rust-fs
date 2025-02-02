# rust-fs
Provides `fs` cli to check file sizes.
A toy project for trying out rust.

# Getting started
```bash
Cargo run -- -r <path>
```
Or
```bash
Cargo run -- -f <file>
```

# -h
```
$ fs -h
A simple file size lookup tool

Usage: fs [OPTIONS]

Options:
  -r, --root <ROOT>    The root directory to start the search from
  -f, --file <FILE>    The file to inspect size of, only used if root is not provided
  -d, --depth <DEPTH>  The maximum depth to dig to. Passing -1 means infinite, but beware larger values could result in long execution time [default: 5]
      --clean          If set to true, will only output filename and size, default is false
      --reverse        If set to true, will reverse the rows shown, default is false
  -h, --help           Print help
  -V, --version        Print version
```

# Unfinished Exercise
Multi-threadding to improve performance. Due to nature of dfs, i am 
not sure how would i implement this.
