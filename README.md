# LISON: LIghtweight Shape Object Notation

LISONは軽量ベクター画像形式です。このレポジトリは

- LISONの仕様
- LISONファイルを検証するためのJSON Schema
- CairoによるLISONレンダラ

を含みます。

## `liblison`

```cpp
std::variant<lison::Image, lison::ParseFailure> lison::parse(std::string_view text);
void lison::render(const lison::Image &image, cairo_t *cr, double ppi, double scale);
```

## `lison2png`

```console
usage: lison2png [-o output] [-r resolution] [-s scale] input
options:
  -h        : print help message.
  -o <file> : output file name.
  -r <num>  : resolution in ppi.
  -s <num>  : magnification ratio.
```
