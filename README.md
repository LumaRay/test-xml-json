# Benchmarking different XML and JSON parsers

The testing has been performed on Windows 7x64 host with Ubuntu 18.04 x64 virtual machine with 8Gb RAM and 4 cores, Intel-VT enabled.

Host:
- CPU: Intel Core i7-4790 3.6 GHz 4 cores / 8 threads
- RAM: 32 Gb

AVX2 is used whenever possible.

Full release optimizations.

Parsing is used over files:

```
mondial-3.0-mini.xml (1'174'507 bytes)
mondial-3.0-mini.json (1'174'212 bytes)
```

1'000 cycles per test.

All tests use 1 core.

Test results are given in milliseconds of total execution time, in descending order, worst to best.

You can find links to the algorithms' web pages in the first comments of the corresponding main source files.

## Test Results

| Language | Package  | Format | DOM | Time, ms  |
|:-------:|:---------:|:---------:|:---------:|:---------:|
|  rust  | serde-xml-rs  | xml |  |  |
|  rust  | xml-rs  | xml |  | 70167 |
|  rust  | xml-oxide  | xml |  | 43509 |
|  rust  | dummy-xml  | xml | + | 26170 |
|  rust  | serde-json  | json |  | 18461 |
|  rust  | simd-json  | json |  | 14503 |
|  rust  | roxmltree  | xml | + | 10040 |
|  rust  | json  | json |  | 7692 |
|  rust  | vtd-xml  | xml | + | 7519 |
|  rust  | rapid-xml  | xml |  | 3946 |
|  rust  | quick-xml  | xml |  | 2507 |
