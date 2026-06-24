//! Modul utilitas untuk parsing format waktu "HH:MM AM/PM".
//!
//! Sebelumnya logic ini terduplikasi di `wrapper.rs` (ColumnRes::get_start_time,
//! ColumnRes::get_duration) dan `ext.rs` (parse_time_block, parse_time) dengan
//! dua gaya implementasi berbeda — satu menggunakan `.unwrap()` (rawan panic),
//! satu menggunakan `Option`/`?` (aman). Modul ini menyatukan keduanya menjadi
//! satu sumber kebenaran yang aman, sebagai bagian dari refactoring kualitas kode.

/// Parse satu titik waktu (misal "9:00 AM") menjadi (jam_24, menit).
/// Mengembalikan `None` jika format tidak valid, alih-alih panic.
pub fn parse_time(time_str: &str) -> Option<(u32, u32)> {
    let time_str = time_str.to_uppercase();
    let is_pm = time_str.contains("PM");
    let is_am = time_str.contains("AM");

    let time_clean = time_str.replace("AM", "").replace("PM", "");
    let parts: Vec<&str> = time_clean.trim().split(':').collect();

    if parts.len() != 2 {
        return None;
    }

    let hour = parts[0].trim().parse::<u32>().ok()?;
    let minute = parts[1].trim().parse::<u32>().ok()?;

    if hour > 12 || minute >= 60 {
        return None;
    }

    let hour24 = if hour == 12 {
        if is_am { 0 } else { 12 }
    } else if is_pm {
        hour + 12
    } else {
        hour
    };

    Some((hour24, minute))
}

/// Parse rentang waktu (misal "9:00 AM - 10:00 AM") menjadi (jam_mulai, durasi_menit).
/// Menangani kasus saat AM/PM hanya disebut di akhir string (mis. "9:00 - 10:00 AM"),
/// dan mendukung rentang yang melewati tengah malam.
/// Mengembalikan `None` jika format tidak valid, alih-alih panic.
pub fn parse_time_range(input: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 2 {
        return None;
    }

    let start_str = parts[0].trim();
    let end_str = parts[1].trim();

    let end_upper = end_str.to_uppercase();
    let has_am_pm_at_end = end_upper.contains("AM") || end_upper.contains("PM");
    let start_has_am_pm = start_str.to_uppercase().contains("AM") || start_str.to_uppercase().contains("PM");

    let (start_time, end_time) = if has_am_pm_at_end && !start_has_am_pm {
        let am_pm_suffix = if end_upper.contains("AM") { " AM" } else { " PM" };
        let start_with_suffix = format!("{}{}", start_str, am_pm_suffix);
        (parse_time(&start_with_suffix)?, parse_time(end_str)?)
    } else {
        (parse_time(start_str)?, parse_time(end_str)?)
    };

    let start_minutes = start_time.0 * 60 + start_time.1;
    let end_minutes = end_time.0 * 60 + end_time.1;

    let duration = if end_minutes >= start_minutes {
        end_minutes - start_minutes
    } else {
        // Rentang melewati tengah malam
        (24 * 60) - start_minutes + end_minutes
    };

    Some((start_time.0, duration))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_am_time() {
        assert_eq!(parse_time("9:00 AM"), Some((9, 0)));
    }

    #[test]
    fn parses_simple_pm_time() {
        assert_eq!(parse_time("9:00 PM"), Some((21, 0)));
    }

    #[test]
    fn parses_noon_and_midnight_edge_cases() {
        assert_eq!(parse_time("12:00 PM"), Some((12, 0)));
        assert_eq!(parse_time("12:00 AM"), Some((0, 0)));
    }

    #[test]
    fn rejects_invalid_format_instead_of_panicking() {
        assert_eq!(parse_time("not a time"), None);
        assert_eq!(parse_time("25:00 AM"), None);
        assert_eq!(parse_time("9:99 AM"), None);
    }

    #[test]
    fn parses_time_range_with_am_pm_on_both_sides() {
        // 9:00 AM - 10:00 AM => mulai jam 9, durasi 60 menit
        assert_eq!(parse_time_range("9:00 AM - 10:00 AM"), Some((9, 60)));
    }

    #[test]
    fn parses_time_range_with_am_pm_only_at_end() {
        // "9:00 - 10:00 AM" => AM diterapkan juga ke waktu mulai
        assert_eq!(parse_time_range("9:00 - 10:00 AM"), Some((9, 60)));
    }

    #[test]
    fn parses_time_range_crossing_midnight() {
        // 11:00 PM - 1:00 AM => durasi 2 jam melewati tengah malam
        assert_eq!(parse_time_range("11:00 PM - 1:00 AM"), Some((23, 120)));
    }

    #[test]
    fn rejects_malformed_range_instead_of_panicking() {
        assert_eq!(parse_time_range("not a valid range"), None);
        assert_eq!(parse_time_range("9:00 AM"), None); // tidak ada tanda '-'
    }
}
