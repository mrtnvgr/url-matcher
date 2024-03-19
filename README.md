# url-matcher

## Usage

```rust
use url_matcher::matcher;
let result = matcher("http://github.com/:owner", "http://github.com/mrtnvgr").unwrap();
assert_eq!(result.get("owner"), Some("mrtnvgr").as_ref());
```
