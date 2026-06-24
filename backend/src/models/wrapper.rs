use std::collections::HashMap;
use std::io::ErrorKind;
use serde::{Deserialize, Serialize};
use crate::models::main_data::{Slot,SlotRes};
use crate::models::meta_data::TimeTableMetaData;
use bcrypt::{hash, verify, DEFAULT_COST,BcryptError};

#[derive(Serialize,Deserialize,Debug,Default, Clone)]
pub struct TimeStamp{
    pub hr:u8,
    pub min:u8,
}
#[derive(Serialize,Deserialize,Debug,Default, Clone)]
pub struct Column{
    pub start_time:TimeStamp,
    pub duration:u16,
    pub schedules:Vec<Slot>,
}

#[derive(Serialize,Deserialize,Debug,Default, Clone)]
pub struct ColumnRes{
    pub start_time:TimeStamp,
    pub duration:u16,
    pub schedules:Vec<SlotRes>,
}

impl ColumnRes{
    pub fn transform(&self , meta_data:&TimeTableMetaData) -> Column{
        let mut new_slots = Vec::new();
        for i in &self.schedules {
            new_slots.push(i.transform(meta_data));
        }
        Column{
            start_time: self.start_time.clone(),
            duration: self.duration,
            schedules: new_slots,
        }
    }

    /// Catatan refactoring: sebelumnya fungsi ini (dan `get_duration`) memiliki
    /// implementasi parsing waktu sendiri yang terduplikasi dengan
    /// `models::ext::parse_time_block`, dan menggunakan `.unwrap()` yang bisa
    /// menyebabkan panic jika format input tidak sesuai. Logic ini sekarang
    /// didelegasikan ke `models::time_parser`, satu sumber kebenaran yang aman
    /// (mengembalikan default 0 jika parsing gagal, alih-alih crash).
    fn get_start_time(time: &str) -> u16 {
        crate::models::time_parser::parse_time_range(time)
            .map(|(start_hour, _duration)| start_hour as u16)
            .unwrap_or(0)
    }

    fn get_duration(time: &str) -> u16 {
        crate::models::time_parser::parse_time_range(time)
            .map(|(_start_hour, duration)| duration as u16)
            .unwrap_or(0)
    }

    pub fn frm_json(time : &str , slots: &Vec<String>) ->Self {
        let start_time = TimeStamp{
            hr : ColumnRes::get_start_time(time) as u8,
            min: 0
        };
        let duration = ColumnRes::get_duration(time);
        let mut schedules:Vec<SlotRes> = vec![];
        for s in slots {
            let res = SlotRes::frm_str(s);
            schedules.push(res);
        }
        Self{
            start_time,
            duration,
            schedules
        }
    }
}


#[derive(Serialize,Deserialize,Debug,Default, Clone)]
pub struct Day {
    pub day:u8,
    pub cols: Vec<Column>,
}
#[derive(Serialize,Deserialize,Debug,Default, Clone)]
pub struct DayRes {
    pub day:u8,
    pub cols: Vec<ColumnRes>,
}
impl DayRes{
    pub fn transform(&self, meta_data:&TimeTableMetaData) -> Day{
        let mut new_slots = Vec::new();
        for i in &self.cols {
            new_slots.push(i.transform(meta_data));
        }
        Day{
            day: self.day,
            cols: new_slots,
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Default, Clone)]
pub struct TimeTable{
    year:u8,
    days:Vec<Day>,
}

#[derive(Serialize,Deserialize,Debug,Default, Clone)]
pub struct Collection{
    timetables:Vec<TimeTable>,
}
impl Collection{
    pub fn get_timetables(&self) -> Vec<TimeTable>{
        self.timetables.clone()
    }
}

#[derive(Serialize,Deserialize,Debug,Default, Clone)]
pub struct TimeTableRes{
    ver:u8,
    days:Vec<DayRes>,
}
impl TimeTableRes{
    pub fn transform(&self, meta_data:&TimeTableMetaData) -> TimeTable{
        let mut new_slots = Vec::new();
        for i in &self.days {
            new_slots.push(i.transform(meta_data));
        }
        TimeTable{
            year: self.ver,
            days: new_slots,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MetaData{
    pub version:u16,
    pub data: TimeTableMetaData
}
impl MetaData {
    pub fn merge(&self , new:&TimeTableMetaData) -> MetaData {
        MetaData{
            version: self.version + 1,
            data: self.data.merge(new)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Res {
    pub key: String,
    pub timetable: Option<TimeTableRes>,
}

impl Res {
    pub fn transform(&self, meta_data: &TimeTableMetaData) -> Result<TimeTable, BcryptError> {
        let stored_hash = "$2b$12$p6iy1Fciwj.IasMAVBEhOODdgfoQZx3vFsiP2m8Uql.sA9Cc9/e9W";

        if verify(&self.key, &stored_hash)? {
            if let Some(timetable) = &self.timetable {
                Ok(timetable.transform(meta_data))
            } else {
                Err(BcryptError::Io(std::io::Error::new(ErrorKind::InvalidData,"Bad Data"))) 
            }
        } else {
            Err(BcryptError::Io(std::io::Error::new(ErrorKind::InvalidData,"Bad Auth")))
        }
    }
}

fn gen_hash(password:&str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn get_hash() {
        let a =gen_hash("Pass").unwrap();
        dbg!(a);
        assert_eq!(2,2);
    }
    #[test]
    fn test_verify() {
        let stored_hash = "$2b$12$YtQs1d9.s3GX8KP3GoY13OEOmo.Z2lPl/wn0ZHK4KEUkcs6UD57h2".to_string();
        let a = verify("Pass",&stored_hash);
        assert!(a.is_ok() && a.unwrap());
    }
}