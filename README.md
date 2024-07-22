# ambiq-apollo4p-pac
Peripheral access API for Ambiq Apollo 4 Plus microcontrollers

## Generated automatically using svd2rust

```
cargo install svd2rust form
svd2rust -i apollo4.svd --target=cortex-m
form -i lib.rs -o src/ && rm lib.rs
```
