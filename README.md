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
2 1
0.29287463 0.28693992 -46 76 0.07725787 0.8459307 0.9453458 169 45 91 4 0.4179544 0.4179544 0.4179544 0.4179544
0.29676783 0.9442217 60 -35 0.9081171 0.47889143 0.21851474 125 33 3 3 0.114198565 0.114198565 0.114198565
0.1608715 0.9641768 -31 50 0.41751534 0.7899522 0.08099252 112 50 67 4 0.27751178 0.27751178 0.27751178 0.27751178
```