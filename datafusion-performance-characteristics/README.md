# Testing DataFusion performance characteristics

- What kind of queries benefit from parallelism?
- How should we organize our data to benefit from parallelism?
- When to download the parquet and run local queries vs trying the 'object store' route?

## Resources

- https://db.in.tum.de/~leis/papers/morsels.pdf
- https://datafusion.apache.org
- https://dl.acm.org/doi/pdf/10.1145/93605.98720
- https://substrait.io/relations/physical_relations/#exchange-types

### Volcano-style parallelism

	$ git log --reverse -S ExchangeKind
	commit 0f83ffc448a4d7fb4297148f653e267a847d769a
	Author: Ruihang Xia <waynestxia@gmail.com>
	Date:   Sun Dec 17 19:55:29 2023 +0800

	    feat: implement Repartition plan in substrait (#8526)

	    * feat: implement Repartition plan in substrait

	    Signed-off-by: Ruihang Xia <waynestxia@gmail.com>

	    * use substrait_err macro

	    Signed-off-by: Ruihang Xia <waynestxia@gmail.com>

	    ---------

	    Signed-off-by: Ruihang Xia <waynestxia@gmail.com>


Config:
/// When set to true, the physical plan optimizer will try to add round robin
/// repartitioning to increase parallelism to leverage more CPU cores
pub enable_round_robin_repartition: bool, default = true


### Substrait

The “Exchange Types” in Substrait

On the page you linked—“Physical Relations → Exchange Types”—Substrait defines the parameters that an engine can use to implement an Exchange operator. This operator typically handles data redistribution among execution nodes (like shuffles, broadcasts, or partitioned data transfers), much as many distributed query engines do in their physical plans.

