fn k_length_apart(nums: Vec<i32>,k: i32)->bool{
    let mut count=0;
    //make vector string 'length'
    let mut length=Vec::new();
    for i in 0..nums.len(){
        if nums[i]==0{
            count+=1;
        }
        else {
            length.push(count);
            count=0;
        }
    }
    //check if the vector string 'length' has any value less than k
   for i in 1..length.len(){
    if length[i]<k{
        return false;
    }
}
true
}
fn main() {
    let nums=vec![1,0,0,0,1,0,0,1];
    let k=2;
    println!("{}", k_length_apart(nums,k));
}
