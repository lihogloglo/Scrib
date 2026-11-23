//! [`CalendarView::format`]: a date as text from a pattern the game picks (UI, save menu); names
//! come from the record.

use crate::{CalendarView, Date};

impl CalendarView {
    /// `date` as text: `pattern` with its placeholders replaced. `{d}` day, `{dd}` day on two
    /// digits, `{month}` month name, `{m}` month number (1-based), `{year}` absolute year,
    /// `{yoe}` year of era, `{era}` era name, `{weekday}` weekday name, `{season}` season name,
    /// `{hh}` hour and `{mm}` minute on two digits. A missing name renders empty; any other
    /// text, unknown placeholders included, is kept as written.
    pub fn format(&self, date: &Date, pattern: &str) -> String {
        let mut out = String::with_capacity(pattern.len() + 16);
        let mut rest = pattern;
        while let Some(open) = rest.find('{') {
            out.push_str(&rest[..open]);
            let tail = &rest[open..];
            let Some(close) = tail.find('}') else {
                rest = tail;
                break;
            };
            let name = |n: Option<&str>| n.unwrap_or_default().to_owned();
            let text = match &tail[1..close] {
                "d" => date.day.to_string(),
                "dd" => format!("{:02}", date.day),
                "month" => name(self.month_name(date.month)),
                "m" => (date.month + 1).to_string(),
                "year" => date.year.to_string(),
                "yoe" => date.year_of_era.to_string(),
                "era" => name(date.era.and_then(|e| self.era_name(e))),
                "weekday" => name(date.weekday.and_then(|w| self.weekday_name(w))),
                "season" => name(date.season.and_then(|s| self.season_name(s))),
                "hh" => format!("{:02}", date.hour),
                "mm" => format!("{:02}", date.minute),
                _ => {
                    // Not a placeholder: keep the brace and go on after it.
                    out.push('{');
                    rest = &tail[1..];
                    continue;
                }
            };
            out.push_str(&text);
            rest = &tail[close + 1..];
        }
        out.push_str(rest);
        out
    }
}
