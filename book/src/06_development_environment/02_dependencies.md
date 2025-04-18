# Dependencies

If you want to add other Veryl projects to dependencies of your project, you can add them to `[dependencies]` section in `Veryl.toml`.
The left hand side of entry is the project name of the dependency, and the right hand side is the source and version.
`github` is a syntax sugger to refer a repository on GitHub. Instead of it, `git` with a full URL can be used.

```toml
[dependencies]
veryl_sample = {github = "veryl-lang/veryl_sample", version = "0.1.0"}

# This is as the same as above
veryl_sample = {git = "https://github.com/veryl-lang/veryl_sample", version = "0.1.0"}
```

By default, the namespace of the dependency is the same as the project name of the dependency.
If you want to specify a namespace through the left hand side, you should specify the project name through `project` field.

```toml
[dependencies]
veryl_sample_alt = {github = "veryl-lang/veryl_sample", project = "veryl_sample", version = "0.2.0"}
```

Inner projects in a repository can be used like below:

```toml
[dependencies]
inner_prj1 = {github = "veryl-lang/veryl_sample", version = "0.1.0"}
inner_prj2 = {github = "veryl-lang/veryl_sample", version = "0.1.0"}
inner_prj3 = {github = "veryl-lang/veryl_sample", version = "0.1.0"}
```

## Usage of dependency

After adding dependencies to `Veryl.toml`, you can use `module`, `interface` and `package` in the dependencies.
The following example uses `delay` module in the `veryl_sample` dependency.

```veryl
module ModuleA (
    i_clk: input  clock,
    i_rst: input  reset,
    i_d  : input  logic,
    o_d  : output logic,
) {
    inst u_delay: veryl_sample::delay (
        i_clk,
        i_rst,
        i_d  ,
        o_d  ,
    );
}
```

> Note: The result of play button in the above code is not exact because it doesn't use dependency resolution.
> Actually the module name becomes `veryl_sample_delay`

## Version Requirement

The `version` field of `[dependencies]` section shows version requirement.
For example, `version = "0.1.0"` means the latest version which has compatibility with `0.1.0`.
The compatibility is judged by [Semantic Versioning](https://semver.org/).
A version is constructed from the following three parts.

* `MAJOR` version when you make incompatible API changes
* `MINOR` version when you add functionality in a backwards compatible manner
* `PATCH` version when you make backwards compatible bug fixes

If `MAJOR` version is `0`, `MINOR` is interpreted as incompatible changes.

If there are `0.1.0` and `0.1.1` and `0.2.0`, `0.1.1` will be selected.
This is because

* `0.1.0` is compatible with `0.1.0`.
* `0.1.1` is compatible with `0.1.0`.
* `0.2.0` is not compatible with `0.1.0`.
* `0.1.1` is the latest in the compatible versions.

The `version` field allows other version requirement representation like `=0.1.0`.
Please see version requirement of Rust for detailed information: [Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-cratesio).

## Relative path dependency

For local development, dependency to a local file path is useful in some cases.
Relative path dependency can be specified like below:

```toml
[dependencies]
veryl_sample = {path = "../../veryl_sample"}
```

If there are relative path dependencies in a project, the project can't be published through `veryl publish`.

## Override by local path

Sometimes, using dependencies of locally modified version becomes necessary.
In the case, overriding dependencies by local path can be used like below:

```toml
[dependencies]
veryl_sample = {github = "veryl-lang/veryl_sample", version = "0.1.0", path = "../veryl_sample"}
```

This means that if there is `../veryl_sample`, it is used, and if not, it is pulled from the Git repository.
