# Incus Isolation Adapter

Read-only adapter between sanitized Incus inventory and
`crowsi://network/boundary-input/v1`.

```bash
cargo run -- sample
cargo run -- transform examples/incus-inventory.sample.json
cargo test
```

The adapter does not call the Incus API or CLI. A deployment-specific collector
must obtain project, network, profile, and instance counts with read-only
credentials, remove addresses and certificates, then pass the bounded inventory
document to this adapter.

Incus Japan owns Incus-specific collection guidance and transformation. Crowsi
owns provider-neutral evaluation, alert vocabulary, and the Coela dashboard.
