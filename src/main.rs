use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

fn main() {
    // Execute the "dir" command in the Command Prompt
    let mut command = Command::new("cmd");
    command.arg("/c").arg("dir");

    // Execute the command and capture the output
    let output = command.output().expect("Failed to execute command");

    // Print the output (if any)
    if !output.stdout.is_empty() {
        println!("Command output:\n{}", String::from_utf8_lossy(&output.stdout));
    }

    // Print the error output (if any)
    if !output.stderr.is_empty() {
        println!("Command error:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    // Execute a PowerShell command
    let mut command2 = Command::new("powershell.exe");
    command2.args(["-Command", "Write-Host", "Hello from PowerShell, user: $env:FIRST_NAME"]);
    command2.env("FIRST_NAME", "Weldon");

    // Spawn a new process for the PowerShell command
    let mut command2_handle = command2.spawn().unwrap();

    println!("Doing some more work ...");

    // Wait for the PowerShell command to complete
    let result = command2_handle.wait().unwrap();
    println!("Exited with status code: {}", result.code().unwrap());

    test_stdin();
}

fn test_stdin() {
    // Open the file for reading
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Read the first two lines from the file
    let mut input_data = String::new();
    for (_, line) in reader.lines().enumerate() {
            input_data.push_str(&line.expect("Failed to read line"));
            input_data.push('\n');
    }

    // Set up the PowerShell command
    let mut head_cmd = Command::new("powershell.exe");
    head_cmd.arg("-Command")
            .arg("$input | ForEach-Object { $_.ToUpper() } | Select-Object -First 2");
    head_cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    // Spawn the process
    let mut proc_handle = head_cmd.spawn().expect("Failed to spawn process");

    // Write the input_data to the command's standard input
    {
        let stdin_handle = proc_handle.stdin.as_mut().expect("Failed to open stdin");
        stdin_handle.write_all(input_data.as_bytes()).expect("Failed to write to stdin");
    }

    // Read the output from the command
    let output = proc_handle.wait_with_output().expect("Failed to read output");
    let output_buffer = String::from_utf8_lossy(&output.stdout);

    // Print the result
    println!("Result is: {}", output_buffer);
}
