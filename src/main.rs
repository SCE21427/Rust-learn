static DAY_IN_SOLAR_YEAR_APPROXIMATE: f64 = 365.2422; //静态变量
static  mut DAY_IN_YEAR: u16 = 365; //可变静态变量

fn main() {
    c1_variable();
    c2_const_and_static();
    c3_data_types();
    c3_1_calculate();
    c4_compound_types()
}

fn c1_variable() {
    //不可变与命名
    let count_a = 15; //自动推导变量类型为 i32
    let count_b: i16 = 15; //显式指定变量类型为 i16
    /*count_a = 2;*/ //尝试改变不可变变量的值,会报错
    let mut count_c: i32 = 2147483647; //声明可变
    count_c = -2147483648;
    //遮蔽(Shadow)
    let x = 1;
    {
        let x: bool = false; //覆盖x(数值）
        {
            let x = "Shadowing"; //遮蔽x(布尔值)
            println!("{}", x); //打印以验证
        } //x(字符串)作用域结束(释放)
        println!("{}", x); //打印以验证
    } //x(布尔值)作用域结束(释放)
    println!("{}", x); //打印以验证
    let mut y = 4;
    y = 5;
    println!("{count_a},{count_b},{count_c},{y}");
}

fn c2_const_and_static() {
    const SECONDS_IN_WEEK: u32 = 60 * 60 * 24; //使用`const`关键字声明常量
    /* SECONDS_IN_WEEK = 5; */ //尝试改变常量的值,会报错
    const SECONDS_IN_DAY: u32 = 86_400; //常量(另一种写法)
    {
        const A: u8 = 255;
    }
    /*println!("{A}");*/ //尝试打印不在作用域内的常量A,会报错
    const SECONDS_IN_SOLAR_YEAR: u64 = 60 * 60 * 24 * 365 + 60 * 60 * 5 + 60 * 48 + 46;
    
    /*unsafe {
        DAY_IN_YEAR = 366;
        println!("{DAY_IN_YEAR}"); //打印可变静态变量
    }*/ //使用`unsafe`修改可变静态变量
    
    println!("One week has {SECONDS_IN_WEEK} seconds |\
    \0 One day has {SECONDS_IN_DAY} seconds |\
    \0 One solar year has {SECONDS_IN_SOLAR_YEAR} seconds");
    
    println!("One solar year has {DAY_IN_SOLAR_YEAR_APPROXIMATE} days");
}
fn c3_data_types() {
    let a = 5; //整数, 默认i32
    let b: i32 = -5; //整数, 显式指定类型为i32
    let c: u32 = 5; //整数, 显式指定类型为u32, 仅支持自然数
    let d = 1.14; //浮点数, 默认f64
    let e: f32 = 3.14; //浮点数, 显式指定类型为f32
    let t = true; //布尔值
    let f: bool = false; //布尔值
    let character = '😅'; //字符类型, 4字节
    println!("{a}, {b}, {c}, {d}, {e}, {t}, {f}, {character}");
}

fn c3_1_calculate() {
    let a: i32 = 5;
    let c: f64 = 3.14;
    let e: i32 = 10;
    let f: i32 = 17;
    let add = a + e; //加法
    let sub: i32 = e - a; //减法
    let mul = e as f64 * c; //乘法
    let div = e / a; //除法
    let rem = e % a; //取余(能整除, 结果为0)
    let rems = f % a; //取余(不能整除）
    println!("{add}, {sub}, {mul}, {div}, {rem}, {rems}");
}

fn c4_compound_types() {
    fn array() {
        let a = [1, 2, 3, 4, 5]; //声明数组, 使用的是最基本的方法
        let b: [i32/*指定类型为i32*/; 6 /*指定数组的长度为6*/] = [0, 2, 4, 6, 8, 10];
        let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "November", "October", "December"];
        let a_first = a[0]; //访问数组a的第一个元素
        let b_second = b[1]; //访问数组b的第二个元素
        let length = months.len(); //获取数组months的长度
        println!("{a_first}, {b_second}, {length}");
    } 
    
    fn tuple() {
        let a = (1, 2, 3); //声明元组
        let b: (u8, i32, f64) = (7 ,-9, 33.2676); //声明元组, 显式指定类型
        let c = (1, 2.0, "Hello"); //声明元组, 不同类型
        let a_first = a.0; //访问元组a的第一个元素
        let b_second = b.1; //访问元组b的第二个元素
        let c_third = c.2; //访问元组c的第三个元素
        println!("{a_first}, {b_second}, {c_third}");
    }
    array();
    tuple();
}
