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
        let f = |f_char: &Vec<char>, s_char: &Vec<char>| {
            f_char
                .iter()
                .zip(s_char)
                .map(|(cx, cy)| vec![cx, cy])
                .flatten()
                .collect::<String>()
        };
        if nums.len() == chars.len() {
            f(&nums, &chars)
        } else if nums.len() == chars.len() - 1 {
            let mut res = f(&chars, &nums);
            res.push(*chars.last().unwrap());
            res
        } else if nums.len() == chars.len() + 1 {
            let mut res = f(&nums, &chars);
            res.push(*nums.last().unwrap());
            res
        } else {
            "".to_string()
        }
    }
}
mod medium {
    use std::collections::HashMap;

    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut memo = HashMap::with_capacity(n as usize);
        memo.insert(1, vec![1]);
        fn beautiful_array_f(n: i32, memo: &mut HashMap<i32, Vec<i32>>) -> Vec<i32> {
            if !memo.contains_key(&n) {
                let odds = beautiful_array_f((n + 1) / 2, memo);
                let evens = beautiful_array_f(n / 2, memo);
                memo.insert(n, {
                    odds.iter()
                        .map(|&odd| 2 * odd - 1)
                        .chain(evens.iter().map(|&even| 2 * even))
                        .collect::<Vec<i32>>()
                });
            }
            memo.get(&n).unwrap().iter().map(|&num| num).collect()
        }
        beautiful_array_f(n, &mut memo)
    }
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut vis = vec![false; nums.len()];
        let mut perm = vec![];
        nums.sort();
        fn backtrack(
            nums: &Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
            idx: usize,
            perm: &mut Vec<i32>,
            vis: &mut Vec<bool>,
        ) {
            if idx == nums.len() {
                ans.push((*perm).clone());
                return;
            }
            for i in 0..nums.len() {
                if vis[i] || (i > 0 && nums[i] == nums[i - 1] && !vis[i - 1]) {
                    continue;
                }
                perm.push(nums[i]);
                vis[i] = true;
                backtrack(nums, ans, idx + 1, perm, vis);
                vis[i] = false;
                perm.pop();
            }
        }
        backtrack(&nums, &mut ans, 0, &mut perm, &mut vis);

        ans
    }
}

#[cfg(test)]
mod test {
    use super::{
        easy::{distance_between_bus_stops, reformat},
        medium::{beautiful_array, permute_unique},
    };

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
    fn test_2() {
        assert_eq!(beautiful_array(4), vec![2, 1, 4, 3]);
        assert_eq!(beautiful_array(5), vec![3, 1, 2, 5, 4]);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
        assert_eq!(
            permute_unique(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
        assert_eq!(
            permute_unique(vec![3, 3, 0, 3]),
            vec![
                vec![0, 3, 3, 3],
                vec![3, 0, 3, 3],
                vec![3, 3, 0, 3],
                vec![3, 3, 3, 0]
            ]
        );
    }
}
