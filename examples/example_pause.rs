use func_trace::trace;

func_trace::init_depth_var!();

fn main() {
    foo(1);
}

#[trace(pause)]
fn foo(a: i32) -> i32 {
    a
}
