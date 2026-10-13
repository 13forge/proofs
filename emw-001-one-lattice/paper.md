# One Lattice, Three Addresses
### State unranking in Sanskrit prosody and zero-allocation integer systems

**Sean (Deveraux)** · 2748684 Alberta Ltd · deveraux.dev
**Series:** East Meets West, EMW-001 · **Status:** DRAFT v0.1 (2026-10-13)
**Licence:** text CC BY 4.0 · code MIT OR Apache-2.0

---

## Status legend

| Tag | Meaning |
|---|---|
| **PROVEN** | Mathematical statement with proof given here, plus exhaustive machine check (Appendix A) |
| **BUILT** | Running code exists in the 13Forge workspace with receipts |
| **DESIGN_TARGET** | Intended integration, not yet built |
| **[VERIFY]** | Historical claim that needs a primary or peer-reviewed source before release |

---

## Abstract

Sanskrit prosody needed to catalogue every possible metre. Over about fifteen centuries, it produced three exact, table-free ways to go between an integer index and a metre: positional (binary), weight-constrained (binomial), and duration-constrained (Fibonacci). Modern enumerative combinatorics calls these *unranking* algorithms. This paper states the three addressing schemes precisely, proves them, and machine-checks them exhaustively. It also corrects common inflations of the historical record and maps each scheme onto zero-allocation state addressing in a deterministic integer runtime. We do not claim descent from these texts to modern computing. Our claim is narrower: anyone who must enumerate a discrete space under a hard constraint ends up on the same lattice, and the prosodists reached it from a linguistic constraint.

---

## 1. Scope and non-claims

1. **No lineage.** Modern computing descends from Leibniz → Boole → Shannon. Indian prosody is independent discovery, not ancestry.
2. **No hardware.** These are methods for enumerating and counting. There is no logic-gate design and no Boolean algebra.
3. **No "father of computing."** That framing is rejected.
4. **No priority on the bridge.** We know of no prior work that frames sūci-prastāra as a scan primitive. Absence of evidence is not proof of absence, so we make no "first" claim.
5. **What is claimed:** the same three addressing structures appear in a premodern linguistic tradition, in exact algorithmic form, and they transfer without modification to a zero-allocation integer system.

---

## 2. Chronology (corrected)

| Date | Source | Contribution | Status |
|---|---|---|---|
| c. 3rd–2nd c. BCE | Piṅgala, *Chandaḥśāstra* | Terse sūtras for prastāra (enumeration), naṣṭa, uddiṣṭa, saṅkhyā | [VERIFY sūtra numbers] |
| c. 600–800 CE | Virahāṅka, *Vṛttajātisamuccaya* | Mātrā count: each term = sum of the previous two | Singh 1985 |
| c. 7th c. CE | Virahāṅka, *Vṛttajātisamuccaya* | **Sūci-prastāra** ("needle spread"): binomial columns by repeated partial sums, dropping the last term each pass. Shah identifies it with Piṅgala's *lagakriyā*. | Shah 2008 [VERIFY against the Velankar edition] |
| before 1135 | Gopāla (commentary on Virahāṅka) | Worked values 3, 5, 8, 13, 21 | Singh 1985 |
| c. 1150 | Hemacandra | Same recurrence, stated independently | Singh 1985 |
| 10th c. | Halāyudha, *Mṛtasañjīvanī* (commentary on Piṅgala) | Explicit procedures, including **meru-prastāra** (the triangle). Reading it back into Piṅgala's sūtra is **disputed**: Weber and Shah reject it, Alsdorf defends it. | Shah 2008 |
| 1202 | Fibonacci, *Liber Abaci* | Same recurrence, in Europe | — |
| early 14th c. | *Prākṛta Paiṅgala* | **Mātrā-meru**: mātrā metres tabulated by guru count; entries are binomial | Singh 1985 |
| 1356 | Nārāyaṇa Paṇḍita, *Gaṇita Kaumudī* | **Sūcī-paṅkti** ("needle sequence"): a separate construction, built from products, yielding multinomial generalisations. Not the same as Virahāṅka's sūci-prastāra. | Singh 1985 |
| 1011–1077 | Shao Yong | Binary-ordered hexagram arrangement | [VERIFY] |
| 1679 | Leibniz, *De progressione dyadica* | Binary arithmetic | — |
| 1701 | Bouvet → Leibniz | Hexagram diagram, which Leibniz took as **confirmation**, not origin | — |
| 1703 | Leibniz, *Explication de l'Arithmétique Binaire* | Publication | — |
| 1967 | Ingerman, *CACM* 10(3) | Proposes "Pāṇini–Backus Form" | — |

