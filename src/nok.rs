use std::process::Command;

fn main() {
    // Specify the command and arguments for the child process
    let command = "./monitor";

    // Spawn the child process
    let mut child_process = Command::new(command)
        .spawn()
        .expect("Failed to start child process");

    // Optionally, you can do other work here while the child process is running

    // The parent process does not exit immediately
    // It can continue to do other work or wait for the child process if needed

    // For example, waiting for the child process to finish

    // let status = child_process
    //     .wait()
    //     .expect("Failed to wait for child process");

    // // Optionally, you can handle the exit status of the child process here
    // if status.success() {
    //     println!("Child process exited successfully");
    // } else {
    //     println!("Child process exited with an error: {:?}", status);
    // }

    // The parent process can continue its execution here

    // The parent process will exit when it reaches the end of the main function
}
