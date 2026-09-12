use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Safe,
    Warning,
    Danger,
}

impl RiskLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            RiskLevel::Safe => "safe",
            RiskLevel::Warning => "warning",
            RiskLevel::Danger => "danger",
        }
    }
}

pub fn assess_process_risk(exe_name: &str) -> RiskLevel {
    let lower = exe_name.to_lowercase();
    let name = lower.trim_end_matches(".exe");

    // Critical Windows kernel and session processes (Killing may cause BSOD or session kill)
    const DANGER_LIST: &[&str] = &[
        "csrss",
        "lsass",
        "smss",
        "services",
        "wininit",
        "winlogon",
        "ntoskrnl",
        "dwm",
        "system",
        "registry",
        "fontdrvhost",
    ];

    if DANGER_LIST.contains(&name) {
        return RiskLevel::Danger;
    }

    // System services, shells, and sync engines (Should be treated with caution)
    const WARNING_LIST: &[&str] = &[
        "explorer",
        "svchost",
        "onedrive",
        "onedrivestandaloneupdater",
        "dropbox",
        "googledrivesync",
        "searchindexer",
        "searchhost",
        "spoolsv",
        "taskhostw",
        "sihost",
        "ctfmon",
        "securityhealthservice",
        "msmpeng",
        "antimalware",
    ];

    if WARNING_LIST.contains(&name) {
        return RiskLevel::Warning;
    }

    RiskLevel::Safe
}
