/*
    C version for speed comparison. The Rust version is slightly faster on
    average.

    Compile with
    gcc -Ofast -march=native -fopenmp dynkin.c -o dynkin
*/

#include <omp.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_N 7
#define MAX_SUBSETS (1 << MAX_N)
#define BITSET_WORDS (MAX_SUBSETS / 64)

// Bitset structure to hold the bits for subsets
typedef struct {
    uint64_t bits[BITSET_WORDS];
} Bitset;

// Clear the bitset by setting all bits to 0.
static inline void bs_clear(Bitset *bs) {
    memset(bs->bits, 0, sizeof(bs->bits));
}

// Copy the contents of one bitset to another.
static inline void bs_copy(Bitset *dst, const Bitset *src) {
    memcpy(dst->bits, src->bits, sizeof(dst->bits));
}

// Set a specific bit in the bitset.
static inline void bs_set(Bitset *bs, uint32_t idx) {
    bs->bits[idx >> 6] |= (uint64_t)1 << (idx & 63);
}

// Get the value of a specific bit in the bitset.
static inline int bs_get(const Bitset *bs, uint32_t idx) {
    return (bs->bits[idx >> 6] >> (idx & 63)) & 1;
}

// Extend closure: returns 1 on success, 0 on contradiction
static int extend_closure(uint32_t omega, const Bitset *included,
                          uint32_t extension, const Bitset *excluded,
                          Bitset *closure) {
    bs_copy(closure, included);
    bs_set(closure, extension);

    uint32_t queue[MAX_SUBSETS];
    int qs = 0, qe = 0;
    queue[qe++] = extension;

    while (qs < qe) {
        uint32_t x = queue[qs++];
        uint32_t comp = omega ^ x;
        if (!bs_get(closure, comp)) {
            if (bs_get(excluded, comp))
                return 0;
            bs_set(closure, comp);
            queue[qe++] = comp;
        }

        for (int w = 0; w < BITSET_WORDS; ++w) {
            uint64_t word = closure->bits[w];
            while (word) {
                uint32_t bit = __builtin_ctzll(word);
                word &= word - 1;
                uint32_t y = (w << 6) + bit;
                if ((x & y) == 0) {
                    uint32_t z = x | y;
                    if (!bs_get(closure, z)) {
                        if (bs_get(excluded, z))
                            return 0;
                        bs_set(closure, z);
                        queue[qe++] = z;
                    }
                }
            }
        }
    }
    return 1;
}

// Serial inner enumeration
static size_t inner(uint32_t omega, uint32_t lb, Bitset *included,
                    Bitset *excluded) {
    size_t count = 1;
    uint32_t limit = (omega + 1) >> 1;

    Bitset new_inc, new_exc, closure;
    for (uint32_t m = lb; m < limit; ++m) {
        if (bs_get(included, m) || bs_get(excluded, m))
            continue;
        if (extend_closure(omega, included, m, excluded, &closure)) {
            bs_copy(&new_inc, &closure);
            bs_copy(&new_exc, excluded);
            count += inner(omega, m + 1, &new_inc, &new_exc);
        }
        bs_set(excluded, m);
        bs_set(excluded, omega ^ m);
    }
    return count;
}

int main(void) {
    omp_set_num_threads(omp_get_max_threads());

    for (uint32_t n = 0; n <= MAX_N; ++n) {
        uint32_t omega = (n > 0) ? ((1u << n) - 1) : 0;

        // base family always has ∅ and the whole set
        Bitset included;
        bs_clear(&included);
        bs_set(&included, 0);
        bs_set(&included, omega);

        // how many non-complementary subsets we have to consider
        uint32_t limit = (omega + 1) >> 1;

        if (limit == 0) {
            // only the trivial Dynkin system
            printf("%u %zu\n", n, (size_t)1);
            continue;
        }

        // precompute, in serial, the "excluded" bitset up through each m
        Bitset root_excluded[limit];
        bs_clear(&root_excluded[0]);
        for (uint32_t m = 1; m < limit; ++m) {
            bs_copy(&root_excluded[m], &root_excluded[m - 1]);
            bs_set(&root_excluded[m], m);
            bs_set(&root_excluded[m], omega ^ m);
        }

        size_t total = 0;

// parallelize the top‐level loop, each iteration sees exactly
// the same `included` and the corresponding `excluded` at m−1
#pragma omp parallel for schedule(dynamic) \
                reduction(+ : total) firstprivate(included)
        for (uint32_t m = 1; m < limit; ++m) {
            const Bitset *ex_here = &root_excluded[m - 1];

            // skip sets already forced‐in or forced‐out
            if (bs_get(&included, m) || bs_get(ex_here, m))
                continue;

            Bitset closure;
            if (extend_closure(omega, &included, m, ex_here, &closure)) {
                // build this branch’s excluded set
                Bitset new_exc;
                bs_copy(&new_exc, ex_here);
                bs_set(&new_exc, m);
                bs_set(&new_exc, omega ^ m);

                total += inner(omega, m + 1, &closure, &new_exc);
            }
        }

        // account for the “no more subsets” choice
        total += 1;

        printf("%u %zu\n", n, total);
    }

    return 0;
}
