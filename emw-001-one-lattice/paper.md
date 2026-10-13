# One Lattice, Three Addresses
### State unranking in Sanskrit prosody and zero-allocation integer systems

**Sean (Deveraux)** · 2748684 Alberta Ltd · deveraux.dev
**Series:** East Meets West, EMW-001 · **Status:** DRAFT v0.3 (2026-10-13; v0.2 2026-10-12; v0.1 2026-10-13 UTC)
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
| c. 3rd–2nd c. BCE | Piṅgala, *Chandaḥśāstra* 8.20–8.35 | Terse sūtras: prastāra 8.20–23, naṣṭa 8.24–25, uddiṣṭa 8.26–27, saṅkhyā 8.28–31, adhvayoga 8.32–33, "pare pūrṇam" 8.34–35. The Yajur recension carries an extra lagakriyā sūtra ("ekottarakramaśaḥ pūrvapṛktā lasaṃkhyā") absent from Halāyudha's text. | Kedāranātha ed. 1938 (Kāvyamālā 91); Kulkarni 2008; Shah 2008 pp. 8, 29 |
| c. 1st c. CE | Bharata, *Nāṭyaśāstra* | **Lagakriyā by repeated partial sums**, dropping the terminal sum each pass: the earliest firmly attested binomial construction in the Indian record. | Shah 2008 pp. 2, 9–10 |
| c. 7th c. CE | Virahāṅka, *Vṛttajātisamuccaya* 6.7–6.9, 6.10–6.12 | Names and gives **both** constructions: **sūci-prastāra** ("needle spread", the partial-sum column method) and **meru-prastāra** (the two-cells-above triangle). Shah: "Sūci is Piṅgala's Lagakriyā"; meru first appears here, not in Halāyudha. Also the mātrā rule: each count = sum of the previous two. | Shah 2008 pp. 10, 19, 31–32 (translating Velankar 1962); Singh 1985 |
| before 1135 | Gopāla (commentary on Virahāṅka) | Worked values 3, 5, 8, 13, 21 | Singh 1985 |
| c. 1150 | Hemacandra | Same recurrence, stated independently | Singh 1985 |
| c. 950 | Halāyudha, *Mṛtasañjīvanī* (commentary on Piṅgala) | First to read the repeated sūtra 8.34–35 "pare pūrṇam iti" as an instruction to build the meru. That reading is **disputed**: Weber (1863) and Shah reject it, Alsdorf (1933) defends it. The triangle itself is three centuries older (Virahāṅka). | Shah 2008 pp. 2, 8, 29 |
| 1202 | Fibonacci, *Liber Abaci* | Same recurrence, in Europe | — |
| early 14th c. | *Prākṛta Paiṅgala* | **Mātrā-meru**: mātrā metres tabulated by guru count; entries are binomial | Singh 1985 |
| 1356 | Nārāyaṇa Paṇḍita, *Gaṇita Kaumudī* | **Sūcī-paṅkti** ("needle sequence"): a separate construction, built from products, yielding multinomial generalisations. Not the same as Virahāṅka's sūci-prastāra. | Singh 1985 |
| 1011–1077 | Shao Yong | The *xiantian* ("Fuxi") hexagram arrangement, later read as a binary ordering. Whether Shao Yong's own usage was binary is contested (Maitre 2025). | Swarthmore Leibniz–Bouvet archive; Maitre 2025 |
| 15 Mar 1679 | Leibniz, *De progressione dyadica* (MS, Hannover) | Binary arithmetic, dated in Leibniz's hand | Hochstetter & Greve 1966 facsimile |
| 4 Nov 1701 | Bouvet → Leibniz (received 1 Apr 1703) | Hexagram woodcut, which Leibniz took as **confirmation**, not origin. Leibniz had sent Bouvet his binary table on 15 Feb 1701. | Swarthmore Leibniz–Bouvet archive |
| 1703 (printed 1705) | Leibniz, *Explication de l'Arithmétique Binaire* | Sent to the Académie within a week of Bouvet's letter arriving; printed in the *Mémoires* for 1703 | Swarthmore Leibniz–Bouvet archive |
| 1967 | Ingerman, *CACM* 10(3) | Proposes "Pāṇini–Backus Form" | — |

