#[macro_export]
macro_rules! error_and_panic {
    ($message:expr) => {{
        error!("{}", $message);
        panic!($message);
    }};

    ($message:expr, $error:expr) => {{
        error!("{}: [{}]", $message, $error);
        panic!("{}: [{}]", $message, $error);
    }};
}

#[macro_export]
macro_rules! log_and_throw {
    ($message:expr) => {{
        error!("{}", $message);
        return Err(AppError {
            kind: "Application",
            message: $message,
        });
    }};

    ($message:expr, $error:expr) => {{
        let message = format!("{}: [{}]", $message, $error);
        error!("{}", message);
        return Err($error);
    }};
}
