# LISON: LIghtweight Shape Object Notation

LISONは軽量ベクター画像形式です。このレポジトリは

- LISONの仕様
- LISONファイルを検証するためのJSON Schema
- CairoによるLISONレンダラ

を含みます。

## `lison`

```rust
use lison::image::Image;
use lison::render::render;

let lison_str: &str = /* [..] */;
let image: Image = serde_json::from_str(lison_str).unwrap();

let context: cairo::Context = /* [..] */;
let _ = render(&context, &image, 96.0, 1.0);
```

## `lison-to-png`

```console
usage: lison-to-png [-h] [-o output] [-r resolution] [-s scale] input
options:
  -h        : print help message.
  -o <file> : output file name.
  -r <num>  : resolution in ppi.
  -s <num>  : scale ratio.
```
