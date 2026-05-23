#![allow(dead_code)]

use std::mem::size_of_val;

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// ==================== SECTION 3: Variables ====================

fn s3_t1() {
    let x: i32 = 5;
    let _y: i32;
    assert_eq!(x, 5);
    println!("s3_t1 Success!");
}
fn s3_t2() {
    let mut x = 1;
    x += 2;
    assert_eq!(x, 3);
    println!("s3_t2 Success!");
}
fn s3_t3() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {}", x);
}
fn s3_t4() {
    let x = define_x();
    println!("{}, world", x);
}
fn define_x() -> &'static str { "hello" }
fn s3_t5() {
    let x: i32 = 5;
    { let x = 12; assert_eq!(x, 12); }
    assert_eq!(x, 5);
    let x = 42;
    println!("{}", x);
}
fn s3_t6() {
    let x: i32 = 7;
    let x = x;
    let y = 4;
    let _y = "I can also be bound to text!";
    println!("s3_t6 x={} y={}", x, y);
}
fn s3_t7() { let _x = 1; println!("s3_t7 Success!"); }
fn s3_t8() {
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("s3_t8 Success!");
}
fn s3_t9() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x, y], [3, 2]);
    println!("s3_t9 Success!");
}

// ==================== SECTION 4.1: Numbers ====================

fn s41_t1() {
    let x: i32 = 5;
    let y: i32 = x;
    let _z = 10;
    println!("s41_t1 y={} Success!", y);
}
fn s41_t2() {
    let v: u8 = 38_u16 as u8;
    println!("s41_t2 {} Success!", v);
}
fn s41_t3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
    println!("s41_t3 Success!");
}
fn s41_t4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("s41_t4 Success!");
}
fn s41_t5() {
    let v1 = 251_u8.wrapping_add(8);
    let v2 = i8::checked_add(100, 8).unwrap();
    println!("s41_t5 {},{} Success!", v1, v2);
}
fn s41_t6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
    println!("s41_t6 Success!");
}
fn s41_t7() {
    let x = 1_000_000.1_f64;
    assert_eq!(type_of(&x), "f64".to_string());
    println!("s41_t7 Success!");
}
fn s41_t8() {
    assert!((0.1_f32 + 0.2_f32 - 0.3_f32).abs() < 1e-6);
    assert!((0.1_f64 + 0.2_f64 - 0.3_f64).abs() < 1e-10);
    println!("s41_t8 Success!");
}
fn s41_t9() {
    let mut sum = 0;
    for i in -3..2 { sum += i; }
    assert!(sum == -5);
    for c in 'a'..='z' { print!("{} ", c as u8); }
    println!("\ns41_t9 Success!");
}
fn s41_t10() {
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    println!("s41_t10 Success!");
}
fn s41_t11() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);
    assert!(3 * 50 == 150);
    assert!((9.6_f64 / 3.2 - 3.0).abs() < 1e-10);
    assert!(24 % 5 == 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    println!("s41_t11 Success!");
}

// ==================== SECTION 4.2: Char, Bool, Unit ====================

fn s42_t1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);
    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);
    println!("s42_t1 Success!");
}
fn s42_t2() { let c1 = '中'; print_char(c1); }
fn print_char(c: char) { println!("s42_t2: {}", c); }
fn s42_t3() {
    let _f: bool = false;
    let t = true;
    if t { println!("s42_t3 Success!"); }
}
fn s42_t4() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);
    println!("s42_t4 Success!");
}
fn s42_t5() {
    let _v: () = ();
    implicitly_ret_unit();
    println!("s42_t5 Success!");
}
fn implicitly_ret_unit() { println!("I will return a ()"); }
fn s42_t6() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
    println!("s42_t6 Success!");
}

// ==================== SECTION 4.3: Statements ====================

fn s43_t1() {
    let v = { let mut x = 1; x += 2; x };
    assert_eq!(v, 3);
    println!("s43_t1 Success!");
}
fn s43_t2() {
    let v = { let x = 3; x };
    assert!(v == 3);
    println!("s43_t2 Success!");
}
fn s43_t3() {
    let s = sum43(1, 2);
    assert_eq!(s, 3);
    println!("s43_t3 Success!");
}
fn sum43(x: i32, y: i32) -> i32 { x + y }

// ==================== SECTION 4.4: Functions ====================

fn s44_t1() {
    let (x, y) = (1, 2);
    let s = sum44(x, y);
    assert_eq!(s, 3);
    println!("s44_t1 Success!");
}
fn sum44(x: i32, y: i32) -> i32 { x + y }
fn s44_t2() { print44(); }
fn print44() -> () { println!("s44_t2 Success!"); }
fn s44_t3() { never_return(); }
fn never_return() -> ! { panic!("never return!"); }
fn s44_t4() {
    println!("s44_t4 Success!");
}
fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => Some(1),
        _ => never_return_fn(),
    }
}
fn never_return_fn() -> ! { panic!("never return!"); }
fn s44_t5() {
    let b = true;
    let _v = match b {
        true => 1,
        false => { println!("Success!"); panic!("no value for false"); }
    };
    println!("s44_t5 Failed if this prints!");
}

