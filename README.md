# Spellit

[![Build](https://github.com/kafji/spellit/workflows/Build/badge.svg)](https://github.com/kafji/spellit/actions?query=workflow%3ABuild)
[![Source](https://img.shields.io/badge/Source-666)](https://github.com/kafji/spellit)

CLI application to map characters to its [phonetic alphabets](https://en.wikipedia.org/wiki/NATO_phonetic_alphabet).

## Usage

```
$ spellit 'Hello 123!'
H -> Hotel
e -> Echo
l -> London
l -> London
o -> Oscar
  ->
1 -> One
2 -> Two
3 -> Three
! ->
```

## Installation

```bash
cargo install --git https://github.com/kafji/spellit
```

## Development

### Test

```
cargo test
```

### Benchmark

```
cargo bench
```

## License & Contribution

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
