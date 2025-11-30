# random-x
Berabones Rust template setup for [RandomX](https://github.com/tevador/RandomX) bindings.
<br>
Before execution, pull the submodules by executing:
```shell
$ make bootstrap
```
Example (unsafe) usage is implemented as a unit test and can be executed using:
```shell
$ make test
```

## Template generation
You can generate your own project based on this template using
[`cargo-generate`](https://github.com/cargo-generate/cargo-generate#installation):
```shell
$ cargo generate --git https://github.com/aggstam/random-x --name {project-name}
```
