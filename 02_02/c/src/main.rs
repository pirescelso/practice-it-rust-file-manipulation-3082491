#![allow(unused)]
fn main() {
    let values = vec![1, 2, 3, 4, 5];
    {
        // let result = match IntoIterator::into_iter(values) {
        match IntoIterator::into_iter(values) {
            mut xxxx => loop {
                let yyyy;
                match xxxx.next() {
                    Some(val) => yyyy = val,
                    None => break,
                };
                // let x = yyyy;
                // let () = {
                println!("{yyyy}");
                // };
            },
        };
        // result
    }

    let values_2 = vec![1, 2, 3];
    let mut iter = values_2.into_iter();
    loop {
        match iter.next() {
            Some(x) => println!("{x}"),
            None => break,
        };
    }

    match (55 + 55) {
        mut value => {
            value += 1;
            println!("{}", value)
        }
    }
}
