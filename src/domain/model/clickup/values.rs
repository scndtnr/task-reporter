use chrono::Duration;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClickupDuration(Duration);

impl ClickupDuration {
    /// millisec単位の文字列を受け取る
    pub fn new(millisec_str: Option<&str>) -> Self {
        let millisec = match millisec_str {
            Some(d) => d.parse::<i64>().unwrap(),
            None => 0,
        };
        Self::validate(millisec);
        Self(Duration::milliseconds(millisec))
    }
    pub fn as_duration(&self) -> Duration {
        self.0
    }
    pub fn add(&self, other: ClickupDuration) -> Self {
        Self(self.as_duration() + other.as_duration())
    }

    // millisec単位で検証し、100時間を超えていたらエラーを返す
    fn validate(millisec: i64) {
        let hundred_hours_millisec = 100 * 60 * 60 * 1000;
        if hundred_hours_millisec <= millisec {
            panic!("Too many milliseconds. Please specify less than 100 hours.")
        };
    }
}

impl From<Duration> for ClickupDuration {
    fn from(from: Duration) -> Self {
        Self::validate(from.num_milliseconds());
        Self(from)
    }
}

impl std::fmt::Display for ClickupDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.as_duration().is_zero() {
            write!(f, "")
        } else {
            let sec = self.as_duration().num_seconds() % (60);
            let minute = self.as_duration().num_minutes() % (60);
            let hour = self.as_duration().num_hours();
            write!(f, "{:>02}:{:>02}:{:>02}", hour, minute, sec)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_clickup_duration(hour: i32, minute: i32, sec: i32) -> ClickupDuration {
        let millisec = ((hour * 60 * 60) + (minute * 60) + sec) * 1000;
        ClickupDuration::new(Some(millisec.to_string().as_str()))
    }

    #[test]
    fn to_stringでhhmiss形式の文字列に変換される() {
        let d = gen_clickup_duration(2, 30, 45);
        println!("{}", d);
        assert_eq!(d.to_string(), "02:30:45".to_string());
    }

    #[test]
    fn 秒数が60を超えたら分数に繰り上がる() {
        let d = gen_clickup_duration(2, 30, 75);
        println!("{}", d);
        assert_eq!(d.to_string(), "02:31:15".to_string());
    }

    #[test]
    fn 分数が60を超えたら時間数に繰り上がる() {
        let d = gen_clickup_duration(2, 75, 45);
        println!("{}", d);
        assert_eq!(d.to_string(), "03:15:45".to_string());
    }

    #[test]
    fn 時間数が24を超えても繰り上がらない() {
        let d = gen_clickup_duration(99, 30, 45);
        println!("{}", d);
        assert_eq!(d.to_string(), "99:30:45".to_string());
    }

    #[test]
    #[should_panic = "Too many milliseconds. Please specify less than 100 hours."]
    fn 時間数が100を超えたらエラーになる() {
        let d = gen_clickup_duration(100, 30, 45);
        println!("{}", d);
    }
}
