# OneOff

OneOff is a library for one-off types

```rust
use oneoff::OneOff;

let left = OneOff::Left(1);
let right = OneOff::Right(2);


assert_eq!(left, OneOff::Left(1));
assert_eq!(right, OneOff::Right(2));

assert!(left.is_left());
assert!(!left.is_right());
assert!(right.is_right());
assert!(!right.is_left());

assert_eq!(left.cmp(&right), std::cmp::Ordering::Less);
assert_eq!(right.cmp(&left), std::cmp::Ordering::Greater);

assert_eq!(left.left(), Some(1));
assert_eq!(right.right(), Some(2));
assert_eq!(left.right(), None);
assert_eq!(right.left(), None);

assert_eq!(left, left.clone());
assert_eq!(right, right.clone());
```
