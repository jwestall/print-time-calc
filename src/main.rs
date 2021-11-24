mod ptime;

use crate::ptime::ptime::PTime;

fn main() {
    let start_time = PTime::get_ptime("start".to_string());
    let duration = PTime::get_ptime("print".to_string());
    let end_time = compare_time(start_time, duration);

    println!("Your print will finish at {}:{}.", end_time.hour, end_time.min);
}

fn compare_time(start: PTime, duration: PTime) -> PTime {
    let mut end = PTime{
        hour: start.hour + duration.hour,
        min: start.min + duration.min,
        am: start.am
    };

    if end.min > 60 {
        end.hour += 1;
        end.min -= 60;
    }

    if end.hour > 24 {
        end.hour -= 24;
    }
    if end.hour > 12 {
        end.hour -= 12;
        if end.am == true {
            end.am = false;
        } else {
            end.am = true;
        }
    }

    end
}