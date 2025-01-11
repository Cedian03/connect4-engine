use std::process::Command;

fn main() {
    let output = Command::new("python3").arg("./magic.py").output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Magic script executed successfully!")
            } else {
                panic!(
                    "Magic script failed with status: {}\nstderr: {}",
                    output.status,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(error) => {
            panic!("Failed to execute magic script: {}", error)
        }
    }

    println!("cargo:rerun-if-changed=./magic.py");
}
