# clone_into_derive

This crates generate clone macro for structs by the derive macro `CloneInto`. It's quite convenient if you want to copy a struct to another struct which includes the struct fully, or initialize another struct with the struct.

The derive macro `CloneInto` generates the macro which clone all of public fields in the struct. The macro name derive from the struct's name.
For example, the macro name becomes `aaa_bbb_clone_into!` if the struct name is `AaaBbb`.
```rust
    #[derive(CloneInto)]
    pub struct AaaBbb {
        pub a: i32,
        pub b: i32,
    }
```
This code generate `aaa_bbb_clone_into!` macro. And the `aaa_bbb_clone_into!(aaabbb, ccc)` is expanded like below.
```rust
    ccc.a = aaabbb.a.clone();
    ccc.b = aaabbb.b.clone();
```
`aaa_bbb_ccc_clone!(aaabbb,Ccc{c: 4})` is expanded like below.
```rust
    Ccc {
      a: aaabbb.a.clone(),
      b: aaabbb.b.clone(),
      c: 4
    }
```
You can specify struct name at `Ccc`. But there is one limit. Now, you can't specify the struct name with path, so you have to specfy just only the struct name. It looks like `macro_rules`'s bug. When I used `ty` disanator, The macro expansion result became an error. 

The example is in *tests* directory. 
