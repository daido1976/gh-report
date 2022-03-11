# gh-report

See. https://github.com/daido1976/github-report

## Installation

```sh
$ gh extension install daido1976/gh-report
```

## Usage

```sh
# require a scope for `read:user` to get the contributions collection.
$ gh auth refresh --scopes read:user
$ gh report
```

## Release

```sh
$ git tag <version(e.g.`v0.1`)>
$ git push --tag
```
