// Count the number of Dynkin systems on sets of sizes 1, ..., n.
// A Dynkin system on a set X is a collection D of subsets of X that satisfies
//      (i) ∅ ∈ D,
//     (ii) if A ∈ D, then X-A ∈ D,
//    (iii) if A1, A2, ... ∈ D are disjoint, then ⋃ Ai ∈ D.
//
// This implementation mostly follows the approach used by the Mathoverflow user Peter Taylor
// in the post linked below:
// https://mathoverflow.net/questions/405736/what-is-the-number-of-finite-dynkin-systems?rq=1
//
// His python implementation calculates up to n=7 in about 40 seconds on my machine. This
// rust implementation calculates up to n=7 in about 0.15 seconds on my machine.

const MAX_N: usize = 7; // n=7 takes <1 second, n=8 takes ~24 hours
const MAX_SUBSETS: usize = 1 << MAX_N; // The cardinality of the powerset of a set of size MAX_N
const BITSET_WORDS: usize = MAX_SUBSETS / 64;

/// A bitwise operable representation of the powerset, where 1 represents a valid Dynkin system.
type Bitset = [u64; BITSET_WORDS];

/// Sets all bits in the bitset to zero.
fn bs_clear(bitset: &mut Bitset) {
    for word in bitset.iter_mut() {
        *word = 0;
    }
}

/// Copies the contents of one bitset to another.
fn bs_copy(destination: &mut Bitset, source: &Bitset) {
    destination.copy_from_slice(source);
}

/// Sets a specific bit in the bitset to 1.
fn bs_set(bitset: &mut Bitset, index: usize) {
    let word: usize = index >> 6;
    let bit: usize = index & 63;
    bitset[word] |= 1 << bit;
}

/// Gets the value of a specific bit in the bitset.
/// Returns true if the bit is set, false otherwise.
fn bs_get(bitset: &Bitset, index: usize) -> bool {
    let word: usize = index >> 6;
    let bit: usize = index & 63;
    ((bitset[word] >> bit) & 1) != 0
}

/// A queue to manage the elements being processed during the closure extension.
struct Queue {
    data: [usize; MAX_SUBSETS],
    len: usize,
}

impl Queue {
    fn clear(&mut self) {
        self.len = 0;
    }

    fn push(&mut self, x: usize) {
        self.data[self.len] = x;
        self.len += 1;
    }
}

/// Extends the closure by including a new element and updating the closure set.
/// Returns false if the extension is not valid (i.e., it conflicts with excluded elements).
/// Parameters:
/// - `omega`: the maximum element in the set (i.e., the size of the set minus one)
/// - `included`: the current set of included elements (the closure)
/// - `extension`: the new element to be included in the closure
/// - `excluded`: the current set of excluded elements (elements that cannot be included)
/// - `closure`: the current closure set to be updated
/// - `queue`: a queue to manage the elements being processed
/// Returns true if the extension is valid, false otherwise.
fn extend_closure(
    omega: usize,
    included: &Bitset,
    extension: usize,
    excluded: &Bitset,
    closure: &mut Bitset,
    queue: &mut Queue,
) -> bool {
    bs_copy(closure, included);
    bs_set(closure, extension);
    queue.clear();
    queue.push(extension);
    let mut queue_index: usize = 0;

    while queue_index < queue.len {
        let x: usize = queue.data[queue_index];
        queue_index += 1;
        let complement: usize = omega ^ x;
        if !bs_get(closure, complement) {
            if bs_get(excluded, complement) {
                return false;
            }
            bs_set(closure, complement);
            queue.push(complement);
        }

        for word in 0..BITSET_WORDS {
            let mut bits: u64 = closure[word];

            while bits != 0 {
                let lsb: usize = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                let y: usize = (word << 6) + lsb; // Calculates the index of the bit

                // If x ⋂ y = ∅, then we can add x ⋃ y to the closure
                if (x & y) == 0 {
                    let z: usize = x | y;
                    if !bs_get(closure, z) {
                        if bs_get(excluded, z) {
                            return false;
                        }
                        bs_set(closure, z);
                        queue.push(z);
                    }
                }
            }
        }
    }
    true
}

/// Recursively counts the number of valid subsets of the closure.
/// Given a set system F and a property P, the closure of F with respect to P is the smallest superset of F satisfying P.
/// Parameters:
/// - `omega`: the maximum element in the set (i.e., the size of the set minus one)
/// - `lower_bound`: the lower bound for the next element to consider
/// - `included`: the current set of included elements (the closure)
/// - `excluded`: the current set of excluded elements (elements that cannot be included)
/// - `queue`: a queue to manage the elements being processed
/// Returns the count of valid subsets of the closure.
fn inner(
    omega: usize,
    lower_bound: usize,
    included: &Bitset,
    excluded: &mut Bitset,
    _queue: &mut Queue,
    depth: usize,
) -> usize {
    let mut count: usize = 1;
    let limit: usize = (omega + 1) >> 1;

    // local queue for closure extension
    let mut queue_local: Queue = Queue {
        data: [0; MAX_SUBSETS],
        len: 0,
    };

    for x in lower_bound..limit {
        if bs_get(included, x) || bs_get(excluded, x) {
            continue;
        }

        // Inclusion branch
        let mut closure = [0u64; BITSET_WORDS];
        queue_local.clear();
        if extend_closure(omega, included, x, excluded, &mut closure, &mut queue_local) {
            let mut new_excluded = *excluded;
            count += inner(
                omega,
                x + 1,
                &closure,
                &mut new_excluded,
                &mut queue_local,
                depth,
            );
        }

        // Exclusion branch
        bs_set(excluded, x);
        bs_set(excluded, omega ^ x); // D-x
    }

    count
}

fn main() {
    // Calculate number of Dynkin systems for each set size
    for n in 0..=MAX_N {
        let omega: usize = if n > 0 { (1 << n) - 1 } else { 0 };

        // Initial included bitset: {∅, X}
        let mut included = [0u64; BITSET_WORDS];
        bs_clear(&mut included);
        bs_set(&mut included, 0);
        bs_set(&mut included, omega);

        // Prepare root_excluded array up to halfway
        let limit = (omega + 1) >> 1;
        let mut root_excluded = vec![[0u64; BITSET_WORDS]; limit];
        for m in 1..limit {
            root_excluded[m] = root_excluded[m - 1];
            bs_set(&mut root_excluded[m], m);
            bs_set(&mut root_excluded[m], omega ^ m);
        }

        // Parallel over m choices
        use rayon::prelude::*;
        let sum: usize = (1..limit)
            .into_par_iter()
            .map(|m| {
                if bs_get(&included, m) || bs_get(&root_excluded[m - 1], m) {
                    return 0;
                }
                // Build closure and count branch
                let mut closure = [0u64; BITSET_WORDS];
                let mut queue: Queue = Queue {
                    data: [0; MAX_SUBSETS],
                    len: 0,
                };
                let ex_here = &root_excluded[m - 1];
                let mut count: usize = 0;
                if extend_closure(omega, &included, m, ex_here, &mut closure, &mut queue) {
                    let mut new_exc = *ex_here;
                    bs_set(&mut new_exc, m);
                    bs_set(&mut new_exc, omega ^ m);
                    count += inner(omega, m + 1, &closure, &mut new_exc, &mut queue, 0);
                }
                count
            })
            .sum();

        let total: usize = sum + 1;
        println!("{} -> {}", n, total);
    }
}
