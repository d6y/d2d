use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Settings {
    #[structopt(short, long, default_value = "2.0")]
    factor: f32,

    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::from_args();

    let file = File::open(settings.input)?;
    let reader = BufReader::new(file);
    let input: Vec<i32> = reader
        .lines()
        .map(Result::unwrap)
        .flat_map(|v| v.parse())
        .collect();

    let doubles = doubling_time(settings.factor, &input);
    for d in doubles {
        println!("{:}", d);
    }

    Ok(())
}

fn doubling_time(factor: f32, ts: &[i32]) -> Vec<isize> {
    let scan_forward = |t_index, &t| -> Option<isize> {
        let doubling: i32 = ((t as f32) * factor).round() as i32;
        let halfing: i32 = ((t as f32) / factor).round() as i32;
        let start = t_index + 1;
        num_steps(doubling, halfing, &ts[start..])
    };

    let step_count = |(t_index, t)| -> isize { scan_forward(t_index, t).unwrap_or(0) };

    ts.iter().enumerate().map(step_count).collect()
}

fn num_steps(min_increase: i32, min_decrease: i32, ts: &[i32]) -> Option<isize> {
    let detection = |(index, &value)| -> Option<isize> {
        if value >= min_increase {
            Some(1 + index as isize)
        } else if value <= min_decrease {
            Some(-(1 + index as isize))
        } else {
            None
        }
    };

    ts.iter().enumerate().filter_map(detection).next()
}

#[cfg(test)]
mod tests {
    use super::doubling_time;
    #[test]
    fn exact_doubling() {
        let input = vec![1, 2, 4, 8, 16];
        let steps = vec![1, 1, 1, 1];
        assert_eq!(doubling_time(2.0, &input), steps);
    }

    #[test]
    fn no_doubling() {
        let input = vec![10, 11, 12, 13, 14, 15];
        let steps = vec![];
        assert_eq!(doubling_time(2.0, &input), steps);
    }

    #[test]
    fn delay_doubling() {
        let input = vec![10, 14, 16, 18, 20, 30, 40];
        let steps = vec![4, 4, 4, 3, 2];
        assert_eq!(doubling_time(2.0, &input), steps);
    }

    #[test]
    fn exact_halfing() {
        let mut input = vec![1, 2, 4, 8, 16];
        input.reverse();
        let steps = vec![-1, -1, -1, -1];
        assert_eq!(doubling_time(2.0, &input), steps);
    }
}
