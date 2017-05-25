## How to use this


### ruby
this is rust ffi for rust python node test.

find more at [http://rustbook.cn/content/Rust%20Inside%20Other%20Languages%20%E5%85%B6%E5%AE%83%E8%AF%AD%E8%A8%80%E4%B8%AD%E7%9A%84%20Rust.html](http://rustbook.cn/content/Rust%20Inside%20Other%20Languages%20%E5%85%B6%E5%AE%83%E8%AF%AD%E8%A8%80%E4%B8%AD%E7%9A%84%20Rust.html)

you need ruby ffi installed `gem install ffi`

```$shell
cd rust/ffi/ruby
cargo build --release
ruby src/ffi.rb

ruby src/noffi.rb
```

## python

- `Windows User` need python __amd64__ installed;
maybe this mirror [https://npm.taobao.org/mirrors/python](https://npm.taobao.org/mirrors/python) will help
- ```$shell
  cd rust/ffi/ruby
  cargo build --release
  python src/ffi.py
  ```

## node

- `$ npm install ffi -g`
- `$ npm link ffi`
- ```$shell
    cd rust/ffi/ruby
    cargo build --release
    node src/ffi.js
    ```