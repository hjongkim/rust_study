fn echo_num(n: u32) {
    println!("{}", n.to_string());
}

fn echo_num_multi(n: u32, mut times: u32) {
    if times > 10 {
        times = 10;
    }
    for _ in 0..times {
        echo_num(n);
    }
}

fn echo_str(to_say: &String) {
    println!("{}", to_say);
}

fn echo_str_multi(to_say: &String, times: u32) {
    for _ in 0..times {
        echo_str(to_say);
    } 
}

fn main() {
    let num = 123_u32;
    echo_num(num);
    echo_num_multi(num, 100);

    //ownership
    let say = "hello".to_string();
    echo_str_multi(&say, 100);
}