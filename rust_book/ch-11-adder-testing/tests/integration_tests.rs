use ch_11_adder_testing as adder;

#[test]
fn check_add(){
    assert_eq!(4, adder::add_two(2))
}
