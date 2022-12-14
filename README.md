# opengl_app
[Rustで始めるOpenGL](https://github.com/toyamaguchi/rust_opengl)様を写経して手を加えたものです。

Gifのように立方体をx,y,z方向に回転できるようにしています。

![rust-opengl-rotate-test](https://user-images.githubusercontent.com/20264602/207402872-6ad8526b-f04c-4385-a0d7-68e03e9c9dac.gif)


Rust,sdl2をインストールしてcargo run で動作します。(macでのみ動作確認しています)
```curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install sdl2
cargo run
```
