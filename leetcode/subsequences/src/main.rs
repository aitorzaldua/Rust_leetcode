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
    //let words:Vec<String> = vec!["a".to_string(),"bb".to_string(),"acd".to_string(),"ace".to_string()];
    let words:Vec<String> = vec!["a".to_string(),"bb".to_string()];


  num_matching_subseq(input, words);
}

pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let s_to_vec: Vec<char> = s.chars().collect::<Vec<_>>();
    let mut counter = 0;

    println!("input es {:?}", s_to_vec);

    for i in words {
      let i_to_vec: Vec<char> = i.chars().collect::<Vec<_>>();
      println!("el vector: {:?}", i_to_vec);
      for (i, &x) in i_to_vec.iter().enumerate() {
        println! ("el indice es {} y el valor {}", &i, &x);
        for (y, &z) in s_to_vec.iter().enumerate() {
          counter += 1;
          println! ("iteracion {} -> letra {} con {}", counter, x, z);
           if x == z {
             println! ("{} igual a {}", x, z );

           }
           else {
             println!("no es igual");
           }
        }
      }
    }

    32
}



