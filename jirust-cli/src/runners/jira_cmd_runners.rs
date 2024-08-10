pub mod version_cmd_runner;

pub enum JiraCmdRunners {
    Version(version_cmd_runner::VersionCmdRunner),
}
