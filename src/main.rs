use anyhow::anyhow;
use watchexec::cli::ArgsBuilder;
use watchexec::run::{watch, ExecHandler};

const DETECTOR_PATH: &str = "/cnb/lifecycle/detector";
const BUILDER_PATH: &str = "/cnb/lifecycle/builder";
const LAUNCHER_PATH: &str = "/cnb/lifecycle/launcher";
const PROJECT_PATH: &str = "/source/";
const WORK_DIR_PATH: &str = "/workspace/";

fn main() -> anyhow::Result<()> {
    let rsync_to_cmd = format!(
        "rsync -zrq --exclude .git --exclude node_modules {} {}",
        PROJECT_PATH, WORK_DIR_PATH
    );
    let rsync_from_cmd = format!(
        "rsync -zrq --include 'package-lock.json' --exclude '*' {} {}",
        WORK_DIR_PATH, PROJECT_PATH
    );

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