**Corrections logged against circulating accounts:**
- *"Leibniz got binary from the I Ching."* False. He had binary in 1679 and received the hexagrams in 1703 (letter dated 1701).
- *"Piṅgala wrote Pascal's triangle."* Disputed. The triangle first appears by name in Virahāṅka (7th c.); Halāyudha (10th c.) is the first to attribute it to Piṅgala's sūtra, and Shah argues the sūtra doesn't support that. Shah does trace the *computation* of binomials to Piṅgala via the Yajur-recension lagakriyā sūtra.
- *Sūci-prastāra.* The **name** is Virahāṅka's (7th c.). The **construction** (repeated partial sums, terminal sum dropped) is already in Bharata's *Nāṭyaśāstra* (c. 1st c. CE), so Bharata, not Virahāṅka, holds the earliest firm attestation. Our v0.1 draft said "earliest … earlier than the meru-prastāra"; both halves were wrong, since Virahāṅka gives sūci and meru side by side. Nārāyaṇa's sūcī-paṅkti (1356) is a different object; don't conflate the two.
- *Naṣṭa orientation (our own v0.1 error).* Sūtra 8.24 *l-ardhe*: "if it halves, L"; 8.25 *sa-eke g*: "with one added, G". So an **even** row index yields L and an **odd** one yields G, row 1 being all-G. Our draft had the parities swapped. The "write 1, 2, 4 over the syllables and add" form of uddiṣṭa is Kedāra Bhaṭṭa's (*Vṛttaratnākara*, c. 1100), not Piṅgala's 8.26–27, which doubles from the first laghu and subtracts one at each guru. Corrected in §3–§4; the theorem and code were already consistent with the corrected reading once the bit is read as laghu.
- *"Oldest prefix-sum accumulator in history."* Unsupported superlative. Removed.
- *"Prosodists flattened the triangle to cut working-memory load."* A plausible motive, but no text says it. Removed.

---

## 3. Encoding

A metre of *n* syllables is a word in {L, G}ⁿ. Laghu (L, short) → 1, guru (G, long) → 0. Syllable *i* (0-indexed from the start of the line) is bit *i*, so **the first syllable is the least significant bit**. This is the convention under which Piṅgala's row *k* reads as the binary of *k* − 1 (Van Nooten 1993; Kulkarni 2008 §2.2.1). In mātrā (duration) metres, L weighs 1 and G weighs 2.

The same construction works in any radix *B*: Kulkarni (2008, Table 11) states Piṅgala's uddiṣṭa for base *B* directly. A balanced-ternary lattice (digits −1, 0, +1 stored as 0, 1, 2) is the *B* = 3 case, and its pack/unpack is naṣṭa/uddiṣṭa with 3 in place of 2.

---

## 4. Addressing I: positional (prastāra, naṣṭa, uddiṣṭa)

**Prastāra.** The canonical enumeration of {L, G}ⁿ starts at all-G. Each next row flips the first G to L and resets every earlier syllable to G. That is a binary counter running backwards. Indices are **1-based**.

**Naṣṭa (index → metre), sūtras 8.24–8.25.** Given row *k*: if *k* is even (*l-ardhe*, "it halves"), the syllable is L and *k* ← *k*/2. If *k* is odd (*sa-eke g*, "with one added"), the syllable is G and *k* ← (*k*+1)/2. Repeat *n* times.

**Uddiṣṭa (metre → index), sūtras 8.26–8.27.** Piṅgala: start at 1 on the last syllable and work backwards, doubling at each syllable and subtracting one after doubling at each guru. Kedāra Bhaṭṭa's later form (c. 1100) is the one usually quoted: write 1, 2, 4, … over the syllables, add the values over the **laghus**, then add 1. Both give the same number.

> **Theorem 1 (PROVEN).** Under the encoding of §3, naṣṭa(*k*) = *k* − 1, and uddiṣṭa is its inverse. The prastāra is the bijection {1…2ⁿ} ↔ {0,1}ⁿ given by little-endian binary of *k* − 1.
>
> *Proof.* Write *k* − 1 = Σ bᵢ2ⁱ. Then *k* even ⇔ b₀ = 1 ⇔ L. Both update rules map *k* to 1 + ⌊(*k*−1)/2⌋, which shifts *k* − 1 right by one bit. By induction on *n*, the *i*-th emitted syllable is bᵢ. Kedāra's uddiṣṭa computes 1 + Σ bᵢ2ⁱ = *k* directly; Piṅgala's backward doubling is Horner's rule on the same digits. ∎

**Saṅkhyā (count).** |{L, G}ⁿ| = 2ⁿ, computed by Piṅgala's rule: for even *n*, halve and square; for odd *n*, subtract one and double. This is exponentiation by squaring, O(log *n*), the same method modular exponentiation in RSA uses.

