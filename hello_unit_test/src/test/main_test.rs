#[path = "../main/main.rs"]
mod main;

#[cfg(test)]
#[test]
fn does_it_though() {
    assert_eq!(2 + 2, 4);
}
#[test]
fn it_works() {
    assert_eq!(main::some_function(5), "Yes");
}
