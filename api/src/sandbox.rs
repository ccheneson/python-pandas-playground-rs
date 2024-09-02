pub mod docker_sandbox;

pub trait Sandbox {
    fn execute_in_sandbox(&self, py_code: &str) -> Result<String, anyhow::Error>;
}

pub mod errors {

    use std::{error::Error, fmt};

    #[derive(Debug)]
    pub struct PythonCodeExecutionError(pub String);

    #[derive(Debug)]
    pub struct CommandExecutionError(pub String);

    impl fmt::Display for PythonCodeExecutionError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "PythonCodeExecutionError: {}", self.0)
        }
    }

    impl fmt::Display for CommandExecutionError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "CommandExecutionError: {}", self.0)
        }
    }

    impl Error for PythonCodeExecutionError {}
    impl Error for CommandExecutionError {}
}
