GibleFS
=======

A toy project that maps a Git repository to a virtual filesystem which can be
used to access the repository at any given commit. In another words, `/<commit
hash>` gives you the view of the repository at that commit.

For example, `/deadbeefdeadbeefdeadbeefdeadbeefdeadbeef/foobar` gives you the
content of `foobar` at commit `deadbeefdeadbeefdeadbeefdeadbeefdeadbeef`.

Usage
-----

```
$ cargo run -- <path to git repository> <path to mount>
```

License
-------
MIT
