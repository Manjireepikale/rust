// Function to check whether a given string is a palindrome or not
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

// Function to find the index of the first occurrence of a given number in a sorted array
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// Function to find the shortest word in a string of words
fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|w| w.len())
}

// Function to check whether a given number is prime or not
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Function to find the median of a sorted array
fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2] + arr[len / 2 - 1]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// Function to find the longest common prefix of a given set of strings
fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = String::new();
    for (i, &ch) in strings[0].as_bytes().iter().enumerate() {
        for &s in &strings[1..] {
            if s.as_bytes().get(i) != Some(&ch) {
                return prefix;
            }
        }
        prefix.push(ch as char);
    }
    prefix
}

// Function to find the kth smallest element in a given array
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Function to calculate the maximum depth of a binary tree
fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

// Function to reverse a string
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// Function to check if a number is prime
fn is_prime_rust(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Function to merge two sorted arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);
    merged
}

// Function to find the maximum subarray sum
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = 0;
    let mut max_so_far = std::i32::MIN;
    for &x in arr {
        max_ending_here = max_ending_here.max(0) + x;
        max_so_far = max_so_far.max(max_ending_here);
    }
    max_so_far
}

fn main() {
    // Test cases
    println!("{}", is_palindrome("racecar")); // true
    println!("{}", first_occurrence(&[1, 2, 3, 4, 5], 3).unwrap_or(0)); // 2
    println!("{:?}", shortest_word("This is a test sentence")); // Some("a")
    println!("{}", is_prime(17)); // true
    println!("{}", find_median(&[1, 2, 3, 4, 5])); // 3.0
    println!("{}", longest_common_prefix(&["flower", "flow", "flight"])); // "fl"
    println!("{}", kth_smallest(&[4, 1, 3, 2, 5], 3).unwrap_or(0)); // 3
    let tree = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
        })),
    }));
    println!("{}", max_depth(tree)); // 3
    println!("{}", reverse_string("hello")); // "olleh"
    println!("{}", is_prime_rust(17)); // true
    println!("{:?}", merge_sorted_arrays(&[1, 3, 5], &[2, 4, 6])); // [1, 2, 3, 4, 5, 6]
    println!("{}", max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4])); // 6
}
