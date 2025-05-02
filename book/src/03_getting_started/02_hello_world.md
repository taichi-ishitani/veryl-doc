# Hello, World!

## Create Project

At first, a new Veryl project can be created by:

```
veryl new hello
```

After the command, the following directory and file will be created.
If `git` command is available, the directory is initialized as Git repository and the default `.gitignore` are added.

```
$ veryl new hello
[INFO ]      Created "hello" project
$ cd hello
$ tree
.
├── src
└── Veryl.toml

1 directory, 1 file
```

`Veryl.toml` is the project configuration.

```toml
[project]
name = "hello"
version = "0.1.0"
[build]
source = "src"
target = {type = "directory", path = "target"}
```

The description of all configuration is [here](../06_development_environment/01_project_configuration.md).

## Write Code

You can add source codes at an arbitrary position in the project directory.
This is because Veryl project can be independent or integrated to other SystemVerilog project.
The extension of Veryl's source codes is `.veryl`.

For example, put the following code to `src/hello.veryl`.

```veryl,playground
module ModuleA {
    initial {
        $display("Hello, world!");
    }
}
```

```
$ tree
.
├── src
│   └── hello.veryl
└── Veryl.toml

1 directory, 2 files
```

> Note: Some source codes in the book have play button "▶" which will be appeared when mouse cursor is hovered at the code.
> If you click the button, the transpiled SystemVerilog code will appear. Please try to click the button of `module ModuleA` code.

## Build Code

You can generate a SystemVerilog code by `veryl build`.

```
$ veryl build
[INFO ]   Processing file ([path to hello]/src/hello.veryl)
[INFO ]       Output filelist ([path to hello]/hello.f)
$ tree
.
├── dependencies
├── hello.f
├── src
│   └── hello.veryl
├── target
│   ├── hello.sv
│   └── hello.sv.map
├── Veryl.lock
└── Veryl.toml

3 directories, 6 files
```

By default, SystemVerilog code will be generated at the same directory as Veryl code.
The generated code is `src/hello.sv`.

```verilog
module hello_ModuleA;
    initial begin
        $display("Hello, world!");
    end
endmodule
//# sourceMappingURL=hello.sv.map
```

Additionally, `hello.f` which is the filelist of generated codes will be generated.
You can use it for SystemVerilog compiler.
The following example is to use [Verilator](https://www.veripool.org/verilator/).

```
$ verilator --binary -f hello.f
```

## Clean up the Generated Code

The generated code can be cleaned up by `veryl clean`.

```
$ veryl clean
[INFO ]   Removing file ([path to hello]/src/hello.sv)
[INFO ]   Removing file ([path to hello]/src/hello.sv.map)
[INFO ]   Removing dir  ([path to hello]/dependencies)
[INFO ]   Removing file ([path to hello]/hello.f)
```
