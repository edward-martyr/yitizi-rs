# 異體字 yitizi-rs

Based on the Python/JS version of [nk2028/yitizi](https://github.com/nk2028/yitizi); check it to learn the design choices.
This is a Rust version of the `yitizi` tool, building the `json` file with the cargo build script.

Get all variants (_yitizi_, 異體字) of a Chinese character (Sinograph)!

## Installation

```sh
cargo add yitizi      # for the library
cargo install yitizi  # for the CLI
```

## Usage

### Library

```rust
println!("{:?}", yitizi::get('你'));
```

### CLI

```sh
異體字 yitizi-rs v0.1.0
查詢異體字！輸入 q 退出。(Query for variant Chinese characters! Input q to quit.)
>>> 你我他
'你': {'妳', '奶', '嬭', '伱'}, '我': {}, '他': {'她', '牠', '佗', '它'},
```
