mod easy {
    use crate::structs::list_node::ListNode;
    pub fn kth_to_last(mut head: Option<Box<ListNode>>, k: i32) -> i32 {
        let mut vec = Vec::new();

        loop {
            match head.take() {
                Some(node) => {
                    vec.push(node.val);
                    head = node.next;
                }
                None => {
                    for _ in 1..k {
                        vec.pop();
                    }
                    return vec.pop().unwrap();
                }
            }
        }
    }
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut i = 0;
        loop {
            if num == 0 {
                break;
            }
            if num % 2 == 0 {
                num = num / 2;
                i += 1;
            } else {
                num = num - 1;
                i += 1;
            }
        }

        i
    }
}
mod medium {
    use std::collections::HashMap;

    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut swap_x_count = 0;
        let mut swap_y_count = 0;
        let len = s1.len();
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        for i in 0..len {
            if s1[i] != s2[i] {
                if s1[i] == 'x' {
                    swap_x_count += 1;
                } else {
                    swap_y_count += 1;
                }
            }
        }
        if ((swap_x_count + swap_y_count) & 1) == 1 {
            -1
        } else {
            (swap_x_count + 1) / 2 + (swap_y_count + 1) / 2
        }
    }
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut type_1 = HashMap::with_capacity(len1);
        let mut type_2 = HashMap::with_capacity(len2);
        nums1.iter().for_each(|num| {
            let k = (*num as u64).pow(2);
            type_1.entry(k).and_modify(|count| *count += 1).or_insert(1);
        });
        nums2.iter().for_each(|num| {
            let k = (*num as u64).pow(2);
            type_2.entry(k).and_modify(|count| *count += 1).or_insert(1);
        });
        let mut res = 0;
        for i in 0..len2 {
            for j in i + 1..len2 {
                if let Some(&count) = type_1.get(&(nums2[i] as u64 * nums2[j] as u64)) {
                    res += count;
                }
            }
        }
        for i in 0..len1 {
            for j in i + 1..len1 {
                if let Some(&count) = type_2.get(&(nums1[i] as u64 * nums1[j] as u64)) {
                    res += count;
                }
            }
        }
        res
    }

    pub fn num_traiplets_offcal(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut map1 = HashMap::with_capacity(len1);
        let mut map2 = HashMap::with_capacity(len2);
        nums1.iter().for_each(|num| {
            map1.entry(num).and_modify(|count| *count += 1).or_insert(1);
        });
        nums2.iter().for_each(|num| {
            map2.entry(num).and_modify(|count| *count += 1).or_insert(1);
        });
        get_triplets(&map1, &map2) + get_triplets(&map2, &map1)
    }
    fn get_triplets(map1: &HashMap<&i32, i32>, map2: &HashMap<&i32, i32>) -> i32 {
        let mut triplets = 0;
        for (k1, v1) in map1.iter() {
            let square: u64 = (**k1 as u64).pow(2);
            for (k2, v2) in map2.iter() {
                let num2 = **k2 as u64;
                if square % num2 == 0 {
                    let num3 = square / num2;
                    if num2 == num3 {
                        let cur_traiplets = (*v1) * (*v2) * (*v2 - 1) / 2;
                        triplets += cur_traiplets;
                    } else if num2 < num3 {
                        if let Some(v3) = map2.get(&&(num3 as i32)) {
                            let cur_traiplets = (*v1) * (*v2) * (*v3);
                            triplets += cur_traiplets;
                        }
                    }
                }
            }
        }
        triplets
    }
}

#[cfg(test)]
mod test {
    use crate::list;

    use super::{
        easy::{kth_to_last, number_of_steps},
        medium::{minimum_swap, num_triplets},
    };

    #[test]
    fn test_0() {
        let list = list!(1, 2, 3, 4, 5);
        let val = kth_to_last(list, 2);
        assert_eq!(val, 4);
    }
    #[test]
    fn test_1() {
        assert_eq!(number_of_steps(14), 6);
    }
    #[test]
    fn test_2() {
        assert_eq!(minimum_swap("xx".to_string(), "yy".to_string()), 1);
        assert_eq!(minimum_swap("xy".to_string(), "yx".to_string()), 2);
        assert_eq!(minimum_swap("xx".to_string(), "xy".to_string()), -1);
        assert_eq!(
            minimum_swap("xxyyxyxyxx".to_string(), "xyyxyxxxyx".to_string()),
            4
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(num_triplets(vec![7, 4], vec![5, 2, 8, 9]), 1);
        assert_eq!(num_triplets(vec![1, 1], vec![1, 1, 1]), 9);
        assert_eq!(num_triplets(vec![7, 7, 8, 3], vec![1, 2, 9, 7]), 2);
        assert_eq!(
            num_triplets(vec![4, 7, 9, 11, 23], vec![3, 5, 1024, 12, 18]),
            0
        );
    }
    #[test]
    fn debug() {
        minimum_swap("xx".to_string(), "yy".to_string());
    }
}
