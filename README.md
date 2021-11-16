# Parse and Write match files

Parse and write [match files](https://stereopipeline.readthedocs.io/en/latest/outputfiles.html?highlight=match#guide-to-output-files).

Inspired by [parse_match_file.py](https://github.com/NeoGeographyToolkit/StereoPipeline/blob/master/src/asp/Tools/parse_match_file.py) from [StereoPipeline](https://github.com/NeoGeographyToolkit/StereoPipeline).

## Examples

Generate a random match file for testing purpose.

```
cargo run --example random /tmp/match.bin
```

output

```
random ipmatch written to /tmp/match.bin
```

Parse it.

```
cargo run --example info /tmp/match.bin
```

output (YMMV since the generated file is random)

```
5 3
0.8180394 0.8835012 31 70 0.46054065 0.7019859 0.28444928 55 62 98 4 0.85953516 0.85953516 0.85953516 0.85953516
0.22759074 0.1743275 -47 -86 0.43778998 0.9623804 0.8732923 8 53 8 4 0.5195927 0.5195927 0.5195927 0.5195927
0.40164012 0.8818572 84 45 0.69460994 0.84166193 0.3195945 220 79 5 4 0.20246798 0.20246798 0.20246798 0.20246798
0.5079477 0.14797693 -54 -41 0.049428344 0.12339747 0.5082097 205 10 67 4 0.32223678 0.32223678 0.32223678 0.32223678
0.8215091 0.85995346 39 37 0.5086045 0.026840389 0.9858985 133 59 52 2 0.6899704 0.6899704
0.6373363 0.7324176 86 -91 0.044445693 0.088078916 0.32634288 122 48 62 2 0.85004526 0.85004526
0.93449944 0.3893805 -42 99 0.8485884 0.3959071 0.38207263 66 66 47 2 0.5259358 0.5259358
0.8023596 0.69243395 -61 56 0.24466741 0.25329328 0.49044204 100 27 31 1 0.92097896
```

Convert binary match file to text:

```
cargo run --example convert /tmp/match.bin /tmp/match.bin.txt
```

output:

```
/tmp/match.bin (binary) -> "/tmp/match.bin.txt" (text)
```

Convert text match file to binary:

```
/tmp/match.bin.txt (text) -> "/tmp/match.bin.out" (binary)
```

## Run tests

```
cargo test
```