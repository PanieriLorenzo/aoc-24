fn main() {
    println!(
        "result: {}",
        // wrapping a value in Some lets us map instead of assigning to variables
        Some(
            std::fs::read_to_string("input.txt")
                .unwrap()
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.split("   ")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|v| (v[0], v[1]))
                .unzip::<_, _, Vec<_>, Vec<_>>(),
        )
        // sort is in-place so we use inspect to mutate it without introducing a statement
        // but inspect borrows its argument immutably, so we wrap it in RefCell to allow mutation
        .map(|(v1, v2)| (std::cell::RefCell::new(v1), std::cell::RefCell::new(v2)))
        .inspect(|(v1, _)| v1.borrow_mut().sort())
        .inspect(|(_, v2)| v2.borrow_mut().sort())
        .map(|(v1, v2)| (v1.take().into_iter().zip(v2.take())))
        .unwrap()
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>()
    )
}
