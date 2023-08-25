use rand::Rng;

pub fn rand_code() -> String {
    let mut numbers = vec![0; 6];

    let mut rng = rand::thread_rng();

    // 生成5个随机数字
    for i in 0..6 {
        let mut new_number: i32;

        loop {
            new_number = rng.gen_range(0..10);

            if !numbers.contains(&new_number) {
                break;
            }
        }

        numbers[i] = new_number;
    }
    return numbers
        .as_slice()
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("");
}
