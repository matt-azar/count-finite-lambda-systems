# Count finite lambda systems

This is a Rust program that counts the number of finite lambda systems on a set of $n$ elements.

A Lambda system (or Dynkin system) on a set $X$ is a collection $\mathcal{D}$ of subsets of $X$ that satisfies
(a) $\varnothing \in \mathcal{D}$.  
(b) If $A \in \mathcal{D}$, then $X \setminus A \in \mathcal{D}$.  
(c) If $A_1, A_2, \ldots$ are disjoint, then $\bigcup_{i=1}^\infty A_i \in \mathcal{D}$.  

This implementation mostly follows the approach used by the Mathoverflow user Peter Taylor
in the post linked below:
https://mathoverflow.net/questions/405736/what-is-the-number-of-finite-dynkin-systems?rq=1

His Python implementation calculates up to $n=7$ in about 40 seconds on my machine. This
Rust implementation calculates up to $n=7$ in about 0.15 seconds on my machine. I've also included a C implementation that performs similarly to the Rust implementation.
