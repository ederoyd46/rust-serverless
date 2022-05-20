use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub kind: String,
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppError {{ kind: {}, message: {} }}",
            self.kind, self.message
        )
    }
}

// Implement std::convert::From for AppError; from io::Error
// impl From<io::Error> for AppError {
//     fn from(error: io::Error) -> Self {
//         AppError {
//             kind: String::from("io"),
//             message: error.to_string(),
//         }
//     }
// }
