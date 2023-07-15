
fn main() {

    let arrg = std::env::args();

    let i:i32 = arrg.last().unwrap().trim().parse().unwrap_or(0);

    
    piler(i);
}

fn piler(num: i32) {
    let mut num = num.clone();
    if num < 0 {
        panic!("inter a valed intger")
    }    
    loop {
        println!("{}", get_the_wall(num));
        num-=1;

        if num == 0 {
            break;
        }
    }
}                                                                                                               
fn get_the_wall(num:i32) -> String {
  let mut tmp = num;
  let mut brick = String::new();
  loop {
     brick.push('#');
     tmp-=1; 
     if tmp == 0 {
        break;
     }

  }
  brick
}
