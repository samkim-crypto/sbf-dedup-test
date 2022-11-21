## Description

A small program to test the Solana BPF compute units for deduping a vector of
elements. The program can be used to compare the compute units when finding
duplicates in a vector of `u16` elements via the naive O(n^2) algorithm and a 
O(n) algorithm that relies on hash-sets. 

```bash
$ cargo test-sbf
````

The results show that the naive algorithm costs less compute units when 
deduping vectors of size at most 76.

