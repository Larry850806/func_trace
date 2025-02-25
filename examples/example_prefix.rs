use func_trace::trace;

func_trace::init_depth_var!();

fn main() {
    foo(1, 2);
}

#[trace]
fn foo(a: i32, b: i32) {
    println!("I'm in foo!");
    bar((a, b));
}

#[trace(prefix_enter = "[ENTER]", prefix_exit = "[EXIT]")]
fn bar((a, b): (i32, i32)) -> i32 {
    println!("I'm in bar!");
    if a == 1 {
        2
    } else {
        b
    }
}

#[cfg(test)]
#[macro_use]
mod trace_test;

#[cfg(test)]
trace_test!(test_prefix, main());
