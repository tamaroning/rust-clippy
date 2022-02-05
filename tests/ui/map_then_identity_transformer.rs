#![warn(clippy::map_then_identity_transformer)]
#![allow(clippy::map_identity, clippy::redundant_closure, clippy::unnecessary_filter_map)]

fn main() {
    let a = [1, 2, 3];
    let b = ["ABC", "DEF", "GHI"];

    // should lint
    let _ = a.into_iter().map(|x| bar(foo(x)) && foo(x)).all(|y| !y);
    let _ = a.into_iter().map(|x| x > 1).all(|y| y);
    let _ = a.into_iter().map(|x| foo(x)).any(|y| y);
    let _ = a.into_iter().map(|x| x + 10).find(|&y| y == 10);
    let _ = a.into_iter().map(|x| x + 10).find_map(|y| Some(y));
    let _ = b.into_iter().map(|x| x).flat_map(|y| y.chars());
    let _ = a.into_iter().map(|x| foo(x)).filter_map(|y| Some(!y));
    let _ = a.into_iter().map(|x| x + 30).fold(1, |pd, x| pd * x + 1);
    let _ = a.into_iter().map(|x| foo(x)).map(|y| bar(y));

    // should not lint
    let _ = a.into_iter().map(|x| x > 1).all(|y| foo(0));
    let _ = a.into_iter().map(|x| foo(x)).any(|y| true);
    let _ = a.into_iter().map(|x| 0).find(|&y| y == 10);
    let _ = a.into_iter().map(|x| x * x).find_map(|y| Some(y));
    let _ = b.into_iter().map(|x| "ABC").flat_map(|y| y.chars());
    let _ = a.into_iter().map(|x| foo(x)).filter_map(|y| Some(true));
    let _ = a.into_iter().map(|x| x + 30).fold(1, |pd, x| pd * x * x);
    let _ = a.into_iter().map(|x| foo(x)).map(|y| bar(y) && y);
}

fn foo(a: i32) -> bool {
    unimplemented!();
}

fn bar(a: bool) -> bool {
    unimplemented!();
}
