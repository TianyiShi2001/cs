# cs

Computing the (Longest) Common Substring

## Example

```rust
use cs::longest_common_substring
let lcs = longest_common_substring(&[
    "ZYABCAGB",
    "BCAGDTZYY",
    "DACAGZZYSC",
    "CAGYZYSAU",
    "CAZYUCAGF",
]);
assert_eq!(lcs, "CAG");
```