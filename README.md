# IDM - Inlinable DMenu

## About:
`dmenu` like CLI tool, for using with pipes.

## Using:

Example (currently broken, `cd` can't read from pipe):
```bash
$ ls | idm | cd
```

```bash
$ cd $(ls | idm) # works, not accounting for files
```

```bash
$ alias lvim='vim $(ls | target/debug/idm)' # useful use
```

## TODO:
- [ ] Patch `cd` so it can read from a pipe.
- [ ] Everything.

## Alternatives:
Don't know and don't care!
