# Testing DataFusion performance characteristics

- What kind of queries benefit from parallelism?
- How should we organize our data to benefit from parallelism?
- When to download the parquet and run local queries vs trying the 'object store' route?

## Resources

- https://db.in.tum.de/~leis/papers/morsels.pdf
- https://datafusion.apache.org
- https://dl.acm.org/doi/pdf/10.1145/93605.98720


### Volcano-style parallelism

