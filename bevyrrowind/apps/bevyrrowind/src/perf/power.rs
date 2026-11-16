//! Windows power state for a bench chain (perf-verdict audit 2026-11-16 §5 fix 6): the power plan
//! (`powercfg /list`, `/setactive`), the Windows 11 power mode (the "overlay" set by the Settings slider, read
//! from the registry: `powercfg` can't set it and `unsafe_code = forbid` rules out the `WinAPI` call), and the
//! per-exe power-throttling exemption. Shelling out only; the parsers are pure and tested.

use std::path::{Path, PathBuf};
use std::process::Command;

/// One power plan from `powercfg /list`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scheme {
    pub guid: String,
    pub name: String,
    pub active: bool,
}

/// Ultimate Performance and High performance (Windows' built-in GUIDs), fastest first.
const FAST_GUIDS: [&str; 2] = ["e9a42b02-d5df-448d-aa00-03f14749eb61", "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"];

/// Parses `powercfg /list`: `Power Scheme GUID: <guid>  (<name>) *` (the `*` marks the active one).
pub fn parse_schemes(out: &str) -> Vec<Scheme> {
    out.lines()
        .filter_map(|l| {
            let rest = l.split_once("GUID:")?.1.trim();
            let (guid, tail) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
            let tail = tail.trim();
            let active = tail.ends_with('*');
            let name = tail.trim_end_matches('*').trim().trim_start_matches('(').trim_end_matches(')');
            Some(Scheme { guid: guid.to_ascii_lowercase(), name: name.to_owned(), active })
        })
        .collect()
}

/// A plan the chain should run on: Ultimate Performance, then High performance (by GUID, else by name).
pub fn is_fast(s: &Scheme) -> bool {
    let name = s.name.to_ascii_lowercase();
    FAST_GUIDS.contains(&s.guid.as_str()) || name.contains("ultimate performance") || name.contains("high performance")
}

/// What to do with the plan at chain start.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlanAction {
    /// The active plan is already a fast one.
    Keep(Scheme),
    /// Switch from the active plan to a fast one, restore it after the chain.
    Switch { from: Scheme, to: Scheme },
    /// No fast plan on this machine (laptops with Modern Standby ship only Balanced): keep the active one.
    Missing(Option<Scheme>),
}

pub fn plan_action(schemes: &[Scheme]) -> PlanAction {
    let active = schemes.iter().find(|s| s.active).cloned();
    if let Some(a) = active.as_ref().filter(|a| is_fast(a)) {
        return PlanAction::Keep(a.clone());
    }
    let fast = FAST_GUIDS
        .iter()
        .find_map(|g| schemes.iter().find(|s| s.guid == *g))
        .or_else(|| schemes.iter().find(|s| is_fast(s)));
    match (active, fast) {
        (Some(from), Some(to)) => PlanAction::Switch { from, to: to.clone() },
        (active, _) => PlanAction::Missing(active),
    }
}

/// The power mode GUID (`ActiveOverlay{Ac,Dc}PowerScheme`) from
/// `reg query HKLM\SYSTEM\CurrentControlSet\Control\Power\User\PowerSchemes`.
pub fn parse_overlay(reg: &str, on_battery: bool) -> Option<String> {
    let key = if on_battery { "ActiveOverlayDcPowerScheme" } else { "ActiveOverlayAcPowerScheme" };
    reg.lines().find_map(|l| {
        let mut f = l.split_whitespace();
        (f.next()? == key).then(|| f.nth(1).map(str::to_ascii_lowercase))?
    })
}

/// The Settings slider's name for a power mode GUID.
pub fn overlay_name(guid: &str) -> String {
    match guid {
        "ded574b5-45a0-4f42-8737-46345c09c238" => "best performance".to_owned(),
        "3af9b8d9-7c97-431d-ad78-34a8bfea439f" => "better performance".to_owned(),
        "00000000-0000-0000-0000-000000000000" => "balanced".to_owned(),
        "961cc777-2547-4f9d-8174-7d86181b8a7a" => "best power efficiency".to_owned(),
        g => format!("unknown ({g})"),
    }
}

