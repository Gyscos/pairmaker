# Description

**pairmaker** is a tool to generate a peer-programming schedule.
It takes a list of names, and outputs a JSON-formatted list of session, each session consisting of a list of pairs.

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
