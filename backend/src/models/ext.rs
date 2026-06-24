use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::wrapper::{Column, ColumnRes, DayRes, TimeTable};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Full Code")]
    pub full_code: String,
    #[serde(rename = "Subject")]
    pub subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaySchedule {
    #[serde(flatten)]
    pub time_slots: HashMap<String, Vec<String>>,
}
impl DaySchedule {
    pub fn transform(&self) ->Vec<ColumnRes> {
        let mut cols: Vec<ColumnRes> = vec![];
        for i in &self.time_slots {
            let (date,slots) = i;
            let col = ColumnRes::frm_json(date,slots);
            cols.push(col);
        }
        cols
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtTimetable {
    #[serde(rename = "MON")]
    pub monday: DaySchedule,
    #[serde(rename = "TUES")]
    pub tuesday: DaySchedule,
    #[serde(rename = "WED")]
    pub wednesday: DaySchedule,
    #[serde(rename = "THUR")]
    pub thursday: DaySchedule,
    #[serde(rename = "FRI")]
    pub friday: DaySchedule,
    #[serde(rename = "SAT")]
    pub saturday:DaySchedule,
}

impl ExtTimetable {
    pub fn transform(&self) ->TimeTable {
        let mut days: Vec<DayRes> = vec![];

        let mon = DayRes {
            day: 0,
            cols: self.monday.transform(),
        };
        days.push(mon);

        let tues = DayRes {
            day: 1,
            cols: self.tuesday.transform(),
        };
        days.push(tues);

        let wed = DayRes {
            day: 2,
            cols: self.wednesday.transform(),
        };
        days.push(wed);

        let thurs = DayRes {
            day: 3,
            cols: self.thursday.transform(),
        };
        days.push(thurs);

        let fri = DayRes {
            day: 4,
            cols: self.friday.transform(),
        };
        days.push(fri);

        let sat = DayRes {
            day: 5,
            cols: self.saturday.transform(),
        };
        days.push(sat);
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearData {
    pub timetable: ExtTimetable,
    pub subjects: Vec<Subject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimetableData {
    #[serde(flatten)]
    pub years: HashMap<String, YearData>,
}


// Helper methods for easier access
impl TimetableData {
    pub fn get_year_data(&self, year: &str) -> Option<&YearData> {
        self.years.get(year)
    }

    pub fn get_all_years(&self) -> Vec<&String> {
        self.years.keys().collect()
    }
}

impl YearData {
    pub fn get_subject_by_code(&self, code: &str) -> Option<&Subject> {
        self.subjects.iter().find(|s| s.code == code)
    }

    pub fn get_day_schedule(&self, day: &str) -> Option<&DaySchedule> {
        match day.to_uppercase().as_str() {
            "MON" | "MONDAY" => Some(&self.timetable.monday),
            "TUES" | "TUESDAY" => Some(&self.timetable.tuesday),
            "WED" | "WEDNESDAY" => Some(&self.timetable.wednesday),
            "THUR" | "THURSDAY" => Some(&self.timetable.thursday),
            "FRI" | "FRIDAY" => Some(&self.timetable.friday),
            "SAT" | "SATURDAY" => Some(&self.timetable.saturday),
            _ => None,
        }
    }
}

impl DaySchedule {
    pub fn get_classes_at_time(&self, time_slot: &str) -> Option<&Vec<String>> {
        self.time_slots.get(time_slot)
    }

    pub fn get_all_time_slots(&self) -> Vec<&String> {
        self.time_slots.keys().collect()
    }
}


// Catatan refactoring: fungsi `parse_time_block`, `parse_time`, dan struct
// `TimeInfo` yang sebelumnya ada di sini telah dihapus karena merupakan dead
// code (tidak dipanggil di mana pun) dan logic-nya terduplikasi dengan
// `models::time_parser`. Lihat `models/time_parser.rs` untuk implementasi
// yang dipakai sekarang.

