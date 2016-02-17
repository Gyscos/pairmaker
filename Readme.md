# Description

**pairmaker** is a tool to generate a peer-programming schedule.
It takes a list of names, and outputs a JSON-formatted list of session, each session consisting of a list of pairs.

# Usage

```
$ ./pairmaker -h
pairmaker 0.1.0
Alexandre Bury <alexandre.bury@gmail.com>
A utility to generate pairs for peer-programming schedules.

USAGE:
	pairmaker [FLAGS] [OPTIONS] <NAMES>... [--]

FLAGS:
    -m, --mirror     Add a mirror session after each pair
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --sessions <N>    Number of sessions to print (defaults to number of names minus 1)

ARGS:
    NAMES...    List of names
```

# Example

```json
$ ./pairmaker Alice Bob Carl Dave | jq
[
  [
    [
      "Alice",
      "Bob"
    ],
    [
      "Carl",
      "Dave"
    ]
  ],
  [
    [
      "Alice",
      "Carl"
    ],
    [
      "Dave",
      "Bob"
    ]
  ],
  [
    [
      "Alice",
      "Dave"
    ],
    [
      "Bob",
      "Carl"
    ]
  ]
]
```
