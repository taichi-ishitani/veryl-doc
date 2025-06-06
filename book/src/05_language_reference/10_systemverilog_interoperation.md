# SystemVerilog Interoperation

If you want to access to items of SystemVerilog, `$sv` namespace can be used.
For example, "ModuleA" in SystemVerilog source code can be accessed by `$sv::ModuleA`.
Veryl don't check the existence of the items.

Veryl compiler emits the path name as is excluding `$sv::`.
Therefore, symbols in other HDLs like Verilog/VHDL can be refered through `$sv::` too.
It depeneds on implementation (e.g. simulator, synthesizer) whether each symbols can be resolved.

```veryl,playground
module ModuleA {
    let _a: logic = $sv::PackageA::ParamA;

    inst b: $sv::ModuleB;
    inst c: $sv::InterfaceC;
}
```

To access some identifiers which are used as Veryl's keywords, raw identifier can be used.


```veryl,playground
module ModuleA (
    i_clk: input clock,
) {
    inst a: $sv::ModuleA (
        // clock: i_clk
        // ^ this is syntax error because `clock` is a keyword
        // Instead of it, `r#clock` can be used
        r#clock: i_clk,
    );
}
```
