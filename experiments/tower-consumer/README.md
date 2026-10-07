# Fresh consumer dependency check

This independent consumer declares current `tower-http` alongside web-search.
The default configuration preserves every server/provider feature. Run:

```sh
cargo tree --manifest-path experiments/tower-consumer/Cargo.toml -i tower-http@0.7.1
cargo tree --manifest-path experiments/tower-consumer/Cargo.toml -i tower-http@0.6.11
cargo run --manifest-path experiments/tower-consumer/Cargo.toml
```

The consumer and web-search share 0.7.1. Published Reqwest 0.13.5, Reqwest 0.12.28
(through web-capture/browser-commander), and web-capture 0.3.37 still require 0.6.
Cargo cannot unify incompatible minor versions before those upstream releases
update. A downstream `[patch]` would be consumer-specific and would not propagate
through the published web-search crate.

The core-only consumer has no older tower-http:

```sh
cargo tree --manifest-path experiments/tower-consumer/Cargo.toml --no-default-features -d
cargo run --manifest-path experiments/tower-consumer/Cargo.toml --no-default-features
```
