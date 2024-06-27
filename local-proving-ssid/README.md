# ZK Self-Sovereign Adult Check

This code demonstrates a minimal example of how to use the RISC Zero [zkVM] to make ZK proofs verifying a toy [Self-Sovereign Identity] is "an adult" by checking their age is greater than some preset constant.

## Quick Start

First, follow the [installation guide] to get your environment ready to run this application.

Then, run the example with:

```bash
cargo r -r
```

## Project Organization

zkVM applications are organized into a [host program] and a [guest program].
The host program can be found in [`src/main.rs`], and the guest program can be found in [`methods/guest/src/main.rs`].

The [host] first [executes] the guest program and then [proves the execution] to construct a [receipt].
The receipt can be passed to a third party, who can examine the [journal] to check the program's outputs and can [verify] the [receipt] to ensure the integrity of the [guest program]'s execution.

[`src/main.rs`]: src/main.rs
[`methods/guest/src/main.rs`]: methods/guest/src/main.rs
[host]: https://dev.risczero.com/terminology#host
[executes]: https://dev.risczero.com/terminology#execute
[guest program]: https://dev.risczero.com/terminology#guest-program
[host program]: https://dev.risczero.com/terminology#host-program
[proves the execution]: https://dev.risczero.com/terminology#prove
[receipt]: https://dev.risczero.com/terminology#receipt
[verify]: https://dev.risczero.com/terminology#verify
[journal]: https://dev.risczero.com/terminology#journal
[zkVM]: https://dev.risczero.com/zkvm
[installation guide]: https://dev.risczero.com/api/zkvm/install
[Self-Sovereign Identity]: https://en.wikipedia.org/wiki/Self-sovereign_identity