use std::io;

fn main() {
    // 요일을 숫자로 매핑하는 배열
    let weekdays: Vec<String> = vec!["월", "화", "수", "목", "금", "토", "일"]
        .into_iter()
        .map(String::from)
        .collect();

    // 입력 받기
    let mut month = String::new();
    io::stdin().read_line(&mut month).expect("입력 에러");
    let month: i32 = month.trim().parse().expect("숫자로 변환 에러");

    let mut days = String::new();
    io::stdin().read_line(&mut days).expect("입력 에러");
    let days: Vec<i32> = days
        .split_whitespace()
        .map(|s| s.parse().expect("숫자로 변환 에러"))
        .collect();

    let mut yoil = String::new();
    io::stdin().read_line(&mut yoil).expect("입력 에러");
    let yoil = yoil.trim().to_string();

    // 요일을 인덱스로 변환
    let index = weekdays.iter().position(|x| x == &yoil).expect("잘못된 요일");

    for _ in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 에러");
        let date: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("숫자로 변환 에러"))
            .collect();

        let m = date[0];
        let d = date[1];

        let mut ttlyoil = 0;
        let mut final = 0;

        if m > month || d - days[m as usize - 1] > 0 {
            println!("땡");
        } else {
            for (i, w) in days.iter().enumerate() {
                if m == (i + 1) as i32 {
                    ttlyoil = (ttlyoil + d - 1) % 7;
                    break;
                }
                ttlyoil += w;
            }

            final = (index + ttlyoil) % 7;
            println!("{}", weekdays[final as usize]);
        }
    }
}
