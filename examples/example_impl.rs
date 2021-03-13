use func_trace::trace;

func_trace::init_depth_var!();

fn main() {
    let foo = Foo;
    Foo::foo(2);
    foo.bar(7);
}

struct Foo;

#[trace]
impl Foo {
    fn foo(b: i32) -> i32 {
        b
    }

    fn bar(&self, a: i32) -> i32 {
        a
    }
}

#[cfg(test)]
#[macro_use]
mod trace_test;

#[cfg(test)]
trace_test!(test_impl, main());
