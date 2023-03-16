fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
    let mut units = Vec::new();
    let mut longest = 0;
    units.push(logs[0][1] - 0);
    for i in 1..logs.len() { 
        units.push(logs[i][1] - logs[i-1][1]);
    }
  // code that exclude units that have same value, the smaller one has to be survived
    for i in 0..units.len() {
        for j in i+1..units.len() {
            if units[i] == units[j] {
                if logs[i][0] < logs[j][0] {
                    units[j] = 0;
                } else {
                    units[i] = 0;
                }
            }
        }
    }
    for i in 0..units.len() {
        if units[i as usize] > longest {
            longest = units[i as usize];
        }
    }
    
   logs[units.iter().position(|&x| x == longest).unwrap()][0]
}

fn main() {
    let n = 10;
    let logs = vec![vec![0,3], vec![2,5], vec![0,9],vec![1,15]];
    println!("{}", hardest_worker(n, logs));
}
