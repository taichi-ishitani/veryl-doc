# Generics

Generics can define parameterized items which can't achieved by parameter override.
The following items support generics:

* function
* module
* interface
* package
* struct
* union

Each generic definition has generic parameters (often an uppercase letter is used like `T`) which can be placed as identifier or expression in the definition.
Generic parameters are declarated after item's identifier with `::<>`.

Each generic parameter should have generic bound after colon like `T: TypeName`.
Generic bound represents what value can be passed to the generic parameter.
The available generic bounds are below:

* `type` : means arbitrary type can be passed
* `inst: X` : instance of `X`
* named prototype, user defined data type or fixed data type

Named prototype is a special generic bound. See [Prototype](14_generics/02_prototype.md) for details.

At the usage of generics, actual parameters can be given through `::<>`.
As the actual parameters, numeric literal and identifier concatenated by `::` can be used.

Additionally, the actual parameters should be accessible at the position of the generics declaration.
For example, module names can be used as actual parameters because it is accessible through the whole project.
On the other hand, local parameters can't be used as actual parameters in many cases.
This is caused by that the local parameters is not accessible from the potision of the generics declaration.

## Generic Function

```veryl,playground
module ModuleA {
    function FuncA::<T: u32> (
        a: input logic<T>,
    ) -> logic<T> {
        return a + 1;
    }

    let _a: logic<10> = FuncA::<10>(1);
    let _b: logic<20> = FuncA::<20>(1);
}
```

## Generic Module/Interface

```veryl,playground
module ModuleA {
    inst u0: ModuleB::<ModuleC>;
    inst u1: ModuleB::<ModuleD>;
}

proto module ProtoA;

module ModuleB::<T: ProtoA> {
    inst u: T;
}

module ModuleC for ProtoA {}
module ModuleD for ProtoA {}
```

## Generic Package

```veryl,playground
package PackageA::<T: u32> {
    const X: u32 = T;
}

module ModuleA {
    const A: u32 = PackageA::<1>::X;
    const B: u32 = PackageA::<2>::X;
}
```

## Generic Struct

```veryl,playground
package PackageA {
    type TypeB = u32;
    type TypeC = u64;
}

module ModuleA {
    type TypeA = i32;

    struct StructA::<T: type> {
        A: T,
    }

    // local defined type can be used
    // because `TypeA` is accessible at the definition of `StructA`
    var _a: StructA::<TypeA>          ;
    var _b: StructA::<PackageA::TypeB>;
    var _c: StructA::<PackageA::TypeC>;
}
```
