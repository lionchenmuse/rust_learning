fn main() {
    println!("sum_v1 test Start...");
    let mut vector: Vec<u32> = (0..1000).collect();
    let mut sum = sum_v1(&vector);
    match sum {
        Some(v) => println!("Sum: {}", v),
        None => println!("Overflow"),
    }
    vector = vec![u32::MAX, 1];
    sum = sum_v1(&vector);
    match sum {
        Some(v) => println!("Sum: {}", v),
        None => println!("Overflow"),
    }
    println!("sum_v1 test End...");

    println!("=======================================");
    println!("sum_v2 test Start...");

    vector = (0..1000).collect();
    sum = sum_v2(&vector);
    match sum {
        Some(v) => println!("Sum: {}", v),
        None => println!("Overflow"),
    }

    vector = vec![u32::MAX, 1];
    sum = sum_v2(&vector);
    match sum {
        Some(v) => println!("Sum: {}", v),
        None => println!("Overflow"),
    }
    println!("sum_v2 test End...");

}

fn sum_v1(group: &[u32]) -> Option<u32> {
    let max = u32::MAX as u64;
    if group.is_empty() {
        return None;
    }
    let mut sum = 0_u64;

    for v in group.iter() {
        sum += *v as u64;

        if sum > max {
            return None;
        }
    }
    Some(sum as u32)
}

fn sum_v2(group: &[u32]) -> Option<u32> {
    if group.is_empty() {
        return None;
    }
    let mut sum = 0_u32;
    let mut result = Some(0_u32);

    for g in group.iter() {
        match sum.checked_add(*g) {
            Some(v) => {
                sum = v;
                result = Some(v);
            },
            None => {
                result = None;
                break;
            }
        }
    }
    result
}
