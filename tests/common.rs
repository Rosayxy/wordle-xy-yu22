use std::fs::File;
use std::io::prelude::*;
use std::io::*;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use assert_json_diff::assert_json_eq;
use lazy_static::lazy_static;
use pretty_assertions::assert_eq;
use serde_json;

// Binary targets are automatically built if there is an integration test.
// This allows an integration test to execute the binary to exercise and test its behavior.
// The CARGO_BIN_EXE_<name> environment variable is set when the integration test is built
// so that it can use the env macro to locate the executable.
lazy_static! {
    static ref EXE_PATH: PathBuf = env!("CARGO_BIN_EXE_wordle").into();
}

pub struct TestCase {
    name: String,
    arguments: Vec<String>,
    input: String,
    answer: String,
}

impl TestCase {
    pub fn read(name: &str) -> Self {
        let case_dir = Path::new("tests").join("cases");
        let in_file = case_dir.join(format!("{}.in", name));
        let ans_file = case_dir.join(format!("{}.ans", name));
        let args_file = case_dir.join(format!("{}.args", name));

        let in_content = std::fs::read_to_string(in_file).unwrap();
        let ans_content = std::fs::read_to_string(ans_file).unwrap();
        let args_content = std::fs::read_to_string(args_file).unwrap();

        Self {
            name: name.to_string(),
            arguments: args_content
                .trim()
                .split("\n")
                .filter(|s| s != &"") // remove empty lines
                .map(|s| s.to_string())
                .collect(),
            input: in_content,
            answer: ans_content,
        }
    }

    fn execute_program_and_feed_input(&self) -> Child {
        let mut command = Command::new(EXE_PATH.as_os_str())
            .args(&self.arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        // feed stdin
        command
            .stdin
            .take()
            .unwrap()
            .write_all(self.input.as_bytes())
            .unwrap();
        command
    }

    pub fn run_and_compare_result(&self) {
        let mut command = self.execute_program_and_feed_input();
        // read stdout from user program
        let mut output = Vec::new();
        command
            .stdout
            .take()
            .unwrap()
            .read_to_end(&mut output)
            .unwrap();
        let output = String::from_utf8(output).unwrap();

        // command.try_wait();

        // wait for the program to exit normally
        assert!(
            command.wait().expect("failed to wait on process").success(),
            "case {} should exit normally",
            self.name
        );

        // compare result
        assert_eq!(
            output.trim(),
            self.answer.trim(),
            "case {} incorrect",
            self.name
        );
    }

    pub fn run_and_compare_game_state(&mut self) {
        // read state before & end
        let case_dir = Path::new("tests").join("cases");
        let before_state_file = case_dir.join(format!("{}.before.json", self.name));
        let run_state_file = case_dir.join(format!("{}.run.json", self.name));
        let after_state_file = case_dir.join(format!("{}.after.json", self.name));

        // run with temporary state file
        std::fs::copy(&before_state_file, &run_state_file).unwrap();
        self.arguments.append(&mut vec![
            "--state".to_string(),
            run_state_file.to_str().unwrap().to_string(),
        ]);
        self.run_and_compare_result();

        // load state and compare with answer
        let run_state: serde_json::Value =
            serde_json::from_reader(BufReader::new(File::open(&run_state_file).unwrap())).unwrap();
        let answer_state: serde_json::Value =
            serde_json::from_reader(BufReader::new(File::open(&after_state_file).unwrap()))
                .unwrap();
        assert_json_eq!(run_state, answer_state);
    }

    pub fn run_and_expect_exit(&self) {
        let command = self.execute_program_and_feed_input();
        assert!(
            !command
                .wait_with_output()
                .expect("failed to wait on process")
                .status
                .success(),
            "case {} should exit with error",
            self.name
        );
    }
}
