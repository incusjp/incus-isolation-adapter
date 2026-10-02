# Using incus-isolation-adapter

Turn a sanitized Incus inventory into a document that an isolation-boundary evaluator can read.

## Before you start

A separately configured collector obtains the inventory. This adapter does not call the Incus API or CLI.

## First steps

Run from the repository root:

```sh
cargo test --locked
```

## How to assess the result

- Convert project, network, profile and instance counts.
- Reject unsupported inventory shapes before evaluation.

A passing source-level check establishes only what that check observes. Keep missing configuration, unavailable services and unverified deployment paths visible.

## Continue reading

[Repository overview](../README.md)
