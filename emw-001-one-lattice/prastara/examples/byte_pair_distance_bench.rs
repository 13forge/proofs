//! The §8 row 1 bench harness: a 64 KB pair-distance table vs register decode vs
//! multiply-shift decode for the L1 distance between two 5-digit balanced-ternary
//! bytes (3⁵ = 243 codes, 0..=242; 243..=255 unused). Self-contained, `std` only.
//!
//! Two numbers per candidate, because they answer different questions:
//!
//!   THROUGHPUT  independent ops. `black_box` on the inputs only; the running sum of
//!               results is consumed once per rep. Out-of-order execution overlaps
//!               iterations, so this is the cost per op in a streaming scan.
//!   LATENCY     dependent chain. Op i's inputs are derived from op i-1's result
//!               (`c = (c_i + d) mod 243`, same for q), so iterations cannot overlap.
//!               This is the cost per op when the next address depends on this distance.
//!               The chain link itself (add, compare, conditional subtract) is paid
//!               identically by every candidate.
//!
//! Each is reported WARM (nothing evicts the table) and COLD (64 MB sweep before every rep).
//!
//! Candidates:
//!   LUT   `PAIR_DIST[(c << 8) | q]`, one 64 KB table load. The table is built at
//!         compile time from DEC, so the three candidates share one definition.
//!   DEC   radix-3 digit extraction written as `/ 3`, `% 3` (Addressing I, naṣṭa with
//!         B = 3); what the compiler emits for a constant divisor is its business, this
//!         row measures it.
//!   MUL   explicit multiply-shift quotient `q = (v * 171) >> 9`, exact for v < 256
//!         (171/512 - 1/3 = 6.5e-4, times 255 is 0.166 < 1/3, the smallest gap to the
//!         next integer), written out so the division-free form is measured regardless
//!         of what the optimizer did to DEC.
//!
//! The byte stream comes from a splitmix64 generator seeded per size, so two runs on
//! one machine see the same pairs. No `criterion`, no `rand`: the harness has no deps.
//!
//! `cargo run --release --example byte_pair_distance_bench`
//! `cargo +stable run --release --example byte_pair_distance_bench`   (receipt toolchain)

use std::hint::black_box;
use std::time::Instant;

/// Interior codes of a 5-digit balanced-ternary byte.
const CODES: usize = 243;

/// Radix-3 digit extraction, division form: digit i = (b / 3ⁱ) mod 3, stored 0..=2,
/// shifted to −1..=+1. This is naṣṭa/uddiṣṭa with B = 3 (paper §3).
#[inline(always)]
const fn decode_div(b: u8) -> [i8; 5] {
    [
        (b % 3) as i8 - 1,
        (b / 3 % 3) as i8 - 1,
        (b / 9 % 3) as i8 - 1,
        (b / 27 % 3) as i8 - 1,
        (b / 81 % 3) as i8 - 1,
    ]
}

/// Division-free byte decode: four multiply-shift quotients, each exact for every v < 256.
/// Agreement with [`decode_div`] on the 243 interior codes is asserted in `main`.
#[inline(always)]
fn decode_mulshift(b: u8) -> [i8; 5] {
    let b = b as u32;
    let q1 = (b * 171) >> 9;
    let q2 = (q1 * 171) >> 9;
    let q3 = (q2 * 171) >> 9;
    let q4 = (q3 * 171) >> 9;
    [
        (b - 3 * q1) as i8 - 1,
        (q1 - 3 * q2) as i8 - 1,
        (q2 - 3 * q3) as i8 - 1,
        (q3 - 3 * q4) as i8 - 1,
        q4 as i8 - 1,
    ]
}

#[inline(always)]
const fn l1(a: [i8; 5], b: [i8; 5]) -> u32 {
    let mut dist = 0u32;
    let mut k = 0;
    while k < 5 {
        let d = a[k] - b[k];
        dist += d.unsigned_abs() as u32;
        k += 1;
    }
    dist
}

