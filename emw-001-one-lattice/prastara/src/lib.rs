//! prastara — reference implementation for EMW-001 (draft).
//! Three table-free, zero-allocation state addressings from Sanskrit prosody, plus the
//! signed-weight (combinadic × sign) address for balanced-ternary vectors.
//! Encoding: laghu (L, short) = 1, guru (G, long) = 0. Position 0 = first syllable = LSB.
//! Under this convention Piṅgala's row k is the little-endian binary of k - 1
//! (Van Nooten 1993; Kulkarni 2008 §2.2.1). A set bit is a laghu.
//!
//! Arithmetic contract: no division anywhere in the library. Binomials come from a
//! compile-time sūci table (add-only scans); the multiplicative formula survives only
//! as a test oracle. `no_std`, no `alloc`, no heap.
#![cfg_attr(not(test), no_std)]

/// Naṣṭa (Chandaḥśāstra 8.24–8.25): 1-indexed row k of the n-syllable prastāra -> laghu bitmask.
/// Literal rule: k even ("l-ardhe", it halves) -> L, k <- k/2 ;
///               k odd  ("sa-eke g", add one)  -> G, k <- (k+1)/2.
/// The halvings are shifts; the compiler emits no divide.
pub const fn nashta(mut k: u64, n: u32) -> u64 {
    let mut mask = 0u64;
    let mut i = 0;
    while i < n {
        if k & 1 == 1 {
            k = (k + 1) >> 1;
        } else {
            mask |= 1 << i;
            k >>= 1;
        }
        i += 1;
    }
    mask
}

/// Uddiṣṭa: laghu bitmask -> 1-indexed row.
/// Kedāra Bhaṭṭa's form (Vṛttaratnākara, c. 1100): write 1,2,4,... over the syllables;
/// sum the values over the laghus; add 1. Piṅgala's own 8.26–8.27 (double from the
/// first laghu, subtract one at each guru) is Horner's rule on the same digits.
pub const fn uddishta(mask: u64, n: u32) -> u64 {
    let mut k = 1u64;
    let mut i = 0;
    while i < n {
        if (mask >> i) & 1 == 1 {
            k += 1 << i;
        }
        i += 1;
    }
    k
}

/// Saṅkhyā: 2^n by Piṅgala's halve/square, subtract-one/double rule. O(log n).
pub const fn sankhya(n: u32) -> u64 {
    if n == 0 {
        1
    } else if n & 1 == 0 {
        let h = sankhya(n >> 1);
        h * h
    } else {
        2 * sankhya(n - 1)
    }
}

/// Largest n for which every C(n, k) fits in u64 (C(64, 32) ≈ 1.8·10¹⁸ < 2⁶⁴).
pub const BINOM_MAX_N: usize = 64;

/// C(n, k) for 0 ≤ n, k ≤ [`BINOM_MAX_N`], built at compile time by the sūci-prastāra:
/// k inclusive scans of the all-ones vector leave C(i + k, k) at index i (Theorem 5).
/// Add-only. No multiply, no divide, here or in [`binom`]. 65 × 65 × 8 = 33 800 bytes
/// of read-only data; entries with k > n are 0.
pub const BINOM: [[u64; BINOM_MAX_N + 1]; BINOM_MAX_N + 1] = build_suci_table();

/// Virahāṅka's construction (Shah 2008 pp. 9–10, 31–32): each column is the running
/// sum of the previous one with the last entry dropped. Column k has 65 − k live
/// entries; the scan for column k + 1 runs over 64 − k of them, so no value outside
/// the table is ever formed and nothing can overflow.
const fn build_suci_table() -> [[u64; BINOM_MAX_N + 1]; BINOM_MAX_N + 1] {
    let mut t = [[0u64; BINOM_MAX_N + 1]; BINOM_MAX_N + 1];
    let mut buf = [1u64; BINOM_MAX_N + 1]; // column 0: all ones
    let mut k = 0;
    while k <= BINOM_MAX_N {
        let len = BINOM_MAX_N + 1 - k; // live entries of column k: C(k..=64, k)
        let mut i = 0;
        while i < len {
            t[i + k][k] = buf[i];
            i += 1;
        }
        // sūci pass -> column k + 1, terminal entry dropped
        let mut acc = 0u64;
        let mut i = 0;
        while i + 1 < len {
            acc += buf[i];
            buf[i] = acc;
            i += 1;
        }
        k += 1;
    }
    t
}

