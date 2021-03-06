pub fn failure_to_string
(e: failure::Error)
-> String
{
    use std::fmt::Write;

    let mut result = String::new();

    for (i, cause) in e.iter_chain().collect::<Vec<_>>().into_iter().rev().enumerate() {
        if i > 0 {
            let _ = writeln!(&mut result, "  Which Caused the following issue:");
        }
        let _ = writeln!(&mut result, "{}", cause);

        if let Some(backtrace) = cause.backtrace() {
            let backtrace_str = format!("{}", backtrace);

            if backtrace_str.len() > 0 {
                let _ = writeln!(&mut result, " This happended at {}", backtrace);
            } else {
                let _ = writeln!(&mut result);
            }
        }
    }

    result
}