/// Per-pair L1 distance (max 10) over the full byte square, built at compile time by
/// decoding every byte through [`decode_div`]. Entries with a code ≥ 243 are never read.
static PAIR_DIST: [u8; 65536] = {
    let mut lut = [0u8; 65536];
    let mut a = 0usize;
    while a < 256 {
        let mut b = 0usize;
        while b < 256 {
            lut[(a << 8) | b] = l1(decode_div(a as u8), decode_div(b as u8)) as u8;
            b += 1;
        }
        a += 1;
    }
    lut
};

#[inline]
fn dist_by_lut(c: u8, q: u8) -> u32 {
    PAIR_DIST[((c as usize) << 8) | q as usize] as u32
}

#[inline]
fn dist_by_decode(c: u8, q: u8) -> u32 {
    l1(decode_div(c), decode_div(q))
}

#[inline]
fn dist_by_mulshift(c: u8, q: u8) -> u32 {
    l1(decode_mulshift(c), decode_mulshift(q))
}

/// splitmix64 (Steele, Lea & Flood 2014): the smallest deterministic generator that
/// passes the usual batteries. One u64 of state, no allocation.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `0..bound` by rejection, so the 243-code domain is not biased.
    fn below(&mut self, bound: u32) -> u32 {
        let zone = u32::MAX - (u32::MAX % bound);
        loop {
            let v = (self.next() >> 32) as u32;
            if v < zone {
                return v % bound;
            }
        }
    }
}

/// Interior codes only (`0..=242`); the unused rows of the table are never read.
fn random_pairs(n: usize, seed: u64) -> Vec<(u8, u8)> {
    let mut rng = SplitMix64(seed);
    (0..n)
        .map(|_| {
            let c = rng.below(CODES as u32) as u8;
            let q = rng.below(CODES as u32) as u8;
            (c, q)
        })
        .collect()
}

/// Bigger than any plausible L2/L3, touched at cache-line stride (64 B) so every prior
/// cache resident (the table included) is genuinely evicted.
const EVICT_BYTES: usize = 64 * 1024 * 1024;

fn evict_cache(evictor: &mut [u8]) -> u64 {
    let mut acc = 0u64;
    let mut i = 0usize;
    while i < evictor.len() {
        evictor[i] = evictor[i].wrapping_add(1);
        acc += evictor[i] as u64;
        i += 64;
    }
    black_box(acc)
}

#[derive(Clone, Copy)]
enum Kind {
    Throughput,
    Latency,
}

/// Throughput: independent ops, `black_box` on inputs only.
#[inline(always)]
fn throughput_pass(pairs: &[(u8, u8)], f: impl Fn(u8, u8) -> u32) -> u64 {
    let mut sum = 0u64;
    for &(c, q) in pairs {
        sum += f(black_box(c), black_box(q)) as u64;
    }
    sum
}

/// The chain link: next input depends on this result. `p <= 242`, `d <= 10`, so the
/// conditional subtract keeps the byte interior without a divide.
#[inline(always)]
fn link(p: u8, d: u32) -> u8 {
    let v = p as u32 + d;
    (if v >= CODES as u32 { v - CODES as u32 } else { v }) as u8
}

/// Latency: dependent chain, op i's inputs are derived from op i-1's result.
#[inline(always)]
fn latency_pass(pairs: &[(u8, u8)], f: impl Fn(u8, u8) -> u32) -> u64 {
    let mut sum = 0u64;
    let (mut c, mut q) = (black_box(0u8), black_box(0u8));
    for &(pc, pq) in pairs {
        let d = f(c, q);
        sum += d as u64;
        c = link(pc, d);
        q = link(pq, d);
    }
    black_box((c, q));
    sum
}

/// `reps` timed passes of one kind over `pairs`; the cache hierarchy is swept before
/// each rep when `cold`. Monomorphized per candidate so `f` inlines into the loop.
fn timed(
    kind: Kind,
    pairs: &[(u8, u8)],
    evictor: &mut [u8],
    reps: u32,
    cold: bool,
    f: impl Fn(u8, u8) -> u32,
) -> (u128, u64) {
    let mut total_ns = 0u128;
    let mut sum = 0u64;
    for _ in 0..reps {
        if cold {
            black_box(evict_cache(evictor));
        }
        let t = Instant::now();
        sum += match kind {
            Kind::Throughput => throughput_pass(pairs, &f),
            Kind::Latency => latency_pass(pairs, &f),
        };
        black_box(sum);
        total_ns += t.elapsed().as_nanos();
    }
    (total_ns, sum)
}