fn run(program: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(program).args(args).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

/// `powercfg /list` on this machine (empty elsewhere).
pub fn schemes() -> Vec<Scheme> {
    run("powercfg", &["/list"]).as_deref().map(parse_schemes).unwrap_or_default()
}

/// `powercfg /setactive <guid>`.
pub fn set_active(guid: &str) -> bool {
    run("powercfg", &["/setactive", guid]).is_some()
}

/// The power-mode registry key (for [`parse_overlay`]).
pub fn overlay_reg() -> Option<String> {
    run("reg", &["query", r"HKLM\SYSTEM\CurrentControlSet\Control\Power\User\PowerSchemes"])
}

/// `powercfg /powerthrottling disable /path <exe>` (needs an elevated prompt): whether it took.
pub fn exempt_from_throttling(exe: &Path) -> bool {
    run("powercfg", &["/powerthrottling", "disable", "/path", &exe.display().to_string()]).is_some()
}

/// Undoes [`exempt_from_throttling`].
pub fn reset_throttling(exe: &Path) {
    let _ = run("powercfg", &["/powerthrottling", "reset", "/path", &exe.display().to_string()]);
}

/// The power state of a chain: switches to a fast plan and exempts the game exe from throttling at start;
/// dropping it undoes both. `notes` say what happened, for the summary.
#[derive(Debug, Default)]
pub struct PowerGuard {
    /// Plan to restore on drop (we switched away from it).
    pub restore: Option<String>,
    /// The plan we switched to (so a stale-lock taker only restores when it is still active).
    pub set: Option<String>,
    /// Exe whose throttling exemption we added.
    pub unthrottled: Option<PathBuf>,
    pub notes: Vec<String>,
}

impl PowerGuard {
    /// Applies the chain's power settings for `exe` (Windows only; a no-op note elsewhere).
    pub fn begin(exe: &Path) -> Self {
        let mut g = Self::default();
        if !cfg!(windows) {
            g.notes.push("power plan: not managed (not Windows)".to_owned());
            return g;
        }
        match plan_action(&schemes()) {
            PlanAction::Keep(s) => g.notes.push(format!("power plan: {} (already a performance plan)", s.name)),
            PlanAction::Switch { from, to } => {
                if set_active(&to.guid) {
                    println!("power plan: {} → {} for the chain", from.name, to.name);
                    g.notes.push(format!("power plan: switched {} → {} for the chain (restored after)", from.name, to.name));
                    g.restore = Some(from.guid);
                    g.set = Some(to.guid);
                } else {
                    g.notes.push(format!("power plan: **{}** (switching to {} failed)", from.name, to.name));
                }
            }
            PlanAction::Missing(active) => g.notes.push(format!(
                "power plan: {} (no High performance / Ultimate plan on this machine; the power mode below is what applies)",
                active.map_or_else(|| "?".to_owned(), |a| a.name)
            )),
        }
        if exempt_from_throttling(exe) {
            g.unthrottled = Some(exe.to_owned());
            g.notes.push("power throttling: exe exempted for the chain (`powercfg /powerthrottling disable`)".to_owned());
        } else {
            g.notes.push("power throttling: exemption skipped (needs an elevated prompt)".to_owned());
        }
        g
    }

    /// Restores what [`PowerGuard::begin`] changed.
    pub fn end(&mut self) {
        if let Some(guid) = self.restore.take() {
            if set_active(&guid) {
                println!("power plan restored");
            } else {
                eprintln!("warning: could not restore power plan {guid} (powercfg /setactive {guid})");
            }
        }
        if let Some(exe) = self.unthrottled.take() {
            reset_throttling(&exe);
        }
    }
}

impl Drop for PowerGuard {
    fn drop(&mut self) {
        self.end();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONLY_BALANCED: &str = "\r\nExisting Power Schemes (* Active)\r\n-----------------------------------\r\nPower Scheme GUID: 381b4222-f694-41f0-9685-ff5bb260df2e  (Balanced) *\r\n";

    #[test]
    fn plans_parse_and_pick_the_fast_one() {
        let only = parse_schemes(ONLY_BALANCED);
        assert_eq!(only, [Scheme { guid: "381b4222-f694-41f0-9685-ff5bb260df2e".into(), name: "Balanced".into(), active: true }]);
        assert_eq!(plan_action(&only), PlanAction::Missing(Some(only[0].clone())));

        let three = format!(
            "{ONLY_BALANCED}Power Scheme GUID: 8C5E7FDA-E8BF-4A96-9A85-A6E23A8C635C  (High performance)\r\n\
             Power Scheme GUID: a1841308-3541-4fab-bc81-f71556f20b4a  (Power saver)\r\n"
        );
        let s = parse_schemes(&three);
        let PlanAction::Switch { from, to } = plan_action(&s) else { panic!("switch") };
        assert_eq!((from.name.as_str(), to.name.as_str()), ("Balanced", "High performance"));

        // A localized High performance plan is found by GUID; an already-fast active plan is kept.
        let local = "Power Scheme GUID: 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c  (Hohe Leistung) *\r\n";
        assert!(matches!(plan_action(&parse_schemes(local)), PlanAction::Keep(_)));
        assert_eq!(plan_action(&[]), PlanAction::Missing(None));
    }

    #[test]
    fn power_mode_overlay_reads_ac_or_battery() {
        let reg = "HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\Power\\User\\PowerSchemes\r\n    ActivePowerScheme    REG_SZ    381b4222-f694-41f0-9685-ff5bb260df2e\r\n    ActiveOverlayAcPowerScheme    REG_SZ    ded574b5-45a0-4f42-8737-46345c09c238\r\n    ActiveOverlayDcPowerScheme    REG_SZ    00000000-0000-0000-0000-000000000000\r\n";
        assert_eq!(parse_overlay(reg, false).as_deref().map(overlay_name).as_deref(), Some("best performance"));
        assert_eq!(parse_overlay(reg, true).as_deref().map(overlay_name).as_deref(), Some("balanced"));
        assert_eq!(parse_overlay("nothing", false), None);
    }
}