**Cost:** O(*n*) bit operations, no table, no allocation.

---

## 5. Addressing II: weight-constrained (meru-prastāra)

The number of *n*-syllable metres with exactly *r* laghus is C(*n*, *r*). Lagakriyā ("the L-reckoning") computes these values; the sūci and meru prastāras tabulate them.

> **Theorem 2 (PROVEN).** The weight-*r* rows of the prastāra are addressable by the **combinatorial number system**: every 0 ≤ *k* < C(*n*,*r*) has a unique representation *k* = Σⱼ₌₁ʳ C(cⱼ, j) with *n* > c_r > … > c₁ ≥ 0, and the laghu positions are {cⱼ}. Unranking is greedy: for *j* = *r* down to 1, take the largest *c* with C(*c*, *j*) ≤ *k*, set laghu *c*, and subtract.
>
> *Proof.* Standard (Lehmer; Knuth TAOCP 7.2.1.3). Exhaustively checked for *n* ≤ 20 in Appendix A. ∎

**Cost:** O(*n*) binomial evaluations. In the reference implementation the binomials come from a compile-time table built by the sūci-prastāra of §5.1 (add-only; 65 × 65 entries, every C(*n*, *k*) with *n* ≤ 64), so the unranking path contains no division. The multiplicative formula, which divides, is kept only as a test oracle for that table.

### 5.1 Sūci-prastāra: the binomials as an iterated scan

Bharata, and after him Virahāṅka under the name sūci-prastāra, build the binomial columns by partial sums. Column 1 is 1, 2, …, *n*. Each later column is the running sum of the one before, with the last entry dropped (Shah 2008 pp. 9–10, 31–32; Virahāṅka 6.7–6.9: "one-by-one, add the number above … successively leave out the last number").

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

### 5.2 Signed weight: exactly *r* non-zero digits in {−1, 0, +1}ⁿ

Split a balanced-ternary vector into its **support** *s* (bit *i* set iff digit *i* ≠ 0) and its **sign pattern** *p* ⊆ *s* (bit *i* set iff digit *i* = +1). Let *r* = |*s*|.

> **Theorem 6 (PROVEN). Signed-weight address.** For fixed *n* and *r*, the map
> **rank(*s*, *p*) = combinadic_rank(*s*) · 2ʳ + pext(*p*, *s*)**
> is a bijection from the vectors with exactly *r* non-zero digits onto [0, C(*n*, *r*) · 2ʳ), where pext(*p*, *s*) packs the bits of *p* at the positions of *s* into an *r*-bit integer. The inverse is *s* = combinadic_unrank(⌊*k* / 2ʳ⌋), *p* = pdep(*k* mod 2ʳ, *s*). The classes *r* = 0 … *n* partition the 3ⁿ vectors.
>
> *Proof.* Theorem 2 gives the bijection supports ↔ [0, C(*n*, *r*)). For a fixed support, pext/pdep are mutually inverse bijections between sign patterns and [0, 2ʳ). Composing by the mixed radix (C(*n*, *r*), 2ʳ) gives the result. Σᵣ C(*n*, *r*) 2ʳ = 3ⁿ by the binomial theorem. Exhaustively checked over all 3¹³ = 1 594 323 vectors and at every width *n* ≤ 9 (Appendix A). ∎

This is enumerative coding (Cover 1973; the fixed-weight case is Schalkwijk 1972) under the lexicographic order (support, then sign pattern), applied to a ternary constant-weight set. It is a re-derivation, not a new method. The division is again absent: ⌊*k* / 2ʳ⌋ and *k* mod 2ʳ are a shift and a mask, and the binomials are the §5.1 table. pext/pdep are the BMI2 instructions on x86-64; the reference implementation uses a portable O(*r*) loop by default and the instructions only when the build enables them, and checks the two bit-identical. The address fits a u64 for *n* ≤ 40 (3⁴⁰ < 2⁶⁴ < 3⁴¹).

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
> Checked for *n* ≤ 64 (the extent of the binomial table), and per-guru counts checked for *n* ≤ 20.

**Fibonacci in O(log n).** Fast doubling: F(2*m*) = F(*m*)(2F(*m*+1) − F(*m*)) and F(2*m*+1) = F(*m*)² + F(*m*+1)². This is the saṅkhyā squaring pattern applied to a different recurrence. The same procedure computes both counts.