struct Row {
    lut: f64,
    dec: f64,
    mul: f64,
}

fn measure(kind: Kind, pairs: &[(u8, u8)], evictor: &mut [u8], reps: u32, cold: bool) -> Row {
    let (lut_ns, lut_sum) = timed(kind, pairs, evictor, reps, cold, dist_by_lut);
    let (dec_ns, dec_sum) = timed(kind, pairs, evictor, reps, cold, dist_by_decode);
    let (mul_ns, mul_sum) = timed(kind, pairs, evictor, reps, cold, dist_by_mulshift);
    assert_eq!(lut_sum, dec_sum, "LUT and decode must agree");
    assert_eq!(lut_sum, mul_sum, "LUT and multiply-shift must agree");
    let per_op = |ns: u128| ns as f64 / (pairs.len() as u128 * reps as u128) as f64;
    Row { lut: per_op(lut_ns), dec: per_op(dec_ns), mul: per_op(mul_ns) }
}

fn print_row(kind: &str, label: &str, n: usize, warm: &Row, cold: &Row) {
    println!(
        "{kind:<10} {label:>9} n={n:>8}  WARM LUT={:>6.3} DEC={:>6.3} MUL={:>6.3} DEC/LUT={:>5.2} MUL/LUT={:>5.2}  |  \
         COLD LUT={:>6.3} DEC={:>6.3} MUL={:>6.3} DEC/LUT={:>5.2} MUL/LUT={:>5.2}",
        warm.lut, warm.dec, warm.mul, warm.dec / warm.lut, warm.mul / warm.lut,
        cold.lut, cold.dec, cold.mul, cold.dec / cold.lut, cold.mul / cold.lut,
    );
}

fn main() {
    // Agreement over the whole interior domain before any timing. Codes 243..=255 are
    // outside 3⁵: the division form wraps its top digit there (243 / 81 % 3 = 0) and the
    // multiply-shift form does not, so they are excluded here as they are everywhere else.
    for b in 0..CODES as u8 {
        assert_eq!(decode_mulshift(b), decode_div(b), "mulshift decode {b}");
    }
    for c in 0..CODES as u8 {
        for q in 0..CODES as u8 {
            let l = dist_by_lut(c, q);
            assert_eq!(l, dist_by_decode(c, q));
            assert_eq!(l, dist_by_mulshift(c, q));
        }
    }

    const REPS: u32 = 6;
    println!(
        "PAIR_DIST is {} bytes. Evictor sweep is {} MB per cold rep. ns/op, {REPS} reps.\n\
         THROUGHPUT: independent ops, black_box on inputs only.  LATENCY: dependent chain \
         (next inputs derived from this result).\n",
        std::mem::size_of_val(&PAIR_DIST),
        EVICT_BYTES / (1024 * 1024),
    );

    let mut evictor = vec![0u8; EVICT_BYTES];

    let sizes: &[(&str, usize)] = &[
        ("tiny", 64),
        ("small", 1_024),
        ("l1-edge", 16_384),   // ~32KB working set, at a typical L1d boundary
        ("l1-full", 24_576),   // ~48KB working set, at a larger L1d boundary
        ("over-lut", 40_000),  // working set alone exceeds the 64KB table footprint
        ("l2-ish", 131_072),
        ("large", 1_048_576),
    ];

    for (label, n) in sizes {
        let pairs = random_pairs(*n, 0xC0FFEE ^ *n as u64);
        let tp_warm = measure(Kind::Throughput, &pairs, &mut evictor, REPS, false);
        let tp_cold = measure(Kind::Throughput, &pairs, &mut evictor, REPS, true);
        print_row("THROUGHPUT", label, *n, &tp_warm, &tp_cold);
        let lat_warm = measure(Kind::Latency, &pairs, &mut evictor, REPS, false);
        let lat_cold = measure(Kind::Latency, &pairs, &mut evictor, REPS, true);
        print_row("LATENCY", label, *n, &lat_warm, &lat_cold);
    }
}
