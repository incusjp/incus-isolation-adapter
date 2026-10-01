# Security boundary

- No Incus certificate, token, address, instance name, or customer data enters
  the output contract.
- Input is a regular size-bounded JSON file.
- The package does not open sockets or mutate Incus state.
- Live collection requires a separate least-privilege read-only process.