**Corrections logged against circulating accounts:**
- *"Leibniz got binary from the I Ching."* False. He had binary in 1679 and received the hexagrams in 1701.
- *"Piṅgala wrote Pascal's triangle."* Disputed. The triangle is in Halāyudha's commentary, about a millennium later, and Shah argues the sūtra doesn't support it.
- *Sūci-prastāra.* Shah (2008) attributes it to Virahāṅka, c. 7th c., and describes it as repeated partial sums. That makes it the **earliest firmly attested binomial construction in the Indian record**, earlier than the meru-prastāra. Nārāyaṇa's sūcī-paṅkti (1356) is a different object; don't conflate the two.
- *"Oldest prefix-sum accumulator in history."* Unsupported superlative. Removed.
- *"Prosodists flattened the triangle to cut working-memory load."* A plausible motive, but no text says it. Removed.

---

## 3. Encoding

A metre of *n* syllables is a word in {L, G}ⁿ. Laghu (L, short) → 0, guru (G, long) → 1. Syllable *i* (0-indexed from the start of the line) is bit *i*, so **the first syllable is the least significant bit**. In mātrā (duration) metres, L weighs 1 and G weighs 2.

---

## 4. Addressing I: positional (prastāra, naṣṭa, uddiṣṭa)

**Prastāra.** The canonical enumeration of {L, G}ⁿ starts at all-G. Each next row flips the first G to L and resets every earlier syllable to G. That is a binary counter running backwards. Indices are **1-based**.

**Naṣṭa (index → metre).** Given row *k*: if *k* is odd, the syllable is L and *k* ← (*k*+1)/2. If *k* is even, the syllable is G and *k* ← *k*/2. Repeat *n* times.

**Uddiṣṭa (metre → index).** Write 1, 2, 4, … over the syllables. Add the values over the gurus, then add 1.

> **Theorem 1 (PROVEN).** Under the encoding of §3, naṣṭa(*k*) = *k* − 1, and uddiṣṭa is its inverse. The prastāra is the bijection {1…2ⁿ} ↔ {0,1}ⁿ given by little-endian binary of *k* − 1.
>
> *Proof.* Write *k* − 1 = Σ bᵢ2ⁱ. The parity of *k* is the complement of b₀: *k* odd ⇔ b₀ = 0 ⇔ L. Both update rules map *k* to 1 + ⌊(*k*−1)/2⌋, which shifts *k* − 1 right by one bit. By induction on *n*, the *i*-th emitted syllable is bᵢ. Uddiṣṭa computes 1 + Σ bᵢ2ⁱ = *k* directly. ∎

**Saṅkhyā (count).** |{L, G}ⁿ| = 2ⁿ, computed by Piṅgala's rule: for even *n*, halve and square; for odd *n*, subtract one and double. This is exponentiation by squaring, O(log *n*), the same method modular exponentiation in RSA uses.

**Cost:** O(*n*) bit operations, no table, no allocation.

---

## 5. Addressing II: weight-constrained (meru-prastāra)

The number of *n*-syllable metres with exactly *r* gurus is C(*n*, *r*). The meru-prastāra tabulates these values.

> **Theorem 2 (PROVEN).** The weight-*r* rows of the prastāra are addressable by the **combinatorial number system**: every 0 ≤ *k* < C(*n*,*r*) has a unique representation *k* = Σⱼ₌₁ʳ C(cⱼ, j) with *n* > c_r > … > c₁ ≥ 0, and the guru positions are {cⱼ}. Unranking is greedy: for *j* = *r* down to 1, take the largest *c* with C(*c*, *j*) ≤ *k*, set guru *c*, and subtract.
>
> *Proof.* Standard (Lehmer; Knuth TAOCP 7.2.1.3). Exhaustively checked for *n* ≤ 20 in Appendix A. ∎

**Cost:** O(*n*) binomial evaluations. Binomials come from the multiplicative formula, so no table is needed.

### 5.1 Sūci-prastāra: the binomials as an iterated scan

Virahāṅka builds the binomial columns by partial sums. Column 1 is 1, 2, …, *n*. Each later column is the running sum of the one before, with the last entry dropped (Shah 2008).

> **Theorem 5 (PROVEN). Hockey-stick = one sūci pass.** Σᵢ₌ᵣⁿ C(*i*, *r*) = C(*n*+1, *r*+1). Equivalently, applying an inclusive scan *k* times to the all-ones vector gives C(*i*+*k*, *k*) at index *i*.
>
> *Proof.* Pascal's rule C(*i*+1, *r*+1) = C(*i*, *r*+1) + C(*i*, *r*) telescopes. Induct on *k* for the scan form. Checked for *r* ≤ 20, *n* ≤ 60 (identity form) and *k* ≤ 20, *i* < 48 (scan form). ∎

