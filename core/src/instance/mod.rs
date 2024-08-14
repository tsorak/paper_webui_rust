use std::path::PathBuf;

use tokio::process::Command;

use crate::error::instance::ServerExecutable;

pub struct InstanceConfig {
    pub name: String,
    pub mem_initial: String,
    pub mem_max: String,
    executable: Option<PathBuf>,
    pub port: String,
}

pub struct Instance {
    cmd: Command,
}

impl Instance {
    pub fn new(cfg: InstanceConfig) -> Result<Self, crate::Error> {
        let mut cmd = Command::new("java");
        cmd.args([
            "-jar",
            "-Xms",
            &cfg.mem_initial,
            "-Xmx",
            &cfg.mem_max,
            cfg.executable
                .ok_or(ServerExecutable::box_msg(
                    "No server executable was provided",
                ))?
                .to_str()
                .ok_or(ServerExecutable::box_msg(
                    "Executable path contains invalid characters",
                ))?,
            "--nogui",
            "--port",
            &cfg.port,
        ]);

        Ok(Self { cmd })
    }
}

impl InstanceConfig {
    pub fn with_executable(&mut self, p: impl Into<PathBuf>) -> Result<(), crate::Error> {
        use crate::error::instance::ServerExecutable;

        let p: PathBuf = p.into();
        let p = p
            .canonicalize()
            .map_err(|e| ServerExecutable::box_new(p, format!("ServerExecutable error: {e}")))?;

        self.executable = Some(p);
        Ok(())
    }
}

impl Default for InstanceConfig {
    fn default() -> Self {
        Self {
            name: "default".into(),
            mem_initial: "256M".into(),
            mem_max: "2G".into(),
            executable: None,
            port: "25565".into(),
        }
    }
}
