fn test_parsing_on_multiple_threads() {
    let mut parser = Parser::new();

    let mut parse_threads = Vec::new();
    for thread_id in 1..5 {
        let mut tree_clone = tree.clone();
        parse_threads.push(thread::spawn(move || {
            // For each thread, prepend a different number of declarations to the
            // source code.
            let mut prepend_line_count = 0;
            let mut prepended_source = String::new();
            let mut parser = Parser::new();
        }));
    }
}