**Systems reading. The claims below are bounded on purpose.**

| Property | Sūci (iterated scan) | Meru (row recurrence) | Multiplicative C(*n*,*k*) |
|---|---|---|---|
| Ops for a column of length *n*, order *k* | *nk* adds | *nk* adds (in-place, right-to-left row update) | O(*k*) multiplies **and divides** per entry |
| Working state | one accumulator + caller buffer | caller buffer | O(1) |
| Arithmetic | **add-only** | add-only | needs division |
| Maps to | standard scan primitive (Blelloch 1990): SIMD, GPU, FPGA accumulator | 2-tap stencil | scalar ALU |

1. Sūci and meru do the **same work** with the **same asymptotics**. A properly written row update is also a sequential single-buffer pass. The claim that meru is "cache-thrashing" doesn't hold up and is not made here.
2. Sūci's real systems property is that **each pass is exactly a prefix scan**, the most heavily optimised parallel primitive there is. Binomial tables therefore inherit O(log *n*)-depth parallel scans and existing hardware scan units with no new kernels.
3. Sūci is **add-only and division-free**. In an integer-deterministic runtime, that removes the only division in the binomial path. This is the property 13Forge actually wants.
4. Bench, as a DESIGN_TARGET: iterated SIMD scan vs in-place row update vs multiplicative formula, for *n* ≤ 4096 and *k* ≤ 64. The hypothesis is parity between the first two. The point of the bench is the division-free result, not raw speed.

---

## 6. Addressing III: duration-constrained (mātrā)

Let M(*n*) be the number of L/G sequences of total duration *n*, with L = 1 and G = 2. The first syllable is either L, leaving *n*−1, or G, leaving *n*−2. So M(*n*) = M(*n*−1) + M(*n*−2), with M(0) = M(1) = 1, and therefore **M(*n*) = F(*n*+1)**. This is Virahāṅka's rule.

> **Theorem 3 (PROVEN). Mātrā unranking.** Order sequences lexicographically with L < G. For a 0-indexed *k* and remaining duration *n*: if *k* < M(*n*−1), emit L and set *n* ← *n*−1. Otherwise emit G, set *k* ← *k* − M(*n*−1), and *n* ← *n*−2. This is a bijection {0 … F(*n*+1)−1} ↔ sequences of duration *n*, and the corresponding ranking is its inverse.
>
> *Proof.* Exactly M(*n*−1) sequences begin with L, and every one of them precedes every sequence beginning with G. Induct on *n*. Checked against brute-force lexicographic enumeration for *n* ≤ 20. ∎

> **Theorem 4 (PROVEN). Bridge identity (shallow diagonals).** A duration-*n* sequence with *g* gurus has *n* − 2*g* laghus, *n* − *g* syllables in total, and C(*n*−*g*, *g*) arrangements. Hence
> **F(*n*+1) = Σ_g C(*n*−*g*, *g*)**.
> The Fibonacci count is the binomial count refined by guru number, which is the shallow-diagonal sum of the binomial triangle. This is the content of the *Prākṛta Paiṅgala* mātrā-meru (Singh 1985).
>
> Checked for *n* ≤ 80, and per-guru counts checked for *n* ≤ 20.

**Fibonacci in O(log n).** Fast doubling: F(2*m*) = F(*m*)(2F(*m*+1) − F(*m*)) and F(2*m*+1) = F(*m*)² + F(*m*+1)². This is the saṅkhyā squaring pattern applied to a different recurrence. The same procedure computes both counts.

**Cost:** O(*n*) steps, one O(log *n*) Fibonacci evaluation per step (or a 94-entry const table, since F(93) is the largest Fibonacci number that fits in u64). No allocation.

---

## 7. The three addressings compared

| Constraint | Count | Unrank method | Prosodic source |
|---|---|---|---|
| fixed length *n* | 2ⁿ | binary digits of *k*−1 | Piṅgala: naṣṭa / uddiṣṭa |
| fixed length *n*, *r* gurus | C(*n*,*r*) | greedy on binomials | sūci-prastāra (Virahāṅka); meru-prastāra (Halāyudha) |
| fixed duration *n* | F(*n*+1) | greedy on Fibonacci | Virahāṅka; mātrā-meru (*Prākṛta Paiṅgala*) |

