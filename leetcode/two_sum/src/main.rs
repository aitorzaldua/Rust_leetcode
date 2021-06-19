//Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//Edge cases: vec[] -- vec[0,4,3,0] target = 0 output = [0,3]


fn main() {
    let nums:Vec<i32> = vec![0,4,3,0];
    let target = 0;


    println!("{:?}",two_sum(nums, target));

}

pub fn two_sum(nums: Vec<i32>, target: i32)  -> Vec<i32>{
    let mut counter_1 = 0;
    let mut push_1: i32 = 0;
    let mut counter_2 = 1;
    let mut push_2: i32 = 1;
    let mut nums_3:Vec<i32> = vec![];


    loop{
        println!("{} - {} - {} - {}", nums[counter_1], nums[counter_2], nums[counter_1] + nums[counter_2], target );
        if nums.len() == 0{
            break
        }
        if counter_2 == nums.len(){
            break
        }
        else if nums[counter_1] + nums[counter_2] == target {
            nums_3.push(push_1);
            nums_3.push(push_2);
            break
        }
        else{
            if counter_2 == nums.len() - 1{
                counter_1 += 1;
                counter_2 = counter_1 + 1;
                push_1 += 1;
                push_2 = push_1 +1;
            }
            else{
                counter_2 += 1;
                push_2 += 1;
            }
        }

    }
nums_3

}
