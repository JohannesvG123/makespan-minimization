
# Introduction

This is a Rust framework for the makespan minimization problem on identical parallel machines. 
It uses parallelization and local search heuristics to achieve high-quality results in practice. 
The framework can be configured to fit one's needs.
<hr/>

# Building

Simply run `cargo build` to build the executable.

# Usage

The framework needs a text file representing a problem instance as input.
This file needs to have the following format:

```
p p_cmax n m
$p_1$ $p_2$ $p_3$ $p_4$ ... $p_n$ 0
```

Further, all command line arguments to configure the frameworks behaviour can be shown via this command:

```
cargo run --package makespan-minimization --bin makespan-minimization -- --help
```

Further references:

**[Bachelor Thesis (explaining the used scheduling approach in detail)](todo)**