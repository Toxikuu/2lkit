

#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {{
        println!("\x1b[34;1m{}\x1b[0m", format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! die {
    ($($arg:tt)*) => {{
        println!("\x1b[31;1m{}\x1b[0m", format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! e2c {
    () => {{
        use std::io::{self, Write};
        print!("Press Enter to continue...");
        io::stdout().flush().unwrap();
        let mut buf = [0u8; 1];
        io::stdin().read_exact(&mut buf).ok();
    }};
}
