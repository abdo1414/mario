use std::vec;

fn main() {

    let arrg = std::env::args();

    let mut i = vec![];
    for a in arrg {
        i.push(a)
    }

    piler(i[1].parse::<i32>().unwrap());
}

fn piler(mut num: i32) {
    if num < 0 {
        panic!("inter a viled intger")
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
