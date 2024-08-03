# keyball-embassy-rp2040

ProMicro RP2040用のRust(+Embassy)製Keyballファームウェアです。
元のリポジトリは[こちら](https://github.com/nazo6/keyball-rs/)ですが、汎用性を高めるためにフレームワーク風にしてみたもがこちらの`rktk`リポジトリになります。
詳しくはリポジトリトップのREADMEを見てください。

紹介記事:
[RustとEmbassyでKeyballのファームウェアを作った](https://zenn.dev/nazo6/articles/keyball-embassy-rp2040)

## ステータス

基本的なキースキャン機能とレイヤ機能を実装しています。`keyboard/keymap.rs`を編集することでキーマップを変更できます。

## 既知の不具合

- 左右間の通信が不安定なことがある
- メディアキーが効かない

## ビルド

1. `elf2uf2-rs`をインストール

```
cargo install elf2uf2-rs
```

2. BOOTSELボタンを押しながらProMicro RP2040をUSBに接続
3. 実行
   ```
   cargo run --release
   ```

> [!TIP]
> このファームにはダブルリセットでBOOTSELに入る機能が内蔵されているため、以降はBOOTSELボタンを押しながら差す必要はありません。

> [!NOTE]
> 通常のProMicroのようにハードウェアでダブルリセットを検知しているわけではないので、ダブルタップが早すぎると検知されません。

## CREDITS

これらの先人がいなければここまで作れませんでした。

- もろもろ参考
  - [keyball-rs](https://github.com/hikalium/keyball-rs)
- ダブルリセットでBOOTSEL
  - https://github.com/Univa/rumcake/blob/2fa47dce9ab2b2406dd5465ccc9ce7b23e5ffdb0/rumcake/src/hw/mod.rs
- PMW3360ドライバ関連
  - https://github.com/kndndrj/mouse/tree/8c3cf4707cc392c16c91dc11e53f954f0fd820f1/firmware-rust/mouse-libs/src/pmw3360
- 分割キーボード間半二重通信
  - [QMK Firmware](https://github.com/qmk/qmk_firmware/blob/master/platforms/chibios/drivers/vendor/RP/RP2040/serial_vendor.c)
  - [rusty-dilemma](https://github.com/simmsb/rusty-dilemma/blob/5ffe8f5d2b6b0d534a4309edc737364cd96f44f1/firmware/src/interboard/onewire.rs)
