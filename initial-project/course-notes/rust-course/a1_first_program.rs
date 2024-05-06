fn main() {
    let data = "\
    common name, length(cm)\
    A, 33\
    B, 65\
    C, 60\
    Invalid, data";

    let records = data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue
        }

        // 类型Vec是vector的缩写，是一个可伸缩的集合类型，可认为是一个动态数组；<_>表示Vec中的元素类型由编译器自行推断
        // 这里map()方法中使用的参数是闭包函数
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        // 这里的输出仅在debug模式下生效
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields); // 输出到标准错误输出
        }

        /**
        0. 类型标注：这里编译器无法推断数据类型，于是通过::<f32> 的使用，告诉编译器 length 是一个 f32 类型的浮点数。
        1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
        2. if let 是一个匹配表达式，用来从 = 右边的结果中，匹配出 length 的值：
            1）当 = 右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，
               if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
            2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        */
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length)
        }



    }
}