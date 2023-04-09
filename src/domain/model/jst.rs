use anyhow::{bail, Ok, Result};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Jst;

impl Jst {
    pub(crate) fn offset() -> FixedOffset {
        let hour = 60 * 60;
        let jst_offset = 9 * hour;
        FixedOffset::east_opt(jst_offset).expect("Fail to create east offset time")
    }

    pub(crate) fn now() -> DateTime<FixedOffset> {
        let now_utc = Utc::now().naive_utc();
        Self::offset().from_utc_datetime(&now_utc)
    }

    pub(crate) fn today() -> NaiveDate {
        Self::now().date_naive()
    }

    pub(crate) fn ymd(year: i32, month: u32, day: u32) -> NaiveDate {
        Self::offset()
            .with_ymd_and_hms(year, month, day, 0, 0, 0)
            .unwrap()
            .date_naive()
    }

    pub(crate) fn offset_datetime_from_native_datetime(
        dt: &NaiveDateTime,
    ) -> DateTime<FixedOffset> {
        Self::offset().from_local_datetime(dt).unwrap()
    }

    pub(crate) fn date_from_str(s: &str) -> Result<NaiveDate> {
        let re = Regex::new(r"^(?P<y>\d{4})[/-](?P<m>\d{1,2})[/-](?P<d>\d{1,2})$").unwrap();
        match re.captures(s) {
            Some(caps) => {
                let year = caps["y"].parse::<i32>().unwrap();
                let month = caps["m"].parse::<u32>().unwrap();
                let day = caps["d"].parse::<u32>().unwrap();
                Ok(Self::ymd(year, month, day))
            }
            None => bail!("Invalid format. It must be 'YYYY/MM/DD'. Input is '{}'", s),
        }
    }

    pub(crate) fn timestamp_millis(millis: i64) -> DateTime<FixedOffset> {
        Self::offset().timestamp_millis_opt(millis).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod learning {
        use super::*;
        use chrono::{Datelike, Timelike};

        #[test]
        fn naive_dateは各offsetにおける年月日を指す() {
            let hour = 60 * 60;
            // UTCなら 2022年12月3日 15時になるはず。
            let jst = FixedOffset::east_opt(9 * hour)
                .unwrap()
                .with_ymd_and_hms(2022, 12, 4, 0, 0, 0)
                .unwrap();
            assert_eq!(jst.to_rfc3339(), "2022-12-04T00:00:00+09:00");
            assert_eq!(jst.date_naive().to_string(), "2022-12-04");
            assert_eq!(jst.date_naive().year(), 2022);
            assert_eq!(jst.date_naive().month(), 12);
            assert_eq!(jst.date_naive().day(), 4);
        }

        #[test]
        fn 同時刻ならfixed_offsetにおいてutcとjstで日時が異なる() {
            let utc = Utc.with_ymd_and_hms(2022, 12, 3, 15, 0, 0).unwrap();
            assert_eq!(utc.year(), 2022);
            assert_eq!(utc.month(), 12);
            assert_eq!(utc.day(), 3);
            assert_eq!(utc.hour(), 15);

            let hour = 60 * 60;
            let offset = FixedOffset::east_opt(9 * hour).unwrap();
            let jst_from_utc = offset.from_utc_datetime(&utc.naive_utc());
            assert_eq!(jst_from_utc.year(), 2022);
            assert_eq!(jst_from_utc.month(), 12);
            assert_eq!(jst_from_utc.day(), 4);
            assert_eq!(jst_from_utc.hour(), 0);
        }
        #[test]
        fn 同時刻ならnaive_dateにおいてもutcとjstで年月日が異なる() {
            let utc = Utc.with_ymd_and_hms(2022, 12, 3, 15, 0, 0).unwrap();

            let hour = 60 * 60;
            let offset = FixedOffset::east_opt(9 * hour).unwrap();
            let jst = offset.from_utc_datetime(&utc.naive_utc());

            // utcの場合、2022年12月3日 15:00:00
            assert_eq!(utc.date_naive().year(), 2022);
            assert_eq!(utc.date_naive().month(), 12);
            assert_eq!(utc.date_naive().day(), 3);

            // jstの場合、2022年12月4日 00:00:00
            assert_eq!(jst.date_naive().year(), 2022);
            assert_eq!(jst.date_naive().month(), 12);
            assert_eq!(jst.date_naive().day(), 4);
        }
    }
}
