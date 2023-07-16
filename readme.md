# Bhtmp 
A reader for the bhtmp file format.

## Usage
```rust
use bhtmp::Bhtmp;

let map: Bhtmp = bhtmp::Bhtmp::new(
    include_bytes!("./HeightMap.bhtmp")
        .iter()
        .map(|v|*v)
        .collect()
);


```