**Cost:** O(*n*) steps, one O(log *n*) Fibonacci evaluation per step (or a 94-entry const table, since F(93) is the largest Fibonacci number that fits in u64). No allocation.

---

## 7. The three addressings compared

| Constraint | Count | Unrank method | Prosodic source |
|---|---|---|---|
| fixed length *n* | 2ⁿ | binary digits of *k*−1 | Piṅgala 8.24–8.27: naṣṭa / uddiṣṭa |
| fixed length *n*, *r* laghus | C(*n*,*r*) | greedy on binomials | lagakriyā (Bharata); sūci- and meru-prastāra (Virahāṅka) |
| fixed duration *n* | F(*n*+1) | greedy on Fibonacci | Virahāṅka; mātrā-meru (*Prākṛta Paiṅgala*) |

Each row is a numeration system whose place values are the counts of the constrained space (powers of two, binomials, Fibonacci numbers). Unranking means writing *k* in that system. This is the single lattice the title refers to.

---

## 8. Engineering mapping (13Forge)

| Use | Addressing | Why it fits | Status |
|---|---|---|---|
| Deterministic state #*k* with no lookup table | I | Index → state in O(*n*) integer ops; nothing allocated. **Bounded, two numbers:** at one byte of state (3⁵ = 243 codes), comparing a 64 KB pair-distance table against register decode on a desktop x86 core (Zen 3), working sets of 32 KB to 2 MB, warm and after a 64 MB cache evict alike: in **throughput** (independent ops) the table runs 0.8–1.2 ns/op against 7.2–7.9 ns/op for decode, 6–9×; in **latency** (each op's inputs derived from the previous result) the table runs 3.4–4.5 ns/op against 10.4–11.9 ns/op for decode, 2.3–3.1×. An explicit multiply-shift decode (no divide instruction) sits at 7.0–7.2 ns/op throughput and 8.6–9.2 ns/op latency, 2.0–2.6× the table in latency. The decode cost is the dependent chain of digit extractions, not the divide: the compiler already strength-reduces a constant divisor. Table-free wins only where a table cannot live: wide states (3¹³ and up), register-only GPU kernels, no_std targets without a data cache. | DESIGN_TARGET (bench exists, local) |
| Exactly *r* active of *n* (voices, channels, capability bits) | II | Dense index over valid states only; invalid states can't be addressed at all | DESIGN_TARGET |
| Exactly *r* non-zero of *n* **signed** digits (balanced ternary) | II × I | Theorem 6: class size C(*n*,*r*)·2ʳ, address = combinadic rank of the support, shifted left *r*, OR the sign bits compressed onto the support. **Re-derivation** of enumerative coding (Cover 1973; Schalkwijk 1972) for a ternary constant-weight set; the refinement by sign count C(*n*,*r*)·C(*r*,*p*) is the constant-composition case and the multinomial Nārāyaṇa's sūcī-paṅkti counts. At one byte (*n* = 5) the 243 (*r*, rank) pairs are a table generated at compile time *from* the formula; the formula is the generator and validator, the table is the hot path. | PROVEN (3¹³ exhaustive); integration DESIGN_TARGET |
| Rhythm space under a fixed bar length | III | Enumerates every subdivision of *n* ticks into 1- and 2-tick units, densely and in order | DESIGN_TARGET |
| Division-free binomial tables | II (sūci) | Iterated scans; add-only integer path; maps onto SIMD scan. Built as a compile-time table in the reference implementation (§5) | BUILT (reference); SIMD DESIGN_TARGET |
| Sequence acceptance | I–III | A *sequence* (a metre, a mātrā string, a support mask) is well-formed iff ranking it and unranking the result gives it back: a mātrā string whose durations do not sum to *n*, or a support with the wrong weight, cannot round-trip. The rank round-trip is an acceptance test for the **input alphabet**, nothing more. | DESIGN_TARGET |
| ID integrity | I–III | A dense ID carries zero redundancy by construction: every integer in [0, count) is some valid state, so a flipped bit in the ID decodes to a valid *wrong* state and no round-trip can notice. Integrity of an ID in flight needs a check field or reserved capacity (addresses above the count) added on top; the addressing itself gives none. | DESIGN_TARGET |

Prior-art search for the signed-weight row (2026-10-13, query "ternary constant-composition code enumerative ranking"): the hits are existence and classification results for ternary constant-composition codes (Bogdanova & Kapralov 2003; Chee, Ge & Ling; Gao & Ge), none on ranking or unranking. The search was not empty, so no candidate claim is made; the row stays a re-derivation of Cover/Schalkwijk.

