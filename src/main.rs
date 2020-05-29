use anyhow::anyhow;
use watchexec::cli::ArgsBuilder;
use watchexec::run::{watch, ExecHandler};

const DETECTOR_PATH: &str = "/cnb/lifecycle/detector";
const BUILDER_PATH: &str = "/cnb/lifecycle/builder";
const LAUNCHER_PATH: &str = "/cnb/lifecycle/launcher";
const PROJECT_PATH: &str = "/project/";

fn main() -> anyhow::Result<()> {
    let rsync_to_cmd = "rsync -zrq --exclude .git --exclude node_modules /project/ /workspace/";
    let rsync_from_cmd =
        "rsync -zrq --include 'package-lock.json' --exclude '*' /workspace/ /project/";

    let args = ArgsBuilder::default()
        .cmd(vec![format!(
            "{} && {} && {} && {} && {}",
            rsync_to_cmd, DETECTOR_PATH, BUILDER_PATH, rsync_from_cmd, LAUNCHER_PATH
        )])
        .paths(vec![PROJECT_PATH.into()])
        .filters(vec![
            "package.json".into(),
            "*.ejs".into(),
            "*.ts".into(),
            "*.js".into(),
        ])
        .restart(true)
        .build()
        .map_err(|string| anyhow!(string))?;
    let handler = ExecHandler::new(args)?;

    Ok(watch(&handler)?)
}