// ==================== SECTION 6.1: String ====================

fn s61_t1() {
    let s: &str = "hello, world";
    println!("s61_t1: {} Success!", s);
}
fn greetings61(s: &str) { println!("{}", s); }
fn s61_t2() {
    let s: Box<str> = "hello, world".into();
    greetings61(&s);
}
fn s61_t3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');
    assert_eq!(s, "hello, world!");
    println!("s61_t3 Success!");
}
fn s61_t4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";
    println!("s61_t4: {}", s);
}
fn s61_t5() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");
    assert_eq!(s1, "I like cats");
    println!("s61_t5 Success!");
}
fn s61_t6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    println!("s61_t6: {}", s3);
}
fn greetings61b(s: String) { println!("{}", s); }
fn s61_t7() {
    let s = "hello, world".to_string();
    greetings61b(s);
}
fn s61_t8() {
    let s = "hello, world".to_string();
    let s1: &str = &s;
    println!("s61_t8: {} Success!", s1);
}
fn s61_t9() {
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);
    let long_string = "String literals
                can span multiple lines.
                The linebreak and indentation here \
                can be escaped too!";
    println!("{}", long_string);
}
fn s61_t10() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: \\x3F \\u{211D}");
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");
    println!("s61_t10 Success!");
}
fn s61_t11() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1];
    assert_eq!(h, "h");
    let h1 = &s1[3..6];
    assert_eq!(h1, "中");
    println!("s61_t11 Success!");
}
fn s61_t12() {
    for c in "你好，世界".chars() {
        println!("{}", c);
    }
}

// ==================== SECTION 6.2: Array ====================

fn s62_t1() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert!(arr.len() == 5);
    println!("s62_t1 Success!");
}
fn s62_t2() {
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    assert!(std::mem::size_of_val(&arr) == 12);
    println!("s62_t2 arr0={:?} arr={:?} Success!", arr0, arr);
}
fn s62_t3() {
    let list: [i32; 100] = [1; 100];
    assert!(list[0] == 1);
    assert!(list.len() == 100);
    println!("s62_t3 Success!");
}
fn s62_t4() {
    let _arr = [1, 2, 3];
    println!("s62_t4 Success!");
}
fn s62_t5() {
    let arr = ['a', 'b', 'c'];
    let ele = arr[0];
    assert!(ele == 'a');
    println!("s62_t5 Success!");
}
fn s62_t6() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    let name0 = names.get(0).unwrap();
    let _name1 = &names[1];
    println!("s62_t6: {} Success!", name0);
}

// ==================== SECTION 6.3: Slice ====================

fn s63_t1() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];
    let s2: &str = "hello, world";
    println!("s63_t1: {:?} {} Success!", s1, s2);
}
fn s63_t2() {
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[..2];
    assert!(std::mem::size_of_val(slice) == 8);
    println!("s63_t2 Success!");
}
fn s63_t3() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
    println!("s63_t3 Success!");
}
fn s63_t4() {
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    assert_eq!(slice1, slice2);
    println!("s63_t4 Success!");
}
fn s63_t5() {
    let s = "你好，世界";
    let slice = &s[0..3];
    assert!(slice == "你");
    println!("s63_t5 Success!");
}
fn first_letter(s: &str) -> &str { &s[..1] }
fn s63_t6() {
    let mut s = String::from("hello world");
    let letter = first_letter(&s);
    println!("the first letter is: {}", letter);
    s.clear();
}

// ==================== SECTION 6.4: Tuple ====================

fn s64_t1() {
    let _t0: (u8, i16) = (0, -1);
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    println!("s64_t1: {} Success!", t.3);
}
fn s64_t2() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
    println!("s64_t2 Success!");
}
fn s64_t3() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}
fn s64_t4() {
    let tup = (1, 6.4, "hello");
    let (x, z, y) = tup;
    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
    println!("s64_t4 Success!");
}
fn s64_t5() {
    let (x, y, z);
    (x, y, z) = (3, 2, 1);
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    assert_eq!(z, 1);
    println!("s64_t5 Success!");
}
fn s64_t6() {
    let (x, y) = sum_multiply((2, 3));
    assert_eq!(x, 5);
    assert_eq!(y, 6);
    println!("s64_t6 Success!");
}
fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

// ==================== SECTION 6.5: Struct ====================

struct Person { name: String, age: u8, hobby: String }
fn s65_t1() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };
    println!("s65_t1: {} Success!", p.name);
}

