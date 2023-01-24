# Result

```
export LIBTORCH='/opt/homebrew/Cellar/pytorch/1.13.1'

export LD_LIBRARY_PATH=$LIBTORCH:$LD_LIBRARY_PATH


cargo run
   Compiling torch-sys v0.10.0
   Compiling tch v0.10.1
   Compiling tch-rs_helloworld v0.1.0 (/Users/globalyoung/Documents/test/test/rust/Machine_Learning_Rust/tch-rs_pytorch/tch-rs_helloworld)
    Finished dev [unoptimized + debuginfo] target(s) in 10.60s
     Running `target/debug/tch-rs_helloworld`
  6
  2
  8
  2
 10
[ CPUIntType{5} ]
```
