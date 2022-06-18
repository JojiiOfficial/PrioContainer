# PrioContainer
Rust library to find n biggest/smallest items within a set of Items without storing more than n items in `O(n log n)`. This can be helpful if you 
have an iterator/stream over many items but don't want to collect and sort them in `O(m log m)` 