Each row is a numeration system whose place values are the counts of the constrained space (powers of two, binomials, Fibonacci numbers). Unranking means writing *k* in that system. This is the single lattice the title refers to.

---

## 8. Engineering mapping (13Forge)

| Use | Addressing | Why it fits | Status |
|---|---|---|---|
| Deterministic state #*k* with no lookup table | I | Index → state in O(*n*) integer ops; nothing allocated, nothing to cache-miss | DESIGN_TARGET |
| Exactly *r* active of *n* (voices, channels, capability bits) | II | Dense index over valid states only; invalid states can't be addressed at all | DESIGN_TARGET |
| Rhythm space under a fixed bar length | III | Enumerates every subdivision of *n* ticks into 1- and 2-tick units, densely and in order | DESIGN_TARGET |
| Division-free binomial tables | II (sūci) | Iterated scans; add-only integer path; maps onto SIMD scan | DESIGN_TARGET |
| Ingress validation | I–III | The ranking function doubles as a validator: a state is well-formed iff ranking it round-trips | DESIGN_TARGET |

The reference implementation (Appendix A) is `no_std`, integer-only, and allocation-free. Promote these rows to **BUILT** once it is lifted into the workspace with benches.

---

## 9. Thesis

Piṅgala, Shao Yong, Leibniz and Shannon arrived at binary enumeration independently. Virahāṅka, Gopāla, Hemacandra and Fibonacci arrived at the mātrā recurrence independently. The binomial triangle appears in India, Persia, China and Europe. These recurrences are not cultural borrowings. They are what you get when you have to count a discrete space under a hard constraint, and they are found again because they are forced by that problem rather than chosen. The Indian record is notable for three reasons: the constraint was linguistic, the methods were algorithmic rather than tabular, and the inverse map (index → object) was stated explicitly. That inverse map is the part a zero-allocation system needs.

---

## References

- Shah, J. (2008/2013). A history of Piṅgala's combinatorics. *Gaṇita Bhāratī* 35. — preprint: sanskrit.uohyd.ac.in/Algorithms_in_Ancient_India/Material/Pingala.pdf [VERIFY final citation]
- Datta, B. & Singh, A. N. (1935–38). *History of Hindu Mathematics.*
- Blelloch, G. E. (1990). Prefix sums and their applications. CMU-CS-90-190.
- Singh, P. (1985). The so-called Fibonacci numbers in ancient and medieval India. *Historia Mathematica* 12, 229–244.
- Ingerman, P. Z. (1967). "Pāṇini–Backus Form" suggested. *Communications of the ACM* 10(3), 137.
- Leibniz, G. W. (1703). Explication de l'Arithmétique Binaire. *Mémoires de l'Académie Royale des Sciences.*
- Knuth, D. E. *The Art of Computer Programming*, Vol. 4A, §7.2.1.3 (combinations) and §7.2.1.7 (history of generation).
- [VERIFY] Primary edition and translation of Piṅgala's *Chandaḥśāstra* with Halāyudha's *Mṛtasañjīvanī*.
- [VERIFY] Van Nooten, B. (1993). Binary numbers in Indian antiquity. *Journal of Indian Philosophy* 21, 31–50.
- [VERIFY] Sridharan, R. et al. on Nārāyaṇa's enumeration of combinations.

---

## Appendix A — Reference implementation and receipts

[`prastara/src/lib.rs`](prastara/src/lib.rs) contains ten exhaustive tests. Run them with `cargo test --release` from the repo root:

```
cargo test --release
test tests::nashta_is_binary_of_k_minus_1 ............ ok   (n ≤ 20, all k)
test tests::uddishta_inverts_nashta .................. ok   (n ≤ 20, all k)
test tests::sankhya_is_pow2 .......................... ok   (n ≤ 63)
test tests::row_with_r_gurus_counts_binomial ......... ok   (n ≤ 20)
test tests::combinadic_is_bijection_onto_weight_r .... ok   (n ≤ 20, all r, all k)
test tests::hockey_stick_column_prefix_sum ........... ok   (r ≤ 20, n ≤ 60)
test tests::suci_iterated_scan_is_binomial ........... ok   (k ≤ 20, i < 48)
test tests::shallow_diagonal_is_matra_count .......... ok   (n ≤ 80)
test tests::matra_unrank_matches_brute_force_lex ..... ok   (n ≤ 20, vs brute force)
test tests::matra_by_guru_count_is_binomial .......... ok   (n ≤ 20)
test result: ok. 10 passed; 0 failed — 0.70s (rustc 1.97.0, release)
```

Reference run: 2026-10-13, Linux x86_64. Re-run locally to reproduce.
