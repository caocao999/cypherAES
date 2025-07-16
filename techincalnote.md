

## ファイルからバイトで一括読み込み
```
use std::fs;

let data = fs::read("path/to/file.txt").expect("Failed to read file");
```