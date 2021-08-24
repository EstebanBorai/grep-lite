<div>
  <h1 align="center">grep-lite</h1>
  <h4 align="center">
    A tiny implementation of the famous utility "grep" in Rust for educational purposes
  </h4>
</div>

## Usage

```bash
# store the contents of input.txt into a variable
GREP_LITE_INPUT="$(cat input.txt)"

# run the grep-lite and use `GREP_LITE_INPUT` variable as input
cargo run --quiet -- "Abbey Road" $GREP_LITE_INPUT
```
