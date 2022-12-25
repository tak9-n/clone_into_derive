#[cfg(test)]
#[macro_use]
pub mod test_structs {
    use clone_into_derive::CloneInto;

    #[derive(CloneInto)]
    pub struct IncludedStruct {
        pub a: i32,
        pub b: i32,
    }
    pub struct FullStruct {
        pub a: i32,        pub b: i32,
        pub c: i32,
    }
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn clone_into() {
        let included = super::test_structs::IncludedStruct {a: 0, b: 1};
        let mut full = super::test_structs::FullStruct {a:2, b:3, c:4};
        included_struct_clone_into!(included, full);
        assert!(included.a == full.a);
        assert!(included.b == full.b);
    }
}
