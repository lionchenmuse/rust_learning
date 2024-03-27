use rand::Rng;

fn main() {
    let mut array: [i32; 10] = [0; 10];
    // 获取一个线程局部的随机数生成器
    let mut rng = rand::thread_rng();

    // 生成一个随机数组
    for i in 0..array.len() {
        array[i] = rng.gen_range(0..100);
    }
    // 打印未排序的数组
    println!("Unsorted array: {:?}", array);
    // 预先clone
    let mut array1 = array.clone();

    // 冒泡排序
    bubbl_sort(&mut array);
    // 打印排序后的数组
    println!("1.Sorted array: {:?}", array);

    // 泛型冒泡排序
    bubbl_sort_generic(&mut array1);
    // 打印排序后的数组
    println!("2.Sorted array by generic sorting: {:?}", array1);

    // 准备一个Vec<String>
    let mut list: Vec<String> = vec![];
    list.push("We".to_string());
    list.push("use".to_string());
    list.push("generics".to_string());
    list.push("to".to_string());
    list.push("create".to_string());
    list.push("definitions".to_string());
    list.push("for".to_string());
    list.push("items".to_string());
    list.push("like".to_string());
    list.push("function".to_string());
    list.push("signatures".to_string());
    list.push("or".to_string());
    list.push("structs".to_string());
    list.push("which".to_string());
    list.push("we".to_string());
    list.push("can".to_string());
    list.push("then".to_string());
    list.push("use".to_string());
    list.push("with".to_string());
    list.push("many".to_string());
    list.push("different".to_string());
    list.push("concrete".to_string());
    list.push("data".to_string());
    list.push("types".to_string());

    print!("Before sorting: {}", list.join(" "));
    println!();
    // 泛型冒泡排序
    bubbl_sort_generic(&mut list);
    print!("After sorting: {}", list.join(" "));
    println!();
}

/// 冒泡排序：非泛型版本
fn bubbl_sort(array: &mut [i32]) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j] > array[j + 1] {
                let tmp = array[j];
                array[j] = array[j + 1];
                array[j + 1] = tmp;
            }
        }
    }
}

/// 冒泡排序：泛型版本
fn bubbl_sort_generic<T: PartialOrd> (list: &mut [T]) {
    for i in 0..list.len() {
        for j in 0..list.len() - i - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j+1);
            }
        }
    }
}