/// C(n, r) from the sūci table. 0 when r > n. Requires n ≤ [`BINOM_MAX_N`] (index panic otherwise).
#[inline]
pub const fn binom(n: u32, r: u32) -> u64 {
    if r > n {
        return 0;
    }
    BINOM[n as usize][r as usize]
}

/// Sūci-prastāra (Virahāṅka, per Shah): column k is the running partial sum of column k-1,
/// with the last entry dropped. Implemented as k in-place inclusive scans over a caller buffer.
/// Add-only: no multiply, no divide. After the call, buf[i] = C(i + k, k) for i in 0..len
/// (equivalently: entry i of column k in the needle layout, before the shift).
/// This is the runtime form of the pass that builds [`BINOM`].
pub fn suci_scan(buf: &mut [u64], k: u32) {
    for x in buf.iter_mut() {
        *x = 1;
    }
    for _ in 0..k {
        let mut acc = 0u64;
        for x in buf.iter_mut() {
            acc += *x;
            *x = acc;
        }
    }
}

/// Combinadic unrank: 0-indexed rank k (colex) -> bitmask with exactly r laghus among n.
pub const fn combinadic_unrank(mut k: u64, n: u32, r: u32) -> u64 {
    let mut mask = 0u64;
    let mut c = n;
    let mut j = r;
    while j >= 1 {
        c -= 1;
        while binom(c, j) > k {
            c -= 1;
        }
        mask |= 1 << c;
        k -= binom(c, j);
        j -= 1;
    }
    mask
}

/// Combinadic rank: inverse of combinadic_unrank.
pub const fn combinadic_rank(mask: u64) -> u64 {
    let mut k = 0u64;
    let mut j = 0u32;
    let mut m = mask;
    while m != 0 {
        let c = m.trailing_zeros();
        j += 1;
        k += binom(c, j);
        m &= m - 1;
    }
    k
}

// ---------------------------------------------------------------------------
// Parallel bit extract / deposit. Portable forms are the default; the BMI2
// instructions are used only when the build enables them (`-C target-feature=+bmi2`
// or `-C target-cpu=native` on a BMI2 core). Both forms are bit-identical.
// ---------------------------------------------------------------------------

/// Portable `pext`: the bits of `x` at the set positions of `mask`, packed into the low
/// bits of the result in ascending order. O(popcount(mask)) iterations, no table.
pub const fn pext_soft(x: u64, mask: u64) -> u64 {
    let mut m = mask;
    let mut out = 0u64;
    let mut k = 0u32;
    while m != 0 {
        let low = m & m.wrapping_neg();
        if x & low != 0 {
            out |= 1 << k;
        }
        k += 1;
        m &= m - 1;
    }
    out
}

/// Portable `pdep`: the low popcount(mask) bits of `x`, scattered onto the set positions
/// of `mask` in ascending order. Inverse of [`pext_soft`] on `mask`.
pub const fn pdep_soft(x: u64, mask: u64) -> u64 {
    let mut m = mask;
    let mut out = 0u64;
    let mut k = 0u32;
    while m != 0 {
        let low = m & m.wrapping_neg();
        if (x >> k) & 1 != 0 {
            out |= low;
        }
        k += 1;
        m &= m - 1;
    }
    out
}

/// Hardware `pext` (BMI2).
///
/// # Safety
/// The CPU must support BMI2. Callers compiled with `target_feature = "bmi2"` satisfy
/// this statically; others must check `is_x86_feature_detected!("bmi2")` first.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "bmi2")]
#[allow(unused_unsafe)] // the intrinsic is a safe fn on 1.87+, unsafe on the 1.85 floor
pub unsafe fn pext_bmi2(x: u64, mask: u64) -> u64 {
    // SAFETY: the caller guarantees BMI2 is available (contract above).
    unsafe { core::arch::x86_64::_pext_u64(x, mask) }
}