struct Unit;
trait SomeTrait { fn behavior(&self); }
impl SomeTrait for Unit { fn behavior(&self) { println!("Unit behavior"); } }
fn do_something_with_unit(u: Unit) { u.behavior(); }
fn s65_t2() {
    let u = Unit;
    do_something_with_unit(u);
    println!("s65_t2 Success!");
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Wait, p is moved above. Let me fix:
fn check_color2(p: Color) {
    assert_eq!(p.0, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}
fn s65_t3() {
    let v = Point(0, 127, 255);
    check_color2(Color(v.0, v.1, v.2));
    println!("s65_t3 Success!");
}

struct Person2 { name: String, age: u8 }
fn s65_t4() {
    let age = 18;
    let mut p = Person2 { name: String::from("sunface"), age };
    p.age = 30;
    let _new_name = String::from("sunfei");
    println!("s65_t4 Success!");
}
fn build_person(name: String, age: u8) -> Person2 {
    Person2 { age, name }
}
fn s65_t5() {
    let p = build_person(String::from("sunface"), 25);
    println!("s65_t5: {} Success!", p.name);
}

struct User { active: bool, username: String, email: String, sign_in_count: u64 }
fn set_email(u: User) -> User {
    User { email: String::from("contact@im.dev"), ..u }
}
fn s65_t6() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };
    let u2 = set_email(u1);
    println!("s65_t6: {} Success!", u2.email);
}

#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }
fn s65_t7() {
    let scale = 2;
    let rect1 = Rectangle { width: dbg!(30 * scale), height: 50 };
    dbg!(&rect1);
    println!("{:?}", rect1);
}

#[derive(Debug)]
struct File { name: String, data: String }
fn s65_t8() {
    let f = File { name: String::from("readme.md"), data: "Rust By Practice".to_string() };
    let _name = f.name.clone();
    println!("{}, {}, {:?}", f.name, f.data, f);
}

// ==================== SECTION 6.6: Enum ====================

enum Number { Zero, One, Two }
enum Number1 { Zero = 0, One, Two }
enum Number2 { Zero = 0, One = 1, Two = 2 }
fn s66_t1() {
    assert_eq!(Number::One as i32, Number1::One as i32);
    assert_eq!(Number1::One as i32, Number2::One as i32);
    println!("s66_t1 Success!");
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn s66_t2() {
    let msg1 = Message::Move { x: 1, y: 2 };
    let msg2 = Message::Write(String::from("hello, world!"));
    println!("s66_t2 Success!");
    let _ = (msg1, msg2);
}
fn s66_t3() {
    let msg = Message::Move { x: 1, y: 1 };
    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN!");
    }
    println!("s66_t3 Success!");
}

#[derive(Debug)]
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn show_message(msg: Message2) { println!("{:?}", msg); }
fn s66_t4() {
    let msgs: Vec<Message2> = vec![
        Message2::Quit,
        Message2::Move { x: 1, y: 3 },
        Message2::ChangeColor(255, 255, 0),
    ];
    for msg in msgs { show_message(msg); }
    println!("s66_t4 Success!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn s66_t5() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    if let Some(n) = six {
        println!("{}", n);
        println!("s66_t5 Success!");
    }
    let _ = none;
}

enum List {
    Cons(u32, Box<List>),
    Nil,
}
impl List {
    fn new() -> List { List::Nil }
    fn prepend(self, elem: u32) -> List { List::Cons(elem, Box::new(self)) }
    fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            List::Nil => format!("Nil"),
        }
    }
}
fn s66_t6() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

fn main() {
    // Section 3
    s3_t1(); s3_t2(); s3_t3(); s3_t4(); s3_t5(); s3_t6(); s3_t7(); s3_t8(); s3_t9();
    // Section 4.1
    s41_t1(); s41_t2(); s41_t3(); s41_t4(); s41_t5(); s41_t6(); s41_t7(); s41_t8(); s41_t9(); s41_t10(); s41_t11();
    // Section 4.2
    s42_t1(); s42_t2(); s42_t3(); s42_t4(); s42_t5(); s42_t6();
    // Section 4.3
    s43_t1(); s43_t2(); s43_t3();
    // Section 4.4
    s44_t1(); s44_t2(); s44_t4(); let _ = get_option(1);
    // Section 6.1
    s61_t1(); s61_t2(); s61_t3(); s61_t4(); s61_t5(); s61_t6(); s61_t7(); s61_t8(); s61_t9(); s61_t10(); s61_t11(); s61_t12();
    // Section 6.2
    s62_t1(); s62_t2(); s62_t3(); s62_t4(); s62_t5(); s62_t6();
    // Section 6.3
    s63_t1(); s63_t2(); s63_t3(); s63_t4(); s63_t5(); s63_t6();
    // Section 6.4
    s64_t1(); s64_t2(); s64_t3(); s64_t4(); s64_t5(); s64_t6();
    // Section 6.5
    s65_t1(); s65_t2(); s65_t3(); s65_t4(); s65_t5(); s65_t6(); s65_t7(); s65_t8();
    // Section 6.6
    s66_t1(); s66_t2(); s66_t3(); s66_t4(); s66_t5(); s66_t6();
    println!("\n=== ALL DONE ===");
}
