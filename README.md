# InsPEctor

This project is a simple proof of concept to explain various concepts among the domain of reverse engineering and  will be developed in several blog posts in [my blog](https://nicolo.dev). Every concept will be introduced by an  article, and every new feature will be based on a branch that will contain the common core and the new feature.

The idea behind InsPEctor is to have a Rust-based binary analysis software for which we could say some information  (e.g. data flow analysis, CFG, or your own analysis) about PE executable files targeting Intel x86_64 architecture. Among all the articles, we will use the same binary: the goal would be for you to write the software analysis, and extract some precious information about how it works.

| Date       | Title                                                        | Concept                                                                                                                                                                      | Branch            |
|------------|--------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------|
| 2025-01-XX | Obtaining instructions: linear sweep and recursive traversal | Explains the first problems of retriving instructions for a given binary, highlighting the two different algorithms for disassembling: linear sweep and recursive traversal. | `1-disassembling` |

**Warning**: this code represents an experiment and does not represent any valid production-ready code. Be aware that the code may not work for all the cases and for the all binaries.


