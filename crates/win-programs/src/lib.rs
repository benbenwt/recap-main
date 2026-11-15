use std::process::Command;
use winreg::HKEY;
use winreg::RegKey;
use winreg::enums::*;

#[derive(Debug)]
pub struct WinProgram {
    name: String,
    exe: String,
}

/// Scans the given Uninstall registry key for installed programs.
fn scan_uninstall_key(root: HKEY) -> Vec<(String, String)> {
    let subkeys = vec![
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
    ];
    let mut results = Vec::new();
    for subkey in subkeys {
        let hive = RegKey::predef(root).open_subkey(subkey);
        if let Ok(key) = hive {
            for subkey in key.enum_keys().flatten() {
                if let Ok(app) = key.open_subkey(&subkey) {
                    let name = app.get_value("DisplayName");
                    let icon = app.get_value("DisplayIcon");

                    if let (Ok(name), Ok(icon)) = (name, icon) {
                        results.push((name, icon));
                    }
                }
            }
        }
    }

    results
}

impl WinProgram {
    /// Creates a new WinProgram instance from the given name and program path.
    pub fn new(name: String) -> Result<Self, String> {
        let name = name.trim().to_string();
        let mut all = scan_uninstall_key(HKEY_LOCAL_MACHINE);
        all.extend(scan_uninstall_key(HKEY_CURRENT_USER));

        let program = all
            .into_iter()
            .find(|(n, _)| n == &name)
            .map(|(_, p)| p)
            .ok_or_else(|| format!("Program '{name}' not found"))?;

        let exe = program
            .split(',') // drop anything after the comma
            .next()
            .unwrap_or_default()
            .trim() // trim whitespace
            .trim_matches('"')
            .to_string();

        Ok(WinProgram { name, exe })
    }

    /// Returns the name of the program.
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn exe(&self) -> &str {
        &self.exe
    }
    pub fn start(&self) -> Result<(), String> {
        self.start_with_args(&[])
    }

    pub fn start_with_args(&self, args: &[&str]) -> Result<(), String> {
        let mut command = Command::new(&self.exe);
        if !args.is_empty() {
            command.args(args);
        }

        match command.spawn() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to start program '{}': {}", self.name, e)),
        }
    }

    pub fn list_all() -> Vec<WinProgram> {
        let mut programs = Vec::new();
        let all = scan_uninstall_key(HKEY_LOCAL_MACHINE);
        let mut all2 = scan_uninstall_key(HKEY_CURRENT_USER);
        all2.extend(all);

        for (name, exe) in all2 {
            if !exe.is_empty() {
                if let Ok(program) = WinProgram::new(name) {
                    programs.push(program);
                }
            }
        }
        programs
    }
}
