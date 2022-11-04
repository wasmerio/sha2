use std::{fs::File, path::Path, process::Command};
use tempfile::tempdir;

use flate2::write::GzEncoder;
use flate2::Compression;

mod common;

// check if wasmer is installed. [done in github actions];

// ./python3 ../../../../../../tests/main.py

#[test]
fn compile_webc_container() {
    let wapm_out = Command::new("cargo")
        .args(["wapm", "--dry-run"])
        .output()
        .expect("Failed to execute command");

    assert!(wapm_out.status.success(), "{wapm_out:?}");

    let temp_dir = tempdir().unwrap();

    let project_dir_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let wapm_dir = project_dir_path.join("target/wapm/sha2-wasm");

    assert!(wapm_dir.exists(), "wapm dir for sha2 not found");

    // Create tar.gz
    let tar_gz = File::create(temp_dir.path().join("sha2_wasm.tar.gz")).unwrap();
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", &wapm_dir).unwrap();
    tar.into_inner()
        .expect("Unable to finalise the tar archive")
        .finish()
        .expect("Unable to finalize the gzip encoder");

    // Wasm to pirita convert
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

    assert!(w2p_out.status.success(), "{w2p_out:?}");

    // Wasmer-pack for generating python bindings
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

    assert!(wasmer_pack_out.status.success(), "{wasmer_pack_out:?}");

    // check python sha2 dir
    let python_sha2_dir = temp_dir.path().join("python_sha2");
    assert!(python_sha2_dir.exists());

    // create python environment
    let python_env_creation_out = Command::new("python3")
        .current_dir(&python_sha2_dir)
        .args(["-m", "venv", "env"])
        .output()
        .expect("Python environment creation failed");

    assert!(
        python_env_creation_out.status.success(),
        "{python_env_creation_out:?}"
    );

    // create python environment
    let test_dir = project_dir_path.join("tests");
    assert!(test_dir.exists(), "Error: No test directory found");

    // install packages in environment using pip
    let pip_out = Command::new("./env/bin/pip")
        .current_dir(&python_sha2_dir)
        .args(["install", ".", "pytest"])
        .output()
        .expect("msg");

    assert!(pip_out.status.success(), "{pip_out:?}");

    // Run the python tests using pytest and record output
    let pytest_out = Command::new("./env/bin/pytest")
        .current_dir(&python_sha2_dir)
        .arg(test_dir.join("main.py"))
        .output()
        .expect("msg");

    assert!(pytest_out.status.success(), "{pytest_out:?}");
}
