mod easy {
    use std::iter::Zip;

    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        // 1 <= n <= 10^4
        // distance.length == n
        // 0 <= start, destination < n
        // 0 <= distance[i] <= 10^4
        if start == destination {
            return 0;
        }
        let mut count = 0;
        if start < destination {
            for i in start..destination {
                count += distance[i as usize];
            }
        } else {
            for i in destination..start {
                count += distance[i as usize];
            }
        }
        count.min(distance.iter().sum::<i32>() - count)
    }
    pub fn reformat(s: String) -> String {
        let mut nums = Vec::new();
        let mut chars = Vec::new();
        s.chars().into_iter().for_each(|ch| {
            if let Some(_num) = ch.to_digit(10) {
                nums.push(ch);
            } else {
                chars.push(ch);
            }
        });
        if nums.len() == 0 && chars.len() == 1 {
            return chars.iter().collect();
        }
        if nums.len() == 1 && chars.len() == 0 {
            return nums.iter().collect();
        }
        if nums.len() == 0 || chars.len() == 0 {
            return "".to_string();
        }
        if nums.len() == chars.len() {
            nums.iter()
                .zip(&chars)
                .map(|(cx, cy)| vec![cx, cy])
                .flatten()
                .collect()
        } else if nums.len() == chars.len() - 1 {
            let mut res = chars
                .iter()
                .zip(&nums)
                .map(|(cx, cy)| vec![cx, cy])
                .flatten()
                .collect::<String>();
            res.push(*chars.last().unwrap());
            res
        } else if nums.len() == chars.len() + 1 {
            let mut res = nums
                .iter()
                .zip(&chars)
                .map(|(cx, cy)| vec![cx, cy])
                .flatten()
                .collect::<String>();
            res.push(*nums.last().unwrap());
            res
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::easy::{distance_between_bus_stops, reformat};

    #[test]
    fn test_0() {
        assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1), 1);
        assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2), 3);
        assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3), 4);
    }
    #[test]
    fn test_1() {
        assert_eq!(reformat("a0b1c2".to_string()), "0a1b2c".to_string());
        assert_eq!(reformat("leetcode".to_string()), "".to_string());
        assert_eq!(reformat("1229857369".to_string()), "".to_string());
        assert_eq!(reformat("covid2019".to_string()), "c2o0v1i9d".to_string());
        assert_eq!(reformat("ab123".to_string()), "1a2b3".to_string());
    }
    #[test]
    fn test_2() {}
}
