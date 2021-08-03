#[spec]
fn my_function(n: u32) -> u32 { n + 1 }

#[exec]
fn main() {}

/*
#[proof]
fn lemma_f(n: u32) {
//    ensures(my_function(n) > n);

}

#[exec]
fn main() {
    #[spec] let x = 42;
    let y = my_function(x);
    let z = 7;
    lemma_f(x);
//    assert(my_function(7) > 7);
//    assert(y > x);
    let w = z + 5;
    ()
}
*/
