# ng-diff

A Hirschberg's LCS-based diffing implementation, working in quadratic time and in linear space.

For example:

```rust
use ng_diff::{AsCharsIter, Hirschberg, HirschbergAlg};

fn main() {
    let diff: Vec<_> = Hirschberg::diff("abcdfghjqvz".chars_iter(), "abcdefgijkrxyz".chars_iter());
    for item in diff {
        print!("{} ", item);
    }
    println!();
}
```

will print:

```
a b c d + e f g - h + i j - q - v + k + r + x + y z 
```

Other examples are in the [examples](examples) directory.