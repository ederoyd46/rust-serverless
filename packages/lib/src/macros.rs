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

#[macro_export]
macro_rules! log_and_throw {
    ($message:expr) => {{
        error!("{}", $message);
        return Err($message);
    }};

    ($message:expr, $error:expr) => {{
        let message = format!("{}: [{}]", $message, $error);
        error!("{}", message);
        return Err($error);
    }};
}

#[macro_export]
macro_rules! log_and_exit {
    ($message:expr) => {{
        error!("{}", $message);
        std::process::exit(1);
    }};

    ($message:expr, $return_code:expr) => {{
        let message = format!("{}: [{}]", $message, $return_code);
        error!("{}", message);
        std::process::exit($return_code);
    }};
}
