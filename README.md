# An extremely fast grep clone written in rust

Todos

- [x] Add Regex

Usage

<pre>
  <code>
$ cargo run --release -- --search YourSearchTerm --directory /
    
    error: the following required arguments were not provided:
    --search <SEARCH>
    --directory <DIRECTORY>
  
    Usage: fuzzyrs --search <SEARCH> --directory <DIRECTORY>
    
    For more information, try '--help'.
  </code>
</pre>

grep-rs vs grep
| Grep-rs | Grep | Difference |
| ------------- | ------------- | ------------- |
| 2.472s | 397.34ms | 144.609% |
