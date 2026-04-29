#![feature(internal_output_capture)]

use std::sync::Arc;

#[test]
fn pyramid() {
    std::io::set_output_capture(Some(Default::default()));

    prog_v::pyramid(5, false);
    prog_v::pyramid(5, true);

    let captured = std::io::set_output_capture(None);

    let captured = captured.unwrap();
    let captured = Arc::try_unwrap(captured).unwrap();
    let captured = captured.into_inner().unwrap();
    let captured = String::from_utf8(captured).unwrap();

    assert_eq!(
        captured,
        "     \n*    \n**   \n***  \n**** \n*****\n**** \n***  \n**   \n*    \n     \n     \n    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n     \n"
    );
}

#[test]
fn first_difference() {
    assert_eq!(
        prog_v::first_difference(
            "lalala Ik eet een ei voor ontbijt.",
            "lalala Ik eet citroen op zondag.",
        ),
        Some(14)
    );
}

#[test]
fn count() {
    assert_eq!(prog_v::count(2, vec![1, 4, 2, 1, 5, 5, 7, 2]), 2);
}

#[test]
fn sequential_pairs_biggest_difference() {
    assert_eq!(
        prog_v::sequential_pairs_biggest_difference(vec![1, 3, 6, 2, 3, 5]),
        4
    );
}

#[test]
fn binary_language() {
    assert_eq!(prog_v::binary_language(vec![0, 1, 1, 0, 1, 1, 0, 1]), true);
}

#[test]
fn average() {
    assert_eq!(prog_v::average(vec![1., 2., 3., 4., 5., 6.]), 3.5)
}

#[test]
fn average_lists() {
    assert_eq!(
        prog_v::average_lists(vec![vec![1., 2., 2., 1.], vec![3., 6.], vec![2., 3., 4.]]),
        3.
    );
}

#[test]
fn fibonacci() {
    assert_eq!(prog_v::fibonacci(10), 55);
}

#[test]
fn fizzbuzz() {
    std::io::set_output_capture(Some(Default::default()));

    prog_v::fizzbuzz();

    let captured = std::io::set_output_capture(None);

    let captured = captured.unwrap();
    let captured = Arc::try_unwrap(captured).unwrap();
    let captured = captured.into_inner().unwrap();
    let captured = String::from_utf8(captured).unwrap();

    assert_eq!(captured.trim(), "1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz 16 17 fizz 19 buzz fizz 22 23 fizz buzz 26 fizz 28 29 fizzbuzz 31 32 fizz 34 buzz fizz 37 38 fizz buzz 41 fizz 43 44 fizzbuzz 46 47 fizz 49 buzz fizz 52 53 fizz buzz 56 fizz 58 59 fizzbuzz 61 62 fizz 64 buzz fizz 67 68 fizz buzz 71 fizz 73 74 fizzbuzz 76 77 fizz 79 buzz fizz 82 83 fizz buzz 86 fizz 88 89 fizzbuzz 91 92 fizz 94 buzz fizz 97 98 fizz buzz\n".trim());
}

#[test]
fn sum_polymorphic() {
    assert_eq!(prog_v::sum_poly(vec![1_f64, 2., 3., 4., 5.]), 15.);
    assert_eq!(prog_v::sum_poly(vec![1_usize, 2, 3, 4, 5]), 15);
}

#[test]
fn average_polymorphic() {
    assert_eq!(prog_v::average_poly(vec![1_f64, 2., 3., 4., 5.]), 3.);
    assert_eq!(prog_v::average_poly(vec![1_usize, 2, 3, 4, 5]), 3);
}

#[test]
fn test_bump_score() {
    //let begin = 42;
    //let mut test = (begin, "John Doe");
    //prog_v::bump_score(&mut test);
    //assert_eq!(test.0, 43);
}

#[test]
fn test_counting() {
    //let mut counter = prog_v::make_counter();
    //assert_eq!(*counter, 0);
    //prog_v::increment(&mut counter);
    //assert_eq!(*counter, 1);
}
