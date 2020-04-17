##JNI Bindings for [rust-sentence-transformer](https://github.com/mladvladimir/rust-sentence-transformers) with Clojure working example

Run
```bash
compile
```

Add libtorch/lib directory to your `$LD_LIBRARY_PATH`. You can set it from same shell session after compilation step:
```bash
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$PWD/rs/target/release/build/torch-sys-0300a6a72f28e2c9/out/libtorch/libtorch/lib/
```

Run clojure repl from `clj/rust-sentence-transformers-clj`:
```bash
cd clj/rust-sentence-transformers-clj
lein repl
```
Example:
```clojure
(def embedder (get-sentence-transformer "/path/to/sentence-transformers-model/bert-base-nli-stsb-mean-tokens", "cpu"))
(encode embedder sentences 8)
```