extern crate chrono;
extern crate regex;

use std::str::FromStr;

use super::Jst;
use anyhow::{bail, Error, Ok, Result};
use chrono::{DateTime, Duration, FixedOffset, NaiveDate, NaiveTime, Timelike};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateRange {
    start_dt: StartDateTime,
    end_dt: EndDateTime,
}

impl DateRange {
    pub fn new<T: Into<String>>(start_date: Option<T>, end_date: Option<T>) -> Self {
        // 日付文字列のパース
        let start = match start_date {
            Some(date) => TargetDate::from_str(&date.into()).unwrap(),
            None => TargetDate::new(),
        };
        let end = match end_date {
            Some(date) => TargetDate::from_str(&date.into()).unwrap(),
            None => start.clone(),
        };

        // 始端日が終端日よりも前であるか検証する
        Self::validate_start_under_end(&start, &end).unwrap();

        // DateTimeの生成
        let start_dt = StartDateTime::from(start.0);
        let end_dt = EndDateTime::from(end.0);

        // インスタンスを返す
        Self { start_dt, end_dt }
    }
    fn validate_start_under_end(start: &TargetDate, end: &TargetDate) -> Result<()> {
        if end < start {
            bail!(
                "Start date must be before end date. \nInput start: {}, \nInput end  : {}",
                start.0,
                end.0
            )
        } else {
            Ok(())
        }
    }
    pub fn start(&self) -> DateTime<FixedOffset> {
        self.start_dt.0
    }
    pub fn end(&self) -> DateTime<FixedOffset> {
        self.end_dt.0
    }
    pub fn start_date_str(&self) -> String {
        self.start_dt.0.format("%Y/%m/%d").to_string()
    }
    pub fn start_datetime_str(&self) -> String {
        self.start_dt.0.format("%Y/%m/%dT%H:%M:%S").to_string()
    }
    pub fn start_unixtime_sec(&self) -> i64 {
        self.start_dt.0.timestamp()
    }
    pub fn start_unixtime_millis(&self) -> i64 {
        self.start_dt.0.timestamp_millis()
    }
    pub fn end_date_str(&self) -> String {
        self.end_dt.0.format("%Y/%m/%d").to_string()
    }
    pub fn end_datetime_str(&self) -> String {
        self.end_dt.0.format("%Y/%m/%dT%H:%M:%S").to_string()
    }
    pub fn end_unixtime_sec(&self) -> i64 {
        self.end_dt.0.timestamp()
    }
    pub fn end_unixtime_millis(&self) -> i64 {
        self.end_dt.0.timestamp_millis()
    }
    /// 指定の始端日～終端日からNaiveDateのベクトルを得る
    /// ※終端日は指定通りにするため-1日の補正をかけている
    /// e.g. vec!["2022/10/19".to_string(), "2022/10/20".to_string()]
    pub fn vec_dates_str(&self) -> Vec<NaiveDate> {
        let start_date = self.start_dt.0.date_naive();
        let end_date = self.end_dt.0.date_naive() + Duration::days(-1);
        start_date
            .iter_days()
            .take_while(|date| date <= &end_date)
            // .map(|date| date.format("%Y/%m/%d").to_string())
            .collect()
    }

    /// 対象のDateTimeをNaiveDateに変換する
    /// ただし、1日の始まりは午前5時とする
    pub(crate) fn convert_datetime_to_date(dt: DateTime<FixedOffset>) -> NaiveDate {
        if dt.hour() >= 5 {
            dt.date_naive()
        } else {
            dt.date_naive() - Duration::days(-1)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TargetDate(NaiveDate);

impl TargetDate {
    fn new() -> Self {
        // 午前5時以降なら今日、以前なら昨日として出力する
        let border = NaiveTime::from_hms_opt(5, 0, 0).unwrap();
        if border < Jst::now().time() {
            Self(Jst::today())
        } else {
            Self(Jst::today() + Duration::days(-1))
        }
    }
}

impl FromStr for TargetDate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_jst = Jst::date_from_str(s)?;
        Ok(Self(date_jst))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct StartDateTime(DateTime<FixedOffset>);

impl From<NaiveDate> for StartDateTime {
    fn from(from: NaiveDate) -> Self {
        let local = from.and_hms_opt(5, 0, 0).unwrap();
        Self(Jst::offset_datetime_from_native_datetime(&local))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct EndDateTime(DateTime<FixedOffset>);

impl From<NaiveDate> for EndDateTime {
    fn from(from: NaiveDate) -> Self {
        let local = from.and_hms_opt(4, 59, 59).unwrap();
        let local_tomorrow = local + Duration::days(1);
        Self(Jst::offset_datetime_from_native_datetime(&local_tomorrow))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::{Duration, Local, NaiveTime};

    mod test_of_target_date_time_by_single_date {
        use super::*;

        #[test]
        fn create_with_some_start_date() {
            let dt = DateRange::new(Some("2012/12/31"), None);
            assert_eq!(dt.start().date_naive(), Jst::ymd(2012, 12, 31));
        }

        #[test]
        fn create_with_no_start_date() {
            let dt = DateRange::new::<&str>(None, None);

            // 時間帯によって対象日が変化するが、
            // それはTargetDateの責務なので、ここでは簡単に済ませる。
            assert_eq!(dt.start().date_naive(), TargetDate::new().0)
        }

        #[test]
        fn compare_target_date_time() {
            let past_dt = DateRange::new(Some("2012/12/31"), None);
            let current_dt = DateRange::new::<&str>(None, None);
            let future_dt = DateRange::new(Some("2222/02/22"), None);

            assert!(past_dt == past_dt);
            assert!(current_dt == current_dt);
            assert!(future_dt == future_dt);

            assert!(past_dt < current_dt);
            assert!(past_dt < future_dt);
            assert!(current_dt < future_dt);
        }

        #[test]
        fn start_less_than_end() {
            let dt = DateRange::new::<&str>(None, None);
            assert!(dt.start() < dt.end());
        }

        #[test]
        #[should_panic = "assertion failed: dt.end() < dt.start()"]
        fn end_less_than_start() {
            let dt = DateRange::new::<&str>(None, None);
            assert!(dt.end() < dt.start());
        }
    }

    mod test_of_target_date_time_by_period {
        use chrono::TimeZone;

        use super::*;

        #[test]
        fn create_with_some_dates() {
            let dt = DateRange::new(Some("2010/12/31"), Some("2022/07/13"));

            assert_eq!(
                dt.start(),
                Jst::offset()
                    .with_ymd_and_hms(2010, 12, 31, 5, 0, 0)
                    .unwrap()
            );
            assert_eq!(
                dt.end(),
                Jst::offset()
                    .with_ymd_and_hms(2022, 7, 14, 4, 59, 59)
                    .unwrap()
            );
        }

        #[test]
        #[should_panic = "Start date must be before end date."]
        fn end_less_than_start() {
            let _ = DateRange::new(Some("2010/12/31"), Some("2010/12/30"));
        }
    }

    mod test_of_target_date {
        use super::*;

        #[test]
        fn date_str_has_slash() {
            let dt = TargetDate::from_str("2021/01/01").unwrap();
            dbg!(&dt);
        }

        #[test]
        fn date_str_has_hyphen() {
            let dt = TargetDate::from_str("2021-01-01").unwrap();
            dbg!(&dt);
        }

        #[test]
        fn date_str_has_one_charactor_date() {
            let dt = TargetDate::from_str("2021/01/1").unwrap();
            dbg!(&dt);
        }

        #[test]
        fn date_str_has_one_charactor_month() {
            let dt = TargetDate::from_str("2021/1/01").unwrap();
            dbg!(&dt);
        }

        #[test]
        #[should_panic = "Invalid format. It must be 'YYYY/MM/DD'. Input is '12/10/2021'"]
        fn date_str_is_incorrect_format() {
            let dt = TargetDate::from_str("12/10/2021").unwrap();
            dbg!(&dt);
        }

        #[test]
        #[should_panic = "No such local time"]
        fn date_str_has_incorrect_numbers() {
            let dt = TargetDate::from_str("2022/13/32").unwrap();
            dbg!(&dt);
        }

        #[test]
        fn check_date() {
            let target_date = TargetDate::new();
            let today = Local::now().date_naive();
            let yesterday = today + Duration::days(-1);
            let border = NaiveTime::from_hms_opt(5, 0, 0).unwrap();

            if border < Local::now().time() {
                assert_eq!(target_date.0, today);
            } else {
                assert_eq!(target_date.0, yesterday);
            }
        }
    }

    mod test_of_target_start_date_time {
        use super::*;

        #[test]
        fn check_start_date() {
            let today = Jst::today();
            let start = StartDateTime::from(today);
            assert_eq!(start.0.date_naive(), today);
        }

        #[test]
        fn check_start_hhmiss() {
            let start = StartDateTime::from(Jst::today());
            assert_eq!(start.0.time(), NaiveTime::from_hms_opt(5, 0, 0).unwrap());
        }
    }

    mod test_of_target_end_date_time {
        use super::*;

        #[test]
        fn check_end_date() {
            let today = Jst::today();
            let end = EndDateTime::from(today);
            assert_eq!(end.0.date_naive(), today + Duration::days(1));
        }

        #[test]
        fn check_end_hhmiss() {
            let end = EndDateTime::from(Jst::today());
            assert_eq!(end.0.time(), NaiveTime::from_hms_opt(4, 59, 59).unwrap());
        }
    }

    mod test_of_date_range {
        use super::*;

        #[test]
        fn check_vec_days_by_single_date() {
            let date_range = DateRange::new(Some("2012/12/31"), None);
            dbg!(date_range.vec_dates_str());
        }

        #[test]
        fn check_vec_days_by_period() {
            let date_range = DateRange::new(Some("2012/12/31"), Some("2013/01/02"));
            dbg!(date_range.vec_dates_str());
        }
    }
}
