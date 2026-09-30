# Contributing

Thanks for helping improve bom-kit. Pull requests and issues are welcome.

## Contributor license agreement

bom-kit is offered under the Functional Source License (`FSL-1.1-ALv2`, see `LICENSE`), and the
maintainer may also offer it under other terms (for example a commercial license). To make that
possible, by submitting a contribution (pull request, patch or otherwise) you agree that:

1. You wrote the contribution, or have the right to submit it, and it does not knowingly infringe
   anyone else's rights.
2. You grant Adrian Jastrzębski (the "Maintainer") a perpetual, worldwide, non-exclusive, royalty-free,
   irrevocable license to use, reproduce, modify, sublicense and distribute your contribution, and to
   relicense it under any terms, including the project's current license and any commercial license.
3. You also grant recipients of the project the patent license in Section 3 of the Apache License 2.0
   for your contribution.
4. You keep the copyright in your contribution.

Sign off each commit (`git commit -s`) to confirm this agreement.

## Development

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets
```
