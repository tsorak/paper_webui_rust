use std::path::PathBuf;

use tokio::fs;

const DOT_DIR: &str = ".pwui";
const SERVERS_DIR: &str = "jars";
const INSTANCE_NAME_PAT: &str = "instance_";

pub struct Fs {
    workdir: PathBuf,
}