/// Hardware `pdep` (BMI2).
///
/// # Safety
/// Same contract as [`pext_bmi2`].
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "bmi2")]
#[allow(unused_unsafe)] // the intrinsic is a safe fn on 1.87+, unsafe on the 1.85 floor
pub unsafe fn pdep_bmi2(x: u64, mask: u64) -> u64 {
    // SAFETY: the caller guarantees BMI2 is available (contract above).
    unsafe { core::arch::x86_64::_pdep_u64(x, mask) }
}

/// `pext` as built: hardware when the build enables BMI2, [`pext_soft`] otherwise.
#[inline]
pub fn pext(x: u64, mask: u64) -> u64 {
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
    {
        // SAFETY: `target_feature = "bmi2"` is a compile-time guarantee for this build.
        unsafe { pext_bmi2(x, mask) }
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
    {
        pext_soft(x, mask)
    }
}

/// `pdep` as built: hardware when the build enables BMI2, [`pdep_soft`] otherwise.
#[inline]
pub fn pdep(x: u64, mask: u64) -> u64 {
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
    {
        // SAFETY: `target_feature = "bmi2"` is a compile-time guarantee for this build.
        unsafe { pdep_bmi2(x, mask) }
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
    {
        pdep_soft(x, mask)
    }
}

/// True when this build dispatches [`pext`] / [`pdep`] to the BMI2 instructions.
pub const fn pext_is_hardware() -> bool {
    cfg!(all(target_arch = "x86_64", target_feature = "bmi2"))
}

// ---------------------------------------------------------------------------
// Signed-weight addressing (Addressing II × I): n balanced-ternary digits with
// exactly r non-zero. Class size C(n, r)·2^r; Σ_r = 3^n. Enumerative coding
// (Cover 1973) under the lexicographic order (support, then sign pattern).
// ---------------------------------------------------------------------------

/// Largest n for which every class address C(n, r)·2^r fits in u64: 3⁴⁰ < 2⁶⁴ < 3⁴¹.
pub const SIGNED_MAX_N: u32 = 40;

/// Number of n-digit balanced-ternary vectors with exactly r non-zero digits: C(n, r)·2^r.
#[inline]
pub const fn signed_class_size(n: u32, r: u32) -> u64 {
    binom(n, r) << r
}

/// The low `r` bits set. r ≤ 63.
#[inline]
const fn low_bits(r: u32) -> u64 {
    (1u64 << r) - 1
}

/// Signed-weight rank, portable/const form. `pos` and `neg` are disjoint bitmasks
/// (bit i set = digit i is +1 / −1). With s = pos | neg and r = popcount(s):
/// rank = combinadic_rank(s)·2^r + pext(pos, s). Dense: for fixed (n, r) the ranks are
/// exactly [0, C(n, r)·2^r). Requires n ≤ [`SIGNED_MAX_N`].
pub const fn signed_rank_soft(pos: u64, neg: u64) -> u64 {
    debug_assert!(pos & neg == 0);
    let s = pos | neg;
    let r = s.count_ones();
    (combinadic_rank(s) << r) | pext_soft(pos, s)
}

/// Inverse of [`signed_rank_soft`] for the class (n, r): returns (pos, neg).
pub const fn signed_unrank_soft(rank: u64, n: u32, r: u32) -> (u64, u64) {
    let s = combinadic_unrank(rank >> r, n, r);
    let pos = pdep_soft(rank & low_bits(r), s);
    (pos, s & !pos)
}

/// Signed-weight rank through [`pext`] (hardware when built with BMI2).
#[inline]
pub fn signed_rank(pos: u64, neg: u64) -> u64 {
    debug_assert!(pos & neg == 0);
    let s = pos | neg;
    let r = s.count_ones();
    (combinadic_rank(s) << r) | pext(pos, s)
}

/// Inverse of [`signed_rank`] for the class (n, r): returns (pos, neg).
#[inline]
pub fn signed_unrank(rank: u64, n: u32, r: u32) -> (u64, u64) {
    let s = combinadic_unrank(rank >> r, n, r);
    let pos = pdep(rank & low_bits(r), s);
    (pos, s & !pos)
}

/// Fibonacci F(n) by fast doubling (saṅkhyā-style). F(0)=0, F(1)=1. Valid n <= 93.
pub const fn fib(n: u32) -> u64 {
    const fn fd(n: u32) -> (u64, u64) {
        if n == 0 {
            return (0, 1);
        }
        let (a, b) = fd(n >> 1);
        let c = a * (2 * b - a);
        let d = a * a + b * b;
        if n & 1 == 0 { (c, d) } else { (d, c + d) }
    }
    fd(n).0
}

/// Mātrā count: number of L/G sequences of total duration n (L=1, G=2) = F(n+1).
pub const fn matra_count(n: u32) -> u64 {
    fib(n + 1)
}

/// Mātrā unrank: 0-indexed k -> sequence of total duration n, lexicographic with L < G.
/// Writes 1 (L) / 2 (G) into `out`; returns syllable count. No allocation.
pub fn matra_unrank(mut k: u64, mut n: u32, out: &mut [u8]) -> usize {
    let mut i = 0;
    while n > 0 {
        let with_l = matra_count(n - 1);
        if n == 1 || k < with_l {
            out[i] = 1;
            n -= 1;
        } else {
            k -= with_l;
            out[i] = 2;
            n -= 2;
        }
        i += 1;
    }
    i
}

/// Mātrā rank: inverse of matra_unrank.
pub fn matra_rank(seq: &[u8], mut n: u32) -> u64 {
    let mut k = 0u64;
    for &s in seq {
        if s == 2 {
            k += matra_count(n - 1);
        }
        n -= s as u32;
    }
    k
}

#[cfg(test)]
mod tests {
    use super::*;
    const N: u32 = 20;

    /// The multiplicative formula the library used before the sūci table. Test oracle
    /// only: it divides, and it needs u128 to survive n = 64.
    fn binom_multiplicative(n: u32, r: u32) -> u64 {
        if r > n {
            return 0;
        }
        let r = r.min(n - r) as u128;
        let n = n as u128;
        let mut acc = 1u128;
        for i in 0..r {
            acc = acc * (n - i) / (i + 1);
        }
        acc as u64
    }

    #[test]
    fn suci_table_matches_multiplicative_binom() {
        for n in 0..=BINOM_MAX_N as u32 {
            for k in 0..=BINOM_MAX_N as u32 {
                assert_eq!(binom(n, k), binom_multiplicative(n, k), "C({n},{k})");
            }
        }
        assert_eq!(binom(64, 32), 1_832_624_140_942_590_534);
    }

    #[test]
    fn nashta_is_binary_of_k_minus_1() {
        for n in 1..=N {
            for k in 1..=(1u64 << n) {
                assert_eq!(nashta(k, n), k - 1);
            }
        }
    }

    #[test]
    fn uddishta_inverts_nashta() {
        for n in 1..=N {
            for k in 1..=(1u64 << n) {
                assert_eq!(uddishta(nashta(k, n), n), k);
            }
        }
    }

    #[test]
    fn sankhya_is_pow2() {
        for n in 0..=63 {
            assert_eq!(sankhya(n), 1u64 << n);
        }
    }

    #[test]
    fn row_with_r_laghus_counts_binomial() {
        for n in 1..=N {
            let mut counts = [0u64; 64];
            for m in 0..(1u64 << n) {
                counts[m.count_ones() as usize] += 1;
            }
            for r in 0..=n {
                assert_eq!(counts[r as usize], binom(n, r));
            }
        }
    }

    #[test]
    fn combinadic_is_bijection_onto_weight_r() {
        for n in 1..=N {
            for r in 0..=n {
                let total = binom(n, r);
                let mut prev: Option<u64> = None;
                for k in 0..total {
                    let m = combinadic_unrank(k, n, r);
                    assert_eq!(m.count_ones(), r);
                    assert!(m < (1u64 << n));
                    assert_eq!(combinadic_rank(m), k);
                    if let Some(p) = prev {
                        assert!(m > p, "colex order is strictly increasing as integers");
                    }
                    prev = Some(m);
                }
            }
        }
    }

    #[test]
    fn hockey_stick_column_prefix_sum() {
        for r in 0..=N {
            for n in r..=60 {
                let s: u64 = (r..=n).map(|i| binom(i, r)).sum();
                assert_eq!(s, binom(n + 1, r + 1));
            }
        }
    }

    #[test]
    fn suci_iterated_scan_is_binomial() {
        let mut buf = [0u64; 48];
        for k in 0..=16 {
            suci_scan(&mut buf, k);
            for i in 0..buf.len() as u32 {
                assert_eq!(buf[i as usize], binom(i + k, k));
            }
        }
    }

    #[test]
    fn shallow_diagonal_is_matra_count() {
        for n in 0..=BINOM_MAX_N as u32 {
            let s: u64 = (0..=n / 2).map(|g| binom(n - g, g)).sum();
            assert_eq!(s, matra_count(n));
        }
    }

    #[test]
    fn matra_unrank_matches_brute_force_lex_enumeration() {
        for n in 1..=N {
            // brute force: all L/G strings of length <= n, keep those of duration n, sort lex L<G
            let mut all: Vec<Vec<u8>> = Vec::new();
            for len in 1..=n {
                for m in 0..(1u64 << len) {
                    let seq: Vec<u8> = (0..len).map(|i| if (m >> i) & 1 == 1 { 2 } else { 1 }).collect();
                    if seq.iter().map(|&x| x as u32).sum::<u32>() == n {
                        all.push(seq);
                    }
                }
            }
            all.sort();
            assert_eq!(all.len() as u64, matra_count(n));
            let mut buf = [0u8; 64];
            for (k, want) in all.iter().enumerate() {
                let len = matra_unrank(k as u64, n, &mut buf);
                assert_eq!(&buf[..len], &want[..]);
                assert_eq!(matra_rank(&buf[..len], n), k as u64);
            }
        }
    }

    #[test]
    fn matra_by_guru_count_is_binomial() {
        for n in 1..=N {
            let mut buf = [0u8; 64];
            let mut by_g = [0u64; 64];
            for k in 0..matra_count(n) {
                let len = matra_unrank(k, n, &mut buf);
                let g = buf[..len].iter().filter(|&&x| x == 2).count();
                by_g[g] += 1;
            }
            for g in 0..=(n / 2) {
                assert_eq!(by_g[g as usize], binom(n - g, g));
            }
        }
    }

    /// Base-3 digits of `code` (digit 0 = LSB) as (pos, neg) masks: 2 -> +1, 0 -> -1, 1 -> 0.
    fn ternary_masks(mut code: u64, n: u32) -> (u64, u64) {
        let (mut pos, mut neg) = (0u64, 0u64);
        for i in 0..n {
            match code % 3 {
                2 => pos |= 1 << i,
                0 => neg |= 1 << i,
                _ => {}
            }
            code /= 3;
        }
        (pos, neg)
    }

    /// Portable and hardware pext/pdep agree bit for bit. Exhaustive over every (x, mask)
    /// pair of 12 bits at offsets 0 and 52, and over every (pos, support) pair of the
    /// 3^13 lattice. Skipped with a message, never silently, when the CPU lacks BMI2.
    #[test]
    fn pext_pdep_hardware_matches_portable() {
        #[cfg(target_arch = "x86_64")]
        {
            if !std::is_x86_feature_detected!("bmi2") {
                eprintln!("SKIP pext_pdep_hardware_matches_portable: CPU lacks BMI2");
                return;
            }
            let check = |x: u64, m: u64| {
                // SAFETY: BMI2 detected at runtime above.
                let (hp, hd) = unsafe { (pext_bmi2(x, m), pdep_bmi2(x, m)) };
                assert_eq!(hp, pext_soft(x, m), "pext x={x:#x} mask={m:#x}");
                assert_eq!(hd, pdep_soft(x, m), "pdep x={x:#x} mask={m:#x}");
            };
            for shift in [0u32, 52] {
                for x in 0u64..4096 {
                    for m in 0u64..4096 {
                        check(x << shift, m << shift);
                    }
                }
            }
            for code in 0..3u64.pow(13) {
                let (pos, neg) = ternary_masks(code, 13);
                check(pos, pos | neg);
            }
            eprintln!("pext dispatch in this build: hardware={}", pext_is_hardware());
        }
        #[cfg(not(target_arch = "x86_64"))]
        eprintln!("SKIP pext_pdep_hardware_matches_portable: not x86_64");
    }

    /// The dispatching [`pext`]/[`pdep`] agree with the portable forms on the 3^13 lattice
    /// whichever path the build selected.
    #[test]
    fn pext_pdep_dispatch_matches_portable() {
        for code in 0..3u64.pow(13) {
            let (pos, neg) = ternary_masks(code, 13);
            let s = pos | neg;
            assert_eq!(pext(pos, s), pext_soft(pos, s));
            let r = s.count_ones();
            let bits = code & low_bits(r);
            assert_eq!(pdep(bits, s), pdep_soft(bits, s));
        }
    }

    /// Collapse test, n = 13: every one of the 3^13 = 1 594 323 states ranks inside its
    /// class, unranks to itself, and the ranks of each class are exactly [0, C(13,r)·2^r).
    /// One failure kills it.
    #[test]
    fn signed_weight_collapse_3_pow_13() {
        const N13: u32 = 13;
        let total = 3u64.pow(N13);
        assert_eq!(total, 1_594_323);
        let sizes: Vec<u64> = (0..=N13).map(|r| signed_class_size(N13, r)).collect();
        assert_eq!(sizes.iter().sum::<u64>(), total, "class sizes partition 3^13");
        let mut seen: Vec<Vec<bool>> = sizes.iter().map(|&s| vec![false; s as usize]).collect();
        for code in 0..total {
            let (pos, neg) = ternary_masks(code, N13);
            let r = (pos | neg).count_ones();
            let k = signed_rank(pos, neg);
            assert_eq!(k, signed_rank_soft(pos, neg), "dispatch vs const path, code {code}");
            assert!(k < sizes[r as usize], "rank {k} outside class r={r}, code {code}");
            assert!(!seen[r as usize][k as usize], "rank {k} hit twice in class r={r}, code {code}");
            seen[r as usize][k as usize] = true;
            assert_eq!(signed_unrank(k, N13, r), (pos, neg), "unrank(rank) != x, code {code}");
            assert_eq!(signed_unrank_soft(k, N13, r), (pos, neg), "const unrank, code {code}");
        }
        for (r, class) in seen.iter().enumerate() {
            assert!(class.iter().all(|&b| b), "class r={r} not fully covered");
        }
    }

    /// Same test at every width 1 ≤ n ≤ 9 (3^9 = 19 683 each), so the n = 13 result is
    /// not a single-width accident.
    #[test]
    fn signed_weight_collapse_small_widths() {
        for n in 1..=9u32 {
            let total = 3u64.pow(n);
            let sizes: Vec<u64> = (0..=n).map(|r| signed_class_size(n, r)).collect();
            assert_eq!(sizes.iter().sum::<u64>(), total);
            let mut seen: Vec<Vec<bool>> = sizes.iter().map(|&s| vec![false; s as usize]).collect();
            for code in 0..total {
                let (pos, neg) = ternary_masks(code, n);
                let r = (pos | neg).count_ones();
                let k = signed_rank(pos, neg);
                assert!(k < sizes[r as usize]);
                assert!(!seen[r as usize][k as usize]);
                seen[r as usize][k as usize] = true;
                assert_eq!(signed_unrank(k, n, r), (pos, neg));
            }
            assert!(seen.iter().all(|c| c.iter().all(|&b| b)));
        }
    }

    /// The u64 ceiling is where the paper says it is.
    #[test]
    fn signed_max_n_is_the_u64_ceiling() {
        let s40: u64 = (0..=40).map(|r| signed_class_size(40, r)).sum();
        assert_eq!(s40, 3u64.pow(40));
        assert!(3u128.pow(41) > u64::MAX as u128);
        assert_eq!(SIGNED_MAX_N, 40);
    }
}
