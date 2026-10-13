//! prastara — reference implementation for WP-003 (draft).
//! Three table-free, zero-allocation state addressings from Sanskrit prosody.
//! Encoding: guru (G, long) = 1, laghu (L, short) = 0. Position 0 = first syllable = LSB.
#![cfg_attr(not(test), no_std)]

/// Naṣṭa: 1-indexed row k of the n-syllable prastāra -> guru bitmask.
/// Literal rule: k odd -> L, k <- (k+1)/2 ; k even -> G, k <- k/2.
pub fn nashta(mut k: u64, n: u32) -> u64 {
    let mut mask = 0u64;
    for i in 0..n {
        if k & 1 == 1 {
            k = (k + 1) / 2;
        } else {
            mask |= 1 << i;
            k /= 2;
        }
    }
    mask
}

/// Uddiṣṭa: guru bitmask -> 1-indexed row.
/// Literal rule: write 1,2,4,... over syllables; sum values over gurus; add 1.
pub fn uddishta(mask: u64, n: u32) -> u64 {
    let mut k = 1u64;
    for i in 0..n {
        if (mask >> i) & 1 == 1 {
            k += 1 << i;
        }
    }
    k
}

/// Saṅkhyā: 2^n by Piṅgala's halve/square, subtract-one/double rule. O(log n).
pub fn sankhya(n: u32) -> u64 {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        let h = sankhya(n / 2);
        h * h
    } else {
        2 * sankhya(n - 1)
    }
}

/// C(n, r), exact in u64 for the ranges used here. No table.
pub fn binom(n: u32, r: u32) -> u64 {
    if r > n {
        return 0;
    }
    let r = r.min(n - r) as u64;
    let n = n as u64;
    let mut acc = 1u64;
    for i in 0..r {
        acc = acc * (n - i) / (i + 1);
    }
    acc
}

/// Sūci-prastāra (Virahāṅka, per Shah): column k is the running partial sum of column k-1,
/// with the last entry dropped. Implemented as k in-place inclusive scans over a caller buffer.
/// Add-only: no multiply, no divide. After the call, buf[i] = C(i + k, k) for i in 0..len
/// (equivalently: entry i of column k in the needle layout, before the shift).
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

/// Combinadic unrank: 0-indexed rank k (colex) -> bitmask with exactly r gurus among n.
pub fn combinadic_unrank(mut k: u64, n: u32, r: u32) -> u64 {
    let mut mask = 0u64;
    let mut c = n;
    for j in (1..=r).rev() {
        c -= 1;
        while binom(c, j) > k {
            c -= 1;
        }
        mask |= 1 << c;
        k -= binom(c, j);
    }
    mask
}

/// Combinadic rank: inverse of combinadic_unrank.
pub fn combinadic_rank(mask: u64) -> u64 {
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

/// Fibonacci F(n) by fast doubling (saṅkhyā-style). F(0)=0, F(1)=1. Valid n <= 93.
pub fn fib(n: u32) -> u64 {
    fn fd(n: u32) -> (u64, u64) {
        if n == 0 {
            return (0, 1);
        }
        let (a, b) = fd(n / 2);
        let c = a * (2 * b - a);
        let d = a * a + b * b;
        if n % 2 == 0 { (c, d) } else { (d, c + d) }
    }
    fd(n).0
}

/// Mātrā count: number of L/G sequences of total duration n (L=1, G=2) = F(n+1).
pub fn matra_count(n: u32) -> u64 {
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
    fn row_with_r_gurus_counts_binomial() {
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
        for k in 0..=20 {
            suci_scan(&mut buf, k);
            for i in 0..buf.len() as u32 {
                assert_eq!(buf[i as usize], binom(i + k, k));
            }
        }
    }

    #[test]
    fn shallow_diagonal_is_matra_count() {
        for n in 0..=80 {
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
}
