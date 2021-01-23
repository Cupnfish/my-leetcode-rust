mod easy {
    use std::{cell::RefCell, collections::HashMap, hash, rc::Rc};

    use crate::structs::tree::TreeNode;

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ret = Vec::new();
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut s: String, ret: &mut Vec<String>) {
            if let Some(node) = node {
                let mut node_ref = node.borrow_mut();
                let val = node_ref.val.to_string();
                s = if s.is_empty() {
                    val
                } else {
                    s + "->" + val.as_str()
                };
                if node_ref.left.is_none() && node_ref.right.is_none() {
                    ret.push(s);
                    return;
                }
                dfs(node_ref.left.take(), s.clone(), ret);
                dfs(node_ref.right.take(), s, ret);
            }
        }
        dfs(root, String::new(), &mut ret);
        ret
    }
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut temp = HashMap::new();
        let mut max = 0;
        fn bfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            layer: usize,
            temp: &mut HashMap<usize, Vec<i32>>,
            max: &mut usize,
        ) {
            if let Some(node) = node {
                let mut node_ref = node.borrow_mut();
                let val = node_ref.val;
                temp.entry(layer)
                    .and_modify(|vec| vec.push(val))
                    .or_insert(vec![val]);
                if node_ref.left.is_none() && node_ref.right.is_none() {
                    *max = layer.max(*max);
                    return;
                }
                bfs(node_ref.left.take(), layer + 1, temp, max);
                bfs(node_ref.right.take(), layer + 1, temp, max);
            }
        }
        bfs(root, 0, &mut temp, &mut max);
        for i in 0..=max {
            if let Some(value) = temp.remove(&i) {
                ret.push(value);
            }
        }
        ret
    }
}
mod medium {
    #![feature(map_first_last)]
    use std::collections::{BTreeMap, HashMap};

    pub fn get_hint(secret: String, guess: String) -> String {
        let mut secret_map = HashMap::with_capacity(10);
        let mut guess_map = HashMap::with_capacity(10);
        let mut bulls = 0;
        let mut cows = 0;
        secret.chars().zip(guess.chars()).for_each(|(s, g)| {
            if s == g {
                bulls += 1;
            } else {
                secret_map
                    .entry(s)
                    .and_modify(|count| {
                        *count += 1;
                    })
                    .or_insert(1);
                guess_map
                    .entry(g)
                    .and_modify(|count| {
                        *count += 1;
                    })
                    .or_insert(1);
            }
        });
        secret_map.iter().for_each(|(k, &secret)| {
            if let Some(&guess) = guess_map.get(k) {
                cows += guess.min(secret);
            }
        });
        format!("{}A{}B", bulls, cows)
    }
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() as i32 % k != 0 {
            return false;
        }
        let mut counts = BTreeMap::new();
        for num in nums{
            counts.entry(num).and_modify(|c|*c+=1).or_insert(1);
        }
        if let Some(mut entry) = counts.first_entry() {
            let &key = entry.key();
            if let Some(&count) = counts.get(&key){
                for i in 1..k {
                  if let Some(&b_count) = counts.get(&(key+1)){
                      if count-b_count>0{
                          counts.entry(key+1).and_modify(|a|*a-=count);
                      }else if count==b_count{
                          counts.remove(&(key+1));
                      }else {
                          return false;
                      }
                  }else {
                      return false;
                  }
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod test {
    use super::medium::{get_hint, is_possible_divide};

    #[test]
    fn test_0() {
        assert_eq!(get_hint("1807".to_string(), "7810".to_string()), "1A3B");
        assert_eq!(get_hint("1123".to_string(), "0111".to_string()), "1A1B");
    }
    #[test]
    fn test_1() {
        assert_eq!(is_possible_divide(vec![1, 2, 3, 3, 4, 4, 5, 6], 4), true);
        assert_eq!(
            is_possible_divide(vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11], 3),
            true
        );
        assert_eq!(is_possible_divide(vec![3, 3, 2, 2, 1, 1], 3), true);
        assert_eq!(is_possible_divide(vec![1, 2, 3, 4], 3), false);
    }
}
