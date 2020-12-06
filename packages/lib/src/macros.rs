#[macro_export]
macro_rules! error_and_panic {
    ($message:expr) => {{
        error!("{}", $message);
        panic!($message);
    }};

    ($message:expr, $error:expr) => {{
        let message = format!("{}: [{}]", $message, $error);
        error!("{}", message);
        panic!(message);
    }};
}
