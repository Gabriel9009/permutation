
fn main() {
    let mut count = 0;
    let mut count1 = 1;
    let mut vec = vec![1, 2, 3, 4];
    for _i in 0..=vec.len() * 2{
    vec.swap(count, count1);
  
         println!("{:?}", vec);
         count += 1;
         count1 += 1;
         if count1 == vec.len(){
            count = 0;
            count1 = 1;
         }
  
}
}
