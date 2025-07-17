# Finite lambda systems counter

This is a Rust program that counts the number of lambda systems on a set of $n$ elements using brute force (but with several clever optimizations taken from Mathoverflow user Peter Taylor in a post linked below).

A Lambda system (or Dynkin system) on a set $\Omega$ is a collection $\mathcal{D}$ of subsets of $\Omega$ that satisfies

(a) $\varnothing \in \mathcal{D}$,

(b) If $A \in \mathcal{D}$, then $\Omega \setminus A \in \mathcal{D}$,

(c) If $A_1, A_2, \ldots$ are pairwise disjoint sets in $\mathcal{D}$, then $\bigcup_{i=1}^\infty A_i \in \mathcal{D}$.  

That is, a Dynkin system $\mathcal D$ on $\Omega$ is a collection of subsets of $\Omega$ that is closed under complements in $\Omega$ and countable disjoint unions in $\mathcal D$.

<https://en.wikipedia.org/wiki/Dynkin_system>

Currently, only the first nine values are known ($n=0,...,8$):
<https://oeis.org/A380571>

This Rust implementation mostly follows the approach used by the Mathoverflow user Peter Taylor
in the following post:
<https://mathoverflow.net/questions/405736/what-is-the-number-of-finite-dynkin-systems?rq=1>

His Python implementation calculates up to $n=7$ in about 40 seconds on my machine. This Rust implementation calculates up to $n=7$ in about 0.15 seconds and $n=8$ in about 24 hours on my machine. I've also included a C implementation that performs similarly to the Rust implementation but slightly slower (in my limited e\Omegaperience, Rust is usually faster than C for basic mathematical operations -- presumably due to the language's strictness allowing for more aggressive compiler optimizations).
