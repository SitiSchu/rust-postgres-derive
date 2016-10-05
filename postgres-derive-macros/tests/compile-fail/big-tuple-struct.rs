#![feature(plugin, custom_derive)]
#![plugin(postgres_derive_macros)]

macro_rules! to_sql_checked {
    () => ()
}

#[derive(Clone, ToSql)] //~ ERROR #[derive(ToSql)] can only be applied to structs, single field tuple structs, and enums
struct Foo(i32, i32);

#[derive(Clone, FromSql)] //~ ERROR #[derive(FromSql)] can only be applied to structs, single field tuple structs, and enums
struct Bar(i32, i32);

fn main() {}
