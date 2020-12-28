use std::time::Instant;

fn main() {
  let start = Instant::now();
  let mut data: Vec<u32> = include_str!("test3.txt").lines().map(|c| c.parse::<u32>().unwrap()).collect();
  data.sort();
  //data.push(data[data.len()-1]+3);

  println!("{:?}", data);

  // let mut last_adapter: u32 = 0;
  // let mut jolts: Vec<u32> = vec![0, 0, 0];

  // for i in data.clone() {
  //   jolts[(i-last_adapter-1) as usize] += 1;
  //   last_adapter = i;
  // }

  // println!("{:?}, sum of 1x3: {}", jolts, jolts[0]*jolts[2]);

  let mut count: u64 = 0;
  let mut count_tmp: u64 = 0;
  let mut recursions: u128 = 0;

  kokotina(0, &data, &mut count, &mut recursions, &mut count_tmp);

  let duration = start.elapsed();
  println!("Numbers: {}\nCount: {}\nRecursions: {}\nDuration: {:?}", data.len()+1, count, recursions, duration);
}

fn kokotina(index: usize, list: &Vec<u32>, mut count: &mut u64, mut recursions: &mut u128, mut count_tmp: &mut u64) {
  'outer: for i in index..list.len() {
    'inner: for s in 1..=4 {
      if s == 4 {
        break 'outer;
      };
      for t in 1..=3 {
        if i+t < list.len() {
          //print!("{}+{}={},",i, t, i+t);
          if list[i]+s == list[i+t] {

            if i+t == list.len()-1 {
              //println!("C ");
              *count += 1;
              break 'outer;
            } else if s == 3 && t == 1 {
              //print!("S ");
              break 'inner;
            };

            //print!("{}->{}+{}={},", i, list[i], s, list[i+t]);
            *recursions += 1;
            kokotina(i+t, &list, &mut count, &mut recursions, &mut count_tmp);
          };
        };
      };
    };
  };
}