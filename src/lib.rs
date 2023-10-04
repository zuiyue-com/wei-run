#[macro_use]
extern crate wei_log;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Run wei command, If the program does not exist/ Under the data/directory, search for the program's configuration file
/// # Arguments
/// * `cmd` - Command name
/// * `param` - Command parameters
pub fn run(cmd: &str, param: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let path = "./".to_owned() + cmd;

    info!("path: {:?}", path);

    if let Ok(data) = command(&path, param.clone()) {
        return Ok(data);
    };

    info!("{} dir: {:?}", cmd, wei_env::dir_bin());
    let path = wei_env::read(&wei_env::dir_bin(),cmd)?;
    command(path.as_str(), param)
}

/// Run command
/// # Arguments
/// * `cmd` - Command name
/// * `param` - Command parameters
pub fn command(cmd: &str, param: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let output = std::process::Command::new(cmd)
    .args(param)
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
    .output()?;

    #[cfg(not(target_os = "windows"))]
    let output = std::process::Command::new(cmd)
    .args(param)
    .output()?;

    match std::str::from_utf8(&output.stdout) {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(Box::new(e))
    }    
}

/// Run wei command, If the program does not exist/ Under the data/directory, search for the program's configuration file
/// # Arguments
/// * `cmd` - Command name
/// * `param` - Command parameters
pub fn run_async(cmd: &str, param: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let path = "./".to_owned() + cmd;

    info!("path: {:?}", path);

    if let Ok(()) = command_async(&path, param.clone()) {
        return Ok(());
    };

    info!("{} dir: {:?}", cmd, wei_env::dir_bin());
    let path = wei_env::read(&wei_env::dir_bin(),cmd)?;
    command_async(path.as_str(), param)
}

pub fn command_async(cmd: &str, param: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    std::process::Command::new(cmd)
    .args(param)
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
    .spawn()?;

    #[cfg(not(target_os = "windows"))]
    std::process::Command::new(cmd)
    .args(param)
    .spawn()?;

    Ok(())
}

use std::process::Command;
pub fn kill(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C").arg(format!("taskkill /IM {}.exe /F", name));
        cmd.output()?;
    }

    #[cfg(target_os = "linux")]
    {
        let mut cmd = Command::new("bash");
        cmd.arg("-c").arg(format!("pkill {}", name));
        cmd.output()?;
    }
    Ok(())
}
