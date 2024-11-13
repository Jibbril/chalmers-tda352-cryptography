# Lecture 2

### Pseudo Random Generators (PRG)
A PSG is a deterministic function $`PRG: \{0,1\}^n -> \{0,1\}^l`$ such that:
- $PRG(\cdot)$ is efficiently computable
- $l > n$
- $`PRG(x) | x <- \$ \{0,1\}^n`$ is pseudo-random (i.e. an efficient adversary cannot separate $PRG(x)$ from any $`y <- \$ \{0,1\}^l`$

There are no mathematical proofs to verify a PRG, only tests and formal security games. These games generally consist of some scenario where a challenger will either generate a key from the "real" distribution $`k \leftarrow \{0,1\}^l`$, or generate a key by using $PRG(\cdot)$. The adversary then needs to try and guess whether the provided key was from the real distribution or from the PRG. If the adversary can consistently guess correctly then the candidate algorithm is not a PRG. More formally we can say that a PRG is secure if 

$$
Adv = |Pr[Adversary wins] - \frac{1}{2} | < negl(n)
$$

### The negligible function
The negligible function $negl(x): \mathbb{N} \rightarrow \mathbb{R}_{\geq 0}$



