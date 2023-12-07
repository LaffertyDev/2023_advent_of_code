pub struct Race {
    series: Vec<Series>
}

struct Series {
    series_duration_ms: u64,
    current_distance_record_ms: u64
}

impl Race {
    pub fn parse(sheet: &str) -> Race {
        let mut lines = sheet.lines().filter(|l| !l.is_empty());
        let mut times = lines.next().unwrap().split_whitespace();
        let mut distances = lines.next().unwrap().split_whitespace();
        times.next().unwrap();
        distances.next().unwrap();

        let series = times.zip(distances).map(|(time, distance)| Series {
            series_duration_ms: time.parse().unwrap(),
            current_distance_record_ms: distance.parse().unwrap()
        }).collect();
        Race {
            series: series
        }
    }

    pub fn parse_as_single(sheet: &str) -> Race {
        let mut lines = sheet.lines().filter(|l| !l.is_empty());
        let mut time = lines.next().unwrap().split_whitespace().skip(1).collect::<String>().parse::<u64>().unwrap();
        let mut distance = lines.next().unwrap().split_whitespace().skip(1).collect::<String>().parse::<u64>().unwrap();

        Race {
            series: vec![Series {
                series_duration_ms: time,
                current_distance_record_ms: distance
            }]
        }
    }

    pub fn get_number_of_winning_moves(&self) -> usize {
        self.series.iter().map(|series| series.get_durations_to_beat_distance().len()).product()
    }
}

impl Series {
    pub fn get_durations_to_beat_distance(&self) -> Vec<u64> {
        // position = v0 * t + 1/2 * a * t^2
        // position = (v + v0)/2 * t
        // 2 * pos = (v + v0) * t
        // 2 * pos / (v + v0) = t
        // 2*pos / v = t

        // maximum states is time
        let mut times_that_could_win = vec![];
        for time_held in 0..self.series_duration_ms {
            // compute distance travelled
            let time_remaining = self.series_duration_ms - time_held;
            let velocity_gained = time_held;
            let distance_moved = velocity_gained * time_remaining;

            if distance_moved > self.current_distance_record_ms {
                // this one will win!
                times_that_could_win.push(time_held);
            }

        }

        times_that_could_win
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day6::race::Race;

    #[test]
    fn input_passes() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let race = Race::parse(input);
        assert_eq!(288, race.get_number_of_winning_moves())
    }

    #[test]
    fn input_single_race_passes() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let race = Race::parse_as_single(input);
        assert_eq!(71503, race.get_number_of_winning_moves())
    }
}