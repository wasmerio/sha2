use std::{fs::File, path::Path, process::Command};
use tempfile::tempdir;

use flate2::write::GzEncoder;
use flate2::Compression;

mod common;

// check if wasmer is installed.

// ./python3 ../../../../../../tests/main.py

#[test]
fn compile_webc_container() {
    Command::new("cargo")
        .args(["wapm", "--dry-run"])
        .output()
        .expect("Failed to execute command");

    let temp_dir = tempdir().unwrap();

    let project_dir_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    // let wapm_dir = project_dir.join("target/wapm/sha2-wasm");

    // let mut wapm_dir_files = vec![];
    // for entry in wapm_dir.read_dir().expect("read_dir call failed") {
    //     if let Ok(entry) = entry {
    //         wapm_dir_files.push(entry.file_name())
    //     }
    // }

    let tar_gz = File::create("sha2_wasm.tar.gz").unwrap();
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("./sha2_wasm", temp_dir.path().display().to_string())
        .unwrap();
    tar.finish().unwrap();
    // let tar_out = Command::new("tar")
    //     .current_dir(&wapm_dir)
    //     .arg("-czvf")
    //     .arg(temp_dir.path().join("sha2_wasm.tar.gz"))
    //     .args(wapm_dir_files)
    //     .output()
    //     .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    /*
    let w2p_out = Command::new("wasmer")
        .current_dir(temp_dir.path())
        .args([
            "run",
            "wapm2pirita",
            "--mapdir",
            format!(".:{}", temp_dir.path().display()).as_str(),
            "--",
            "convert",
        ])
        .arg("sha2_wasm.tar.gz")
        .arg("sha2_wasm.webc")
        .output()
        .expect("wapm2pirita command failed to execute");

    println!("{:?}", w2p_out);
    let wasmer_pack_out = Command::new("wasmer")
        .current_dir(temp_dir.path())
        .args([
            "run",
            "wasmer-pack",
            "--mapdir",
            format!("f:{}", temp_dir.path().display()).as_str(),
            "--",
            "python",
            "f/sha2_wasm.webc",
            "--out-dir",
            "f/python_sha2",
        ])
        .output()
        .expect("wasmer-pack failed to create python binding");

    println!("{:?}", wasmer_pack_out);

    let python_sha2_dir = temp_dir.path().join("python_sha2");

    assert!(python_sha2_dir.exists());
    Command::new("python3")
        .current_dir(&python_sha2_dir)
        .args(["-m", "venv", "env"])
        .output()
        .expect("Python environment creation failed");

    let test_dir = project_dir_path.join("tests");

    let pip_out = Command::new("./env/bin/pip")
        .current_dir(&python_sha2_dir)
        .args(["install", ".", "pytest"])
        .output()
        .expect("msg");

    assert!(pip_out.status.success(), "{pip_out:?}");
    let py_out = Command::new("./env/bin/pytest")
        .current_dir(&python_sha2_dir)
        .arg(test_dir.join("main.py"))
        .output()
        .expect("msg");
    println!("{py_out:?}");
    */
}
