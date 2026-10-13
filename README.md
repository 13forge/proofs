# proofs

Papers and machine-checked proofs. **Sean Morin · 2748684 Alberta Ltd. · [deveraux.dev](https://deveraux.dev)**

Every paper here tags each claim with one of these statuses:

| Tag | Meaning |
|---|---|
| **PROVEN** | Proof given in the text, plus an exhaustive machine check in this repo |
| **BUILT** | Running code with receipts |
| **DESIGN_TARGET** | Intended, not yet built |
| **[VERIFY]** | Historical or external claim still awaiting a primary source |

## Index

| ID | Title | Status | Code |
|---|---|---|---|
| EMW-001 | [One Lattice, Three Addresses: state unranking in Sanskrit prosody and zero-allocation integer systems](emw-001-one-lattice/paper.md) | Draft v0.1 | [`prastara`](emw-001-one-lattice/prastara) |

## Receipts

```
cargo test --release --workspace
```

## Licensing

| What | Licence |
|---|---|
| Papers and prose (`**/*.md`) | [CC BY 4.0](LICENSE-CC-BY-4.0) |
| Code (`**/*.rs`, `Cargo.toml`) | [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE), at your option |

Unless you state otherwise, any contribution you submit for inclusion is dual-licensed as above, with no additional terms.

## Cite

Cite the paper by its ID and commit hash until a Zenodo DOI is minted.
