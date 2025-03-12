use std::io;

fn main() {
    let count = 0;
    
    // 원하는 위치의 숫자입력
    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("읽는것을 실패했습니다");

    let number: u64 = number.trim().parse().expect("읽는것을 실패했습니다");
    
}
