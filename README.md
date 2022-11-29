
# dircmp

[<img alt="github" src="https://img.shields.io/badge/github-jondot/dircmp-8dagcb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/jondot/dircmp)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dircmp.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/dircmp)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-dircmp-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/dircmp)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/jondot/dircmp/Build/master?style=for-the-badge" height="20">](https://github.com/jondot/dircmp/actions?query=branch%3Amaster)

Compare two folders for the similarities and differences.

* Two way comparison
* File types (dir, symlink, etc.)
* Content difference is based on hashing the files, so binaries are also good for comparison
* Good for asserting in tests and for taking snapshot of difference or similarity


# Dependency

```toml
[dependencies]
dircmp = "0.1.0"
```


# Usage

Default usage:

```rust
let cmp = dircmp::Comparison::default();

let result = cmp
    .compare(
        Path::new("/tmp/a"),
        Path::new("/tmp/b"),
    )
```

Ignore components:

```rust
use regex::RegexSet;

let ignores = let set = RegexSet::new(&[
    r"foo",
    r"bar",
]).expect("should compile");

let cmp = dircmp::Comparison::new(ignores);

let result = cmp
    .compare(
        Path::new("/tmp/a"),
        Path::new("/tmp/b"),
    )
```

# Copyright

Copyright (c) 2022 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
