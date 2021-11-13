# Parse and Write match files

Parse and write [match files](https://stereopipeline.readthedocs.io/en/latest/outputfiles.html?highlight=match#guide-to-output-files).

Inspired by [parse_match_file.py](https://github.com/NeoGeographyToolkit/StereoPipeline/blob/master/src/asp/Tools/parse_match_file.py) from [StereoPipeline](https://github.com/NeoGeographyToolkit/StereoPipeline).

## Tools

Generate a random match file for testing purpose.

```bash
cargo run --example random /tmp/match.bin
```

> random ipmatch written to /tmp/match.bin

Parse it.

```bash
cargo run --example info /tmp/match.bin
```

```bash
IPMatch { image_1: [IPRecord { x: 0.7807456, y: 0.202268, xi: 1153703064, yi: 1240783834, orientation: 0.17374378, scale: 0.92560816, interest: 0.09949291, polarity: 240, octave: 2662935178, scale_lvl: 3873043711, ndesc: 2, desc: [0.33003032, 0.33003032] }, IPRecord { x: 0.11048275, y: 0.6147897, xi: 1044064830, yi: 1739617807, orientation: 0.08849937, scale: 0.07429397, interest: 0.882921, polarity: 71, octave: 3131877141, scale_lvl: 3658744998, ndesc: 2, desc: [0.99545664, 0.99545664] }, IPRecord { x: 0.6544174, y: 0.059506536, xi: -1460612552, yi: -660882239, orientation: 0.19595903, scale: 0.7120783, interest: 0.66062784, polarity: 198, octave: 2386742392, scale_lvl: 1668542840, ndesc: 3, desc: [0.22081828, 0.22081828, 0.22081828] }], image_2: [IPRecord { x: 0.12835616, y: 0.80815154, xi: -1646998533, yi: -1441267559, orientation: 0.11998427, scale: 0.6288643, interest: 0.70630723, polarity: 207, octave: 2462366988, scale_lvl: 2336194499, ndesc: 1, desc: [0.14456236] }, IPRecord { x: 0.31592888, y: 0.056107163, xi: 21791764, yi: -1245671424, orientation: 0.8548769, scale: 0.43577862, interest: 0.34628838, polarity: 75, octave: 4272377701, scale_lvl: 4021854306, ndesc: 4, desc: [0.5149509, 0.5149509, 0.5149509, 0.5149509] }, IPRecord { x: 0.36182457, y: 0.25470072, xi: -835106566, yi: -802092706, orientation: 0.9418457, scale: 0.95545626, interest: 0.26207358, polarity: 210, octave: 1628879912, scale_lvl: 4227181730, ndesc: 1, desc: [0.25326836] }, IPRecord { x: 0.1633259, y: 0.35900003, xi: -1140103329, yi: 1415270010, orientation: 0.16165882, scale: 0.79697996, interest: 0.37271672, polarity: 97, octave: 2272887688, scale_lvl: 1123353560, ndesc: 3, desc: [0.33974224, 0.33974224, 0.33974224] }, IPRecord { x: 0.6661362, y: 0.53180015, xi: -623945205, yi: 217075657, orientation: 0.58141017, scale: 0.6567431, interest: 0.41913766, polarity: 53, octave: 1219239249, scale_lvl: 2681313576, ndesc: 3, desc: [0.5145098, 0.5145098, 0.5145098] }] }
```