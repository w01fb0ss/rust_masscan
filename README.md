# A module for masscan

## Examples
Basic usage:

```rust
use rust_masscan::Masscan;

let other_args: Vec<String> = vec!["--banners".to_string()];

let mas = Masscan::default()
        .set_system_path("/usr/local/bin/masscan".to_string())
        .set_ports("22,8080-8100".to_string())
        .set_ranges("xx.xx.xx.xx,yy.yy.yy.yy".to_string())
        .set_rate("10000".to_string())
        .set_other_args(other_args);
let result = mas.run();
println!("{:?}", result);
```