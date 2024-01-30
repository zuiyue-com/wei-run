#[macro_use]
extern crate wei_log;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Run wei command, If the program does not exist/ Under the data/directory, search for the program's configuration file
/// # Arguments
/// * `cmd` - Command name
/// * `param` - Command parameters
pub fn run(cmd: &str, param: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let path = "./".to_owned() + cmd;

    info!("run: {:?}, param: {:?}", path, param);

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
pub fn command(cmd: &str, param: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    info!("wei-run command: {:?}, param: {:?}", cmd, param);
    #[cfg(target_os = "windows")]
    let output = std::process::Command::new(cmd)
    .args(param)
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
    .output()?;

    #[cfg(not(target_os = "windows"))]
    let output = std::process::Command::new(cmd)
    .args(param).output()?;

    let data = format!("{}{}", 
        std::str::from_utf8(&output.stdout)?, 
        std::str::from_utf8(&output.stderr)?
    );

    Ok(data)
}

/// Run command_output
/// # Arguments
/// * `cmd` - Command name
/// * `param` - Command parameters
pub fn command_output(cmd: &str, param: Vec<&str>) -> Result<std::process::Output, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let output = std::process::Command::new(cmd)
    .args(param)
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
    .output()?;

    #[cfg(not(target_os = "windows"))]
    let output = std::process::Command::new(cmd)
    .args(param).output()?;

    Ok(output)
}

/// Run wei command, If the program does not exist/ Under the data/directory, search for the program's configuration file
/// # Arguments
/// * `cmd` - Command name
/// * `param` - Command parameters
pub fn run_async(cmd: &str, param: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let path = "./".to_owned() + cmd;

    info!("run_async: {:?}, param: {:?}", path, param);

    if let Ok(()) = command_async(&path, param.clone()) {
        return Ok(());
    };

    info!("{} dir: {:?}", cmd, wei_env::dir_bin());
    let path = wei_env::read(&wei_env::dir_bin(),cmd)?;
    command_async(path.as_str(), param)
}

pub fn command_async(cmd: &str, param: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn kill(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        // let mut cmd = Command::new("cmd");
        // cmd.arg("/C").arg(format!("taskkill /IM {}.exe /F", name));
        // cmd.output()?;
        psrun("wei-close.ps1", name)?;
    }

    #[cfg(target_os = "linux")]
    {
        let mut cmd = std::process::Command::new("bash");
        cmd.arg("-c").arg(format!("pkill {}", name));
        cmd.output()?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn psrun(name: &str, param: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("powershell")
    .arg("-ExecutionPolicy").arg("Bypass")
    .arg("-File").arg(name).arg(param)
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW).output()?;

    Ok(())
}
