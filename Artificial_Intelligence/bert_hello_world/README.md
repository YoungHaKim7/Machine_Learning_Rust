# Result :

```
$ cargo run

   Compiling bert_hello_world v0.1.0 (/Users/globalyoung/Documents/test/test/rust/Machine_Learning_Rust/Artificial_Intelligence/bert_hello_world)
warning: unused variable: `answers`
 --> src/main.rs:9:9
  |
9 |     let answers = qa_model
  |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_answers`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `bert_hello_world` (bin "bert_hello_world") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.90s
     Running `target/debug/bert_hello_world`
Downloading https://huggingface.co/distilbert-base-cased-distilled-squad/resolve/main/config.json [473B]... ✓ Done! Finished in 0 seconds
Downloading https://huggingface.co/bert-large-cased/resolve/main/vocab.txt [208.45KiB]... ✓ Done! Finished in 0 seconds
Downloading https://huggingface.co/distilbert-base-cased-distilled-squad/resolve/main/rust_model.ot [248.71MiB]..... ✓ Done! Finished in 12 seconds


[[Answer { score: 0.9950505495071411, start: 13, end: 22, answer: "Amsterdam" }]]

```

# Source 1. Question Answering

https://github.com/guillaume-be/rust-bert
