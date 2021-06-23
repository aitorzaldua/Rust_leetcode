//Leetcode day june 22
//Given a string s and an array of strings words, return the number of words[i] that is a subsequence of s.
//A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
//For example, "ace" is a subsequence of "abcde".
//Example
//Input: s = "abcde", words = ["a","bb","acd","ace"]
//Output: 3
//Explanation: There are three strings in words that are a subsequence of s: "a", "acd", "ace".


fn main() {
    let input = String::from("abcde");
    let words:Vec<String> = vec!["a".to_string(),"bb".to_string(),"acd".to_string(),"ace".to_string()];


  num_matching_subseq(input, words);
}

pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let s_to_vec: Vec<char> = s.chars().collect::<Vec<_>>();

    println!("test {:?}", s_to_vec);

    32
}



