# Simple bar builder
## Summary
Simple bar builder takes values from the user to create a static bar to show progress for things such as a notification or statusbar module.

## Building
```sh
git clone git@github.com:klliio/bar-rs.git
cd bars-rs
cargo build -r
```
The binary will be in ```target/release/```

## Examples
```sh
████████████████████████████████▓▓▓▓▓▓▓▓ ./bar-rs -p 24 -m 30 -l 40
@@+++                                    ./bar-rs -p 23 -m 50 -l 5 --fill-char @ --empty-char +
#######---                               ./bar-rs -p 24 -m 32 -l 10 --fill-char \# --empty-char -
▓▓▓▓▓▓▓▓▓▓                               ./bar-rs -p 50 -m 3000 -l 10 
████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓                ./bar-rs -p 100 -m 300 -l 25
```