The reference implementation (Appendix A) is `no_std`, integer-only, and allocation-free. Promote the remaining rows to **BUILT** once it is lifted into the workspace with benches.

---

## 9. Thesis

Piṅgala, Shao Yong, Leibniz and Shannon arrived at binary enumeration independently. Virahāṅka, Gopāla, Hemacandra and Fibonacci arrived at the mātrā recurrence independently. The binomial triangle appears in India, Persia, China and Europe. These recurrences are not cultural borrowings. They are what you get when you have to count a discrete space under a hard constraint, and they are found again because they are forced by that problem rather than chosen. The Indian record is notable for three reasons: the constraint was linguistic, the methods were algorithmic rather than tabular, and the inverse map (index → object) was stated explicitly. That inverse map is the part a zero-allocation system needs.

---

## References

Page numbers for Shah refer to the 2008 preprint (44 pp.), which is the text we read.

- Piṅgala. *Chandaḥśāstram*, with Halāyudha's *Mṛtasañjīvanī* and Madhusūdana's *Chandonirukti*. Ed. Kedāranātha (Jaipur). Kāvyamālā 91, Bombay, 1938. Transliteration of the sūtras: ancient-buddhist-texts.net/Textual-Studies/Chandas-Sastra. [VERIFY: publisher imprint of Kāvyamālā 91 not seen; commonly given as Nirnaya Sagar Press]
- Virahāṅka. *Vṛttajātisamuccaya*, with Gopāla's commentary. Ed. H. D. Velankar. Rajasthan Oriental Research Institute, Jodhpur, 1962 (Rajasthan Puratana Granthamala 61). [VERIFY: edition not consulted directly; verses 6.7–6.12 read in Shah's translation]
- Shah, J. (2008). A history of Piṅgala's combinatorics. Preprint, 44 pp.: sanskrit.uohyd.ac.in/Algorithms_in_Ancient_India/Material/Pingala.pdf. Published as *Gaṇita Bhāratī* 35(1–2), 2013, 1–54 [pages from a secondary listing, not seen].
- Kulkarni, A. (2008). Recursion and combinatorial mathematics in Chandaśśāstra. arXiv:math/0703658v2 [math.HO]. (Sūtra numbering 8.20–8.35; base-*B* uddiṣṭa, Table 11.)
- Weber, A. (1863). *Über die Metrik der Inder.* Berlin (Indische Studien 8). [VERIFY: Shah's text says 1835 in prose and 1863 in the reference list]
- Alsdorf, L. (1933). Die Pratyayas. Ein Beitrag zur indischen Mathematik. *Zeitschrift für Indologie und Iranistik* 9, 97–157. English tr. S. R. Sarma, *Indian Journal of History of Science* 26(1), 1991.
- Van Nooten, B. (1993). Binary numbers in Indian antiquity. *Journal of Indian Philosophy* 21(1), 31–50.
- Singh, P. (1985). The so-called Fibonacci numbers in ancient and medieval India. *Historia Mathematica* 12(3), 229–244. doi:10.1016/0315-0860(85)90021-7.
- Sridharan, Raja, Sridharan, R. & Srinivas, M. D. (2012). Nārāyaṇa Paṇḍita's enumeration of combinations and associated representation of numbers as sums of binomial coefficients. *Indian Journal of History of Science* 47, 607–631.
- Kusuba, T. (1993). *Combinatorics and Magic Squares in India: A Study of Nārāyaṇa Paṇḍita's Gaṇitakaumudī, Chapters 13–14.* PhD thesis, Brown University.
- Datta, B. & Singh, A. N. (1935–38). *History of Hindu Mathematics.*
- Maitre, M.-J. (2025). Is the *Fuxi liushisi gua fangwei* diagram attributed to Shao Yong binary? *Science in Context.*
- Leibniz, G. W. (1679). *De progressione dyadica.* MS dated 15 March 1679, Niedersächsische Landesbibliothek, Hannover. Facsimile: Hochstetter & Greve (1966).
- Leibniz, G. W. (1703, printed 1705). Explication de l'Arithmétique Binaire. *Mémoires de l'Académie Royale des Sciences.* Leibniz–Bouvet correspondence: archive.swarthmore.edu/leibniz-bouvet (letter of 4 Nov 1701, received 1 Apr 1703).
- Ingerman, P. Z. (1967). "Pāṇini–Backus Form" suggested. *Communications of the ACM* 10(3), 137. doi:10.1145/363162.363165.
- Cover, T. M. (1973). Enumerative source encoding. *IEEE Transactions on Information Theory* 19(1), 73–77.
- Schalkwijk, J. P. M. (1972). An algorithm for source coding. *IEEE Transactions on Information Theory* 18(3), 395–399.
- Bogdanova, G. & Kapralov, S. (2003). Enumeration of optimal ternary constant-composition codes. *Problems of Information Transmission* 39(4). doi:10.1023/B:PRIT.0000011273.98799.a8. (Classification, not ranking; cited as the nearest prior-art hit.)
- Blelloch, G. E. (1990). Prefix sums and their applications. CMU-CS-90-190.
- Knuth, D. E. *The Art of Computer Programming*, Vol. 4A, §7.2.1.3 (combinations) and §7.2.1.7 (history of generation).

---

## Appendix A — Reference implementation and receipts

[`prastara/src/lib.rs`](prastara/src/lib.rs) is `no_std` with no `alloc` crate (the test module alone uses `std`), contains no division, and carries sixteen exhaustive tests. Run them with `cargo test --release` from the repo root:

```
cargo test --release
test tests::suci_table_matches_multiplicative_binom .. ok   (n ≤ 64, all k: compile-time sūci table == multiplicative formula)
test tests::nashta_is_binary_of_k_minus_1 ............ ok   (n ≤ 20, all k)
test tests::uddishta_inverts_nashta .................. ok   (n ≤ 20, all k)
test tests::sankhya_is_pow2 .......................... ok   (n ≤ 63)
test tests::row_with_r_laghus_counts_binomial ........ ok   (n ≤ 20)
test tests::combinadic_is_bijection_onto_weight_r .... ok   (n ≤ 20, all r, all k)
test tests::hockey_stick_column_prefix_sum ........... ok   (r ≤ 20, n ≤ 60)
test tests::suci_iterated_scan_is_binomial ........... ok   (k ≤ 16, i < 48)
test tests::shallow_diagonal_is_matra_count .......... ok   (n ≤ 64)
test tests::matra_unrank_matches_brute_force_lex ..... ok   (n ≤ 20, vs brute force)
test tests::matra_by_guru_count_is_binomial .......... ok   (n ≤ 20)
test tests::pext_pdep_hardware_matches_portable ...... ok   (BMI2 vs portable: all 12-bit (x, mask) pairs at offsets 0 and 52, all 3¹³ (sign, support) pairs)
test tests::pext_pdep_dispatch_matches_portable ...... ok   (the build's pext/pdep vs portable, all 3¹³)
test tests::signed_weight_collapse_3_pow_13 .......... ok   (all 1 594 323 states: unrank(rank(x)) = x, rank < C(13,r)·2ʳ, every class covered exactly once)
test tests::signed_weight_collapse_small_widths ...... ok   (same, every n ≤ 9)
test tests::signed_max_n_is_the_u64_ceiling .......... ok   (3⁴⁰ < 2⁶⁴ < 3⁴¹)
test result: ok. 16 passed; 0 failed; 0 ignored — finished in 0.40s
```

Receipts, 2026-10-13, Windows 11 x86_64 (AMD Ryzen 7 5800X3D), all exit 0:

| Toolchain | Build | Result | Wall |
|---|---|---|---|
| **stable** rustc 1.95.0 (59807616e 2026-04-14) | `cargo +stable test --release` | 16 passed, 0 failed; pext dispatch = portable | 0.45 s |
| beta rustc 1.100.0-beta.4 | `cargo test --release` | 16 passed, 0 failed; pext dispatch = portable | 0.40 s |
| beta rustc 1.100.0-beta.4 | `RUSTFLAGS=-C target-feature=+bmi2 cargo test --release` | 16 passed, 0 failed; pext dispatch = BMI2 hardware | 0.37 s |
| beta rustc 1.100.0-beta.4 | `cargo build --release --target wasm32-unknown-unknown` | `no_std` library builds | 25.7 s |

The byte-width bench behind §8 row 1 (stable rustc 1.95.0 primary; beta 1.100.0-beta.4 agrees within noise) ran 6 reps per cell and reports ns/op; the full table is in the bench source's output, not reproduced here.

In the code, bit *i* = 1 means syllable *i* is **laghu**, per §3. The `nashta` rule `k even → set bit` is sūtra 8.24 read literally.
