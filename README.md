# Rust Command Execution and File I/O Example

This project demonstrates how to execute system commands and handle file I/O in Rust. It includes examples of running Command Prompt and PowerShell commands, capturing their output, and reading from and writing to files.

## Requirements

- Rust (latest stable version)
- A Windows environment (for Command Prompt and PowerShell commands)
- `input.txt` file in the same directory as the executable, containing some sample text

## Running the Project

1. **Clone the Repository**
    ```sh
    git clone https://github.com/weldonkipchirchir/spawn_extrenal_process.git
    cd spawn_extrenal_process
    ```

2. **Ensure the `input.txt` File Exists**
    - Create an `input.txt` file in the project directory with some sample text.

3. **Build the Project**
    ```sh
    cargo build
    ```

4. **Run the Project**
    ```sh
    cargo run
    ```

## What the Code Does

### Main Function

1. **Execute Command Prompt `dir` Command**
    - Runs the `dir` command to list directory contents.
    - Captures and prints the output and any errors.

2. **Execute PowerShell Command**
    - Runs a PowerShell command that outputs a message including an environment variable `FIRST_NAME` set to "Weldon".
    - Spawns the command as a separate process and waits for it to complete.
    - Prints the exit status of the command.

3. **Call `test_stdin` Function**
    - Reads the contents of `input.txt`.
    - Passes the contents to a PowerShell command that converts the input to uppercase and selects the first two lines.
    - Captures and prints the output.

### `test_stdin` Function

- Opens and reads the `input.txt` file.
- Executes a PowerShell command with the file contents as input.
- Writes the file contents to the command's standard input.
- Captures and prints the command's output.

# License
This project is licensed under the MIT License. See the LICENSE file for details.

This `README.md` provides a comprehensive overview of your project, including requirements
