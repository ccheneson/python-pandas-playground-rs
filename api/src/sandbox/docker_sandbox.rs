use std::process::Command;

use std::error::Error;

use super::{
    errors::{CommandExecutionError, PythonCodeExecutionError},
    Sandbox,
};

pub struct DockerSandbox {
    py_code: String,
    docker_image: String,
}

impl DockerSandbox {
    pub fn new(py_code: String, docker_image: String) -> Self {
        Self {
            py_code,
            docker_image,
        }
    }
}

impl Sandbox for DockerSandbox {
    fn execute_in_sandbox(&self) -> Result<String, Box<dyn Error>> {
        let execution = Command::new("docker")
            .args(&[
                "run",
                "--rm",
                self.docker_image.as_str(),
                "python",
                "-c",
                self.py_code.as_str(),
            ])
            .output();
        match execution {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
                } else {
                    Err(Box::new(PythonCodeExecutionError(
                        String::from_utf8_lossy(&output.stderr).into_owned(),
                    )))
                }
            }
            Err(err) => Err(Box::new(CommandExecutionError(err.to_string()))),
        }
    }
}
