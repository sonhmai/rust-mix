# Bloom Filter



Refs
- https://avi.im/blag/2024/sqlite-past-present-future/
- https://github.com/facebook/rocksdb/wiki/RocksDB-Bloom-Filter
- https://github.com/facebook/rocksdb/issues/4120


## Double hashing

Problem
- Bloom filters use multiple hash functions to map a key to multiple positions in a bit array. 
- If these hash functions produce similar or highly correlated results, the effectiveness of the filter is reduced, leading to a higher rate of false positives.

Solution: `double hashing`
- a technique used in Bloom filters to generate multiple, distinct hash values from a single input key.
- Double hashing helps to minimize such correlations by producing a more varied set of indices.



