use chrono::Duration;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct TaskDuration(Duration);

impl TaskDuration {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }
    pub fn as_duration(&self) -> Duration {
        self.0
    }
    pub fn add(&self, other: TaskDuration) -> Self {
        Self(self.as_duration() + other.as_duration())
    }
}

impl From<Option<&str>> for TaskDuration {
    fn from(millisec_str: Option<&str>) -> Self {
        let millisec = match millisec_str {
            Some(d) => d.parse::<i64>().unwrap(),
            None => 0,
        };
        Self(Duration::milliseconds(millisec))
    }
}

impl std::fmt::Display for TaskDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.as_duration().is_zero() {
            // write!(f, "n/a")
            write!(f, "00:00:00")
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

    fn gen_task_duration(hour: i32, minute: i32, sec: i32) -> TaskDuration {
        let millisec = ((hour * 60 * 60) + (minute * 60) + sec) * 1000;
        TaskDuration::from(Some(millisec.to_string().as_str()))
    }

    #[test]
    fn to_stringでhhmiss形式の文字列に変換される() {
        let d = gen_task_duration(2, 30, 45);
        println!("{}", d);
        assert_eq!(d.to_string(), "02:30:45".to_string());
    }

    #[test]
    fn 秒数が60を超えたら分数に繰り上がる() {
        let d = gen_task_duration(2, 30, 75);
        println!("{}", d);
        assert_eq!(d.to_string(), "02:31:15".to_string());
    }

    #[test]
    fn 分数が60を超えたら時間数に繰り上がる() {
        let d = gen_task_duration(2, 75, 45);
        println!("{}", d);
        assert_eq!(d.to_string(), "03:15:45".to_string());
    }

    #[test]
    fn 時間数が24を超えても繰り上がらない() {
        let d = gen_task_duration(99, 30, 45);
        println!("{}", d);
        assert_eq!(d.to_string(), "99:30:45".to_string());
    }
}
