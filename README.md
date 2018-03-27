Z-algorithm implement in Rust
=================

Z algorithm is a linear time string matching algorithm which runs in O(n) complexity. It is used to find all occurrence of a pattern in a string , which is common string searching problem.

### Installation:
Cargo.toml:
```
    z_algo = { git = "https://github.com/howeih/z-algorithm.git", branch = "master" }
```

### Usage :
```
    extern crate z_algo;

    use z_algo::Z;

    let z = Z::new("baabaa");
    let matches = z.search("baa");
    assert_eq!(matches, [0,3]);
```