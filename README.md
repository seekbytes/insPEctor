[See the most current branch](https://github.com/seekbytes/insPEctor/tree/1_disassembler)
# InsPEctor

This project is a simple proof of concept to explain various concepts among the domain of reverse engineering and  will be developed in several blog posts in [my blog](https://nicolo.dev). Every concept will be introduced by an  article, and every new feature will be based on a branch that will contain the common core and the new feature.

The idea behind InsPEctor is to have a Rust-based binary analysis software for which we could say some information  (e.g. data flow analysis, CFG, or your own analysis) about PE executable files targeting Intel x86_64 architecture. Among all the articles, we will use the same binary: the goal would be for you to write the various analysis, and extract some precious information about how it works and try to guess what it is.

| Date       | Title                                                        | Branch                                                                                                                                                                      | Concept            |
|------------|--------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------|
| 2025-02-03 | [Obtaining instructions: linear sweep and recursive traversal](https://nicolo.dev/en/blog/disassembling-binary-linear-recursive/) | [`1-disassembler`](https://github.com/seekbytes/insPEctor/tree/1_disassembler) | Explains the first problems of retriving instructions for a given binary, highlighting the two different algorithms for disassembling: linear sweep and recursive traversal. | 

**Warning**: this code represents an experiment and does not represent any valid production-ready code. Be aware that the code may not work for all the cases and for the all binaries.


