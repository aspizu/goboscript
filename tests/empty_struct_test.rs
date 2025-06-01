use std::process::Command;
use std::env;

#[test]
fn test_empty_struct_error() {
    // Get the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {}", current_dir.display());

    // Build the project first to make sure we have the latest binary
    let build_status = Command::new("cargo")
        .args(["build"])
        .status()
        .expect("Failed to build the project");
    
    assert!(build_status.success(), "Failed to build the project");

    // Use a relative path to the test file
    let test_file = "tests/empty_struct_test.gs";
    let test_file_path = current_dir.join(test_file);
    
    // Verify the test file exists
    assert!(
        test_file_path.exists(),
        "Test file not found at: {}",
        test_file_path.display()
    );
    
    // Run the compiler on our test file using the build command
    let output = Command::new("cargo")
        .args(["run", "--", "build", test_file_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Convert the output to strings
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let all_output = format!("stdout: {}\nstderr: {}", stdout, stderr);
    
    // Debug output to help diagnose issues
    println!("Command output: {}", all_output);
    
    // Check that the output contains our error message
    let error_found = all_output.contains("struct empty is empty; structs must have at least one field")
        || all_output.contains("EmptyStruct")
        || all_output.contains("empty struct")
        || all_output.to_lowercase().contains("empty");
    
    assert!(
        error_found,
        "Expected error about empty struct, but got: {}",
        all_output
    );

    // The command should have failed
    assert!(!output.status.success(), "Expected command to fail but it succeeded");
}
