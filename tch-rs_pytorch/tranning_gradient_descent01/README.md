# Result

```
$ cargo nextest run
   Compiling libc v0.2.139
   Compiling pkg-config v0.3.26
   Compiling typenum v1.16.0
   Compiling version_check v0.9.4
   Compiling cfg-if v1.0.0
   Compiling subtle v2.4.1
   Compiling base64ct v1.5.3
   Compiling autocfg v1.1.0
   Compiling crc32fast v1.3.2
   Compiling zstd-safe v5.0.2+zstd.1.5.2
   Compiling adler v1.0.2
   Compiling generic-array v0.14.6
   Compiling time-core v0.1.0
   Compiling miniz_oxide v0.6.2
   Compiling time-macros v0.2.6
   Compiling opaque-debug v0.3.0
   Compiling proc-macro2 v1.0.50
   Compiling itoa v1.0.5
   Compiling rand_core v0.6.4
   Compiling num-traits v0.2.15
   Compiling jobserver v0.1.25
   Compiling cpufeatures v0.2.5
   Compiling getrandom v0.2.8
   Compiling flate2 v1.0.25
   Compiling cc v1.0.78
   Compiling password-hash v0.4.2
   Compiling quote v1.0.23
   Compiling unicode-ident v1.0.6
   Compiling byteorder v1.4.3
   Compiling anyhow v1.0.68
   Compiling curl v0.4.44
   Compiling constant_time_eq v0.1.5
   Compiling time v0.3.17
   Compiling block-buffer v0.10.3
   Compiling crypto-common v0.1.6
   Compiling cipher v0.3.0
   Compiling digest v0.10.6
   Compiling socket2 v0.4.7
   Compiling aes v0.7.5
   Compiling syn v1.0.107
   Compiling hmac v0.12.1
   Compiling sha2 v0.10.6
   Compiling sha1 v0.10.5
   Compiling pbkdf2 v0.11.0
   Compiling zstd-sys v2.0.5+zstd.1.5.2
   Compiling libz-sys v1.1.8
   Compiling bzip2-sys v0.1.11+1.0.8
   Compiling curl-sys v0.4.59+curl-7.86.0
   Compiling num-integer v0.1.45
   Compiling thiserror v1.0.38
   Compiling rawpointer v0.2.1
   Compiling ppv-lite86 v0.2.17
   Compiling matrixmultiply v0.3.2
   Compiling num-complex v0.4.3
   Compiling rand_chacha v0.3.1
   Compiling lazy_static v1.4.0
   Compiling rand v0.8.5
   Compiling half v1.8.2
   Compiling ndarray v0.15.6
   Compiling bzip2 v0.4.4
   Compiling thiserror-impl v1.0.38
   Compiling zstd v0.11.2+zstd.1.5.2
   Compiling zip v0.6.3
   Compiling torch-sys v0.10.0
   Compiling tch v0.10.1
   Compiling tranning_gradient_descent01 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/Machine_Learning_Rust/tch-rs_pytorch/tranning_gradient_descent01)
warning: unused imports: `group_norm`, `layer_norm`
 --> src/lib.rs:1:15
  |
1 | use tch::nn::{group_norm, layer_norm};
  |               ^^^^^^^^^^  ^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `OptimizerConfig`
 --> src/lib.rs:2:23
  |
2 | use tch::nn::{Module, OptimizerConfig};
  |                       ^^^^^^^^^^^^^^^

warning: unused imports: `Kind`, `Reduction`
 --> src/lib.rs:3:29
  |
3 | use tch::{kind, nn, Device, Kind, Reduction, Tensor};
  |                             ^^^^  ^^^^^^^^^

warning: function `my_module` is never used
  --> src/lib.rs:62:4
   |
62 | fn my_module(p: nn::Path, dim: i64) -> impl nn::Module {
   |    ^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: function `round4` is never used
  --> src/lib.rs:98:4
   |
98 | fn round4(t: Tensor) -> Vec<f64> {
   |    ^^^^^^

warning: function `gru_test` is never used
   --> src/lib.rs:224:4
    |
224 | fn gru_test(rnn_config: nn::RNNConfig) {
    |    ^^^^^^^^

warning: function `lstm_test` is never used
   --> src/lib.rs:272:4
    |
272 | fn lstm_test(rnn_config: nn::RNNConfig) {
    |    ^^^^^^^^^

warning: function `embedding_test` is never used
   --> src/lib.rs:321:4
    |
321 | fn embedding_test(embedding_config: nn::EmbeddingConfig) {
    |    ^^^^^^^^^^^^^^

warning: function `linear_test` is never used
   --> src/lib.rs:367:4
    |
367 | fn linear_test(linear_config: nn::LinearConfig) {
    |    ^^^^^^^^^^^

warning: function `apply_conv` is never used
   --> src/lib.rs:428:4
    |
428 | fn apply_conv(xs: &Tensor, padding_mode: nn::PaddingMode) -> Tensor {
    |    ^^^^^^^^^^

warning: `tranning_gradient_descent01` (lib) generated 10 warnings
    Finished test [unoptimized + debuginfo] target(s) in 15.35s
    Starting 17 tests across 1 binaries
     Running [ 00:00:00] [                         ]  0/17: 1 run     Running [ 00:00:00] [                         ]  0/17: 1 run     Running [ 00:00:00] [                         ]  0/17: 1 run     Running [ 00:00:00] [                         ]  0/17: 1 run     Running [ 00:00:00] [                         ]  0/17: 1 run     Running [ 00:00:00] [                         ]  0/17: 2 run     Running [ 00:00:00] [                         ]  0/17: 2 run     Running [ 00:00:00] [                         ]  0/17: 2 run     Running [ 00:00:00] [                         ]  0/17: 2 run     Running [ 00:00:00] [                         ]  0/17: 2 run     Running [ 00:00:00] [                         ]  0/17: 3 run     Running [ 00:00:00] [                         ]  0/17: 3 run     Running [ 00:00:00] [                         ]  0/17: 3 run     Running [ 00:00:00] [                         ]  0/17: 3 run     Running [ 00:00:00] [                         ]  0/17: 3 run     Running [ 00:00:00] [                         ]  0/17: 4 run     Running [ 00:00:00] [                         ]  0/17: 4 run     Running [ 00:00:00] [                         ]  0/17: 4 run     Running [ 00:00:00] [                         ]  0/17: 4 run     Running [ 00:00:00] [                         ]  0/17: 4 run     Running [ 00:00:00] [                         ]  0/17: 5 run     Running [ 00:00:00] [                         ]  0/17: 5 run     Running [ 00:00:00] [                         ]  0/17: 5 run     Running [ 00:00:00] [                         ]  0/17: 5 run     Running [ 00:00:00] [                         ]  0/17: 5 run     Running [ 00:00:00] [                         ]  0/17: 6 run     Running [ 00:00:00] [                         ]  0/17: 6 run     Running [ 00:00:00] [                         ]  0/17: 6 run     Running [ 00:00:00] [                         ]  0/17: 6 run     Running [ 00:00:00] [                         ]  0/17: 6 run     Running [ 00:00:00] [                         ]  0/17: 7 run     Running [ 00:00:00] [                         ]  0/17: 7 run     Running [ 00:00:00] [                         ]  0/17: 7 run     Running [ 00:00:00] [                         ]  0/17: 7 run     Running [ 00:00:00] [                         ]  0/17: 7 run     Running [ 00:00:00] [                         ]  0/17: 8 run     Running [ 00:00:00] [                         ]  0/17: 8 run     Running [ 00:00:00] [                         ]  0/17: 8 run     Running [ 00:00:00] [                         ]  0/17: 8 run        PASS [   0.185s] tranning_gradient_descent01 bn_test
     Running [ 00:00:00] [                         ]  0/17: 8 run     Running [ 00:00:00] [                         ]  0/17: 8 run     Running [ 00:00:00] [                         ]  0/17: 7 run     Running [ 00:00:00] [                         ]  0/17: 7 run     Running [ 00:00:00] [=>                       ]  1/17: 7 run     Running [ 00:00:00] [=>                       ]  1/17: 7 run     Running [ 00:00:00] [=>                       ]  1/17: 7 run     Running [ 00:00:00] [=>                       ]  1/17: 8 run     Running [ 00:00:00] [=>                       ]  1/17: 8 run     Running [ 00:00:00] [=>                       ]  1/17: 8 run        PASS [   0.195s] tranning_gradient_descent01 embedding_default
     Running [ 00:00:00] [=>                       ]  1/17: 8 run     Running [ 00:00:00] [=>                       ]  1/17: 8 run     Running [ 00:00:00] [=>                       ]  1/17: 7 run     Running [ 00:00:00] [=>                       ]  1/17: 7 run     Running [ 00:00:00] [==>                      ]  2/17: 7 run     Running [ 00:00:00] [==>                      ]  2/17: 7 run     Running [ 00:00:00] [==>                      ]  2/17: 7 run     Running [ 00:00:00] [==>                      ]  2/17: 8 run     Running [ 00:00:00] [==>                      ]  2/17: 8 run     Running [ 00:00:00] [==>                      ]  2/17: 8 run        PASS [   0.204s] tranning_gradient_descent01 conv
     Running [ 00:00:00] [==>                      ]  2/17: 8 run     Running [ 00:00:00] [==>                      ]  2/17: 8 run     Running [ 00:00:00] [==>                      ]  2/17: 7 run     Running [ 00:00:00] [==>                      ]  2/17: 7 run     Running [ 00:00:00] [====>                    ]  3/17: 7 run     Running [ 00:00:00] [====>                    ]  3/17: 7 run     Running [ 00:00:00] [====>                    ]  3/17: 7 run     Running [ 00:00:00] [====>                    ]  3/17: 8 run     Running [ 00:00:00] [====>                    ]  3/17: 8 run     Running [ 00:00:00] [====>                    ]  3/17: 8 run        PASS [   0.204s] tranning_gradient_descent01 bn_test_no_affine
     Running [ 00:00:00] [====>                    ]  3/17: 8 run     Running [ 00:00:00] [====>                    ]  3/17: 8 run     Running [ 00:00:00] [====>                    ]  3/17: 7 run     Running [ 00:00:00] [====>                    ]  3/17: 7 run     Running [ 00:00:00] [=====>                   ]  4/17: 7 run     Running [ 00:00:00] [=====>                   ]  4/17: 7 run     Running [ 00:00:00] [=====>                   ]  4/17: 7 run     Running [ 00:00:00] [=====>                   ]  4/17: 8 run     Running [ 00:00:00] [=====>                   ]  4/17: 8 run     Running [ 00:00:00] [=====>                   ]  4/17: 8 run        PASS [   0.204s] tranning_gradient_descent01 embedding_neg_padding
     Running [ 00:00:00] [=====>                   ]  4/17: 8 run     Running [ 00:00:00] [=====>                   ]  4/17: 8 run     Running [ 00:00:00] [=====>                   ]  4/17: 7 run     Running [ 00:00:00] [=====>                   ]  4/17: 7 run     Running [ 00:00:00] [=======>                 ]  5/17: 7 run     Running [ 00:00:00] [=======>                 ]  5/17: 7 run     Running [ 00:00:00] [=======>                 ]  5/17: 7 run     Running [ 00:00:00] [=======>                 ]  5/17: 8 run     Running [ 00:00:00] [=======>                 ]  5/17: 8 run     Running [ 00:00:00] [=======>                 ]  5/17: 8 run     Running [ 00:00:00] [=======>                 ]  5/17: 8 run        PASS [   0.210s] tranning_gradient_descent01 embedding_zero_padding
     Running [ 00:00:00] [=======>                 ]  5/17: 8 run     Running [ 00:00:00] [=======>                 ]  5/17: 8 run     Running [ 00:00:00] [=======>                 ]  5/17: 7 run     Running [ 00:00:00] [=======>                 ]  5/17: 7 run     Running [ 00:00:00] [========>                ]  6/17: 7 run     Running [ 00:00:00] [========>                ]  6/17: 7 run     Running [ 00:00:00] [========>                ]  6/17: 7 run     Running [ 00:00:00] [========>                ]  6/17: 8 run     Running [ 00:00:00] [========>                ]  6/17: 8 run     Running [ 00:00:00] [========>                ]  6/17: 8 run     Running [ 00:00:00] [========>                ]  6/17: 8 run     Running [ 00:00:00] [========>                ]  6/17: 8 run     Running [ 00:00:00] [========>                ]  6/17: 9 run     Running [ 00:00:00] [========>                ]  6/17: 9 run     Running [ 00:00:00] [========>                ]  6/17: 9 run        PASS [   0.236s] tranning_gradient_descent01 gradient_descent_test
     Running [ 00:00:00] [========>                ]  6/17: 9 run     Running [ 00:00:00] [========>                ]  6/17: 9 run     Running [ 00:00:00] [========>                ]  6/17: 8 run     Running [ 00:00:00] [========>                ]  6/17: 8 run     Running [ 00:00:00] [==========>              ]  7/17: 8 run     Running [ 00:00:00] [==========>              ]  7/17: 8 run     Running [ 00:00:00] [==========>              ]  7/17: 8 run     Running [ 00:00:00] [==========>              ]  7/17: 9 run     Running [ 00:00:00] [==========>              ]  7/17: 9 run     Running [ 00:00:00] [==========>              ]  7/17: 9 run     Running [ 00:00:00] [==========>              ]  7/17: 9 run        PASS [   0.188s] tranning_gradient_descent01 layer_norm_test
     Running [ 00:00:00] [==========>              ]  7/17: 9 run     Running [ 00:00:00] [==========>              ]  7/17: 9 run     Running [ 00:00:00] [==========>              ]  7/17: 8 run     Running [ 00:00:00] [==========>              ]  7/17: 8 run     Running [ 00:00:00] [===========>             ]  8/17: 8 run     Running [ 00:00:00] [===========>             ]  8/17: 8 run     Running [ 00:00:00] [===========>             ]  8/17: 8 run     Running [ 00:00:00] [===========>             ]  8/17: 9 run     Running [ 00:00:00] [===========>             ]  8/17: 9 run     Running [ 00:00:00] [===========>             ]  8/17: 9 run        PASS [   0.199s] tranning_gradient_descent01 gru
     Running [ 00:00:00] [===========>             ]  8/17: 9 run     Running [ 00:00:00] [===========>             ]  8/17: 9 run     Running [ 00:00:00] [===========>             ]  8/17: 8 run     Running [ 00:00:00] [===========>             ]  8/17: 8 run     Running [ 00:00:00] [=============>           ]  9/17: 8 run        PASS [   0.224s] tranning_gradient_descent01 gradient_descent_test_clip_norm
     Running [ 00:00:00] [=============>           ]  9/17: 8 run     Running [ 00:00:00] [=============>           ]  9/17: 8 run     Running [ 00:00:00] [=============>           ]  9/17: 7 run     Running [ 00:00:00] [=============>           ]  9/17: 7 run     Running [ 00:00:00] [==============>          ] 10/17: 7 run        PASS [   0.216s] tranning_gradient_descent01 group_norm_test
     Running [ 00:00:00] [==============>          ] 10/17: 7 run     Running [ 00:00:00] [==============>          ] 10/17: 7 run     Running [ 00:00:00] [==============>          ] 10/17: 6 run     Running [ 00:00:00] [==============>          ] 10/17: 6 run     Running [ 00:00:00] [================>        ] 11/17: 6 run        PASS [   0.201s] tranning_gradient_descent01 linear
     Running [ 00:00:00] [================>        ] 11/17: 6 run     Running [ 00:00:00] [================>        ] 11/17: 6 run     Running [ 00:00:00] [================>        ] 11/17: 5 run     Running [ 00:00:00] [================>        ] 11/17: 5 run     Running [ 00:00:00] [=================>       ] 12/17: 5 run     Running [ 00:00:00] [=================>       ] 12/17: 5 run        PASS [   0.199s] tranning_gradient_descent01 lstm
     Running [ 00:00:00] [=================>       ] 12/17: 5 run     Running [ 00:00:00] [=================>       ] 12/17: 5 run     Running [ 00:00:00] [=================>       ] 12/17: 4 run     Running [ 00:00:00] [=================>       ] 12/17: 4 run     Running [ 00:00:00] [===================>     ] 13/17: 4 run        PASS [   0.435s] tranning_gradient_descent01 gradient_clip_test
     Running [ 00:00:00] [===================>     ] 13/17: 4 run     Running [ 00:00:00] [===================>     ] 13/17: 4 run     Running [ 00:00:00] [===================>     ] 13/17: 3 run     Running [ 00:00:00] [===================>     ] 13/17: 3 run     Running [ 00:00:00] [====================>    ] 14/17: 3 run        PASS [   0.232s] tranning_gradient_descent01 layer_norm_parameters_test
     Running [ 00:00:00] [====================>    ] 14/17: 3 run     Running [ 00:00:00] [====================>    ] 14/17: 3 run     Running [ 00:00:00] [====================>    ] 14/17: 2 run     Running [ 00:00:00] [====================>    ] 14/17: 2 run     Running [ 00:00:00] [======================>  ] 15/17: 2 run        PASS [   0.208s] tranning_gradient_descent01 optimizer_test
     Running [ 00:00:00] [======================>  ] 15/17: 2 run     Running [ 00:00:00] [======================>  ] 15/17: 2 run     Running [ 00:00:00] [======================>  ] 15/17: 1 run     Running [ 00:00:00] [======================>  ] 15/17: 1 run     Running [ 00:00:00] [=======================> ] 16/17: 1 run     Running [ 00:00:00] [=======================> ] 16/17: 1 run        PASS [   0.152s] tranning_gradient_descent01 pad
     Running [ 00:00:00] [=======================> ] 16/17: 1 run     Running [ 00:00:00] [=======================> ] 16/17: 1 run     Running [ 00:00:00] [=======================> ] 16/17: 0 run     Running [ 00:00:00] [=======================> ] 16/17: 0 run     Running [ 00:00:00] [=========================] 17/17: 0 run------------
     Summary [   0.543s] 17 tests run: 17 passed, 0 skipped

```

https://github.com/LaurentMazare/tch-rs/blob/main/tests/nn_tests.rs

https://github.com/LaurentMazare/tch-rs
