use func_trace::trace;

func_trace::init_depth_var!();

fn main() {
    let mut a = 10;
    let mut b = 20;
    foo(&mut a, &mut b);
}

#[trace]
fn foo(a: &mut u32, b: &mut u32) {
    *a += 20;
    *b += 40;
    bar(a);
    bar(b);
}

#[trace]
fn bar(x: &mut u32) {
    *x -= 5;
}

#[cfg(test)]
#[macro_use]
mod trace_test;

#[cfg(test)]
trace_test!(test_mut_ref, main());
