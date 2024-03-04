pub fn abc001() {
    /* Input */
    proconio::input! { h1:i32, h2:i32 }
    /* Solve */
    println!("{}", h1 - h2);
}

pub fn abc002() {
    /* Input */
    proconio::input! { x:i32, y:i32 }
    /* Solve */
    println!("{}", if x > y { x } else { y })
}

pub fn abc003() {
    /* Input */
    proconio::input! { n:i32 }
    /* Solve */
    let mut result = 0f32;
    for i in 1..n + 1 {
        result += i as f32 * 10000f32 * (1f32 / n as f32);
    }
    println!("{}", result);
}

pub fn abc004() {
    /* Input */
    proconio::input! { n:i32 }
    /* Solve */
    println!("{}", 2 * n);
}

pub fn abc005() {
    /* Input */
    proconio::input! { x:i32, y:i32 }
    /* Solve */
    println!("{}", y / x);
}

pub fn abc042() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32 }
    /* Solve */
    println!("{}", if a + b + c == 17 { "YES" } else { "NO" });
}

pub fn abc043() {
    /* Input */
    proconio::input! { n:i32 }
    /* Solve */
    let mut result = 0;
    for i in 1..n + 1 {
        result += i;
    }
    println!("{}", result);
}

pub fn abc044() {
    /* Input */
    proconio::input! { n:i32, k:i32, x:i32, y:i32 }
    /* Solve */
    println!(
        "{}",
        (std::cmp::min(n, k) * x) + (std::cmp::max(n - k, 0) * y)
    );
}

pub fn abc045() {
    /* Input */
    proconio::input! { a:i32, b:i32, h:i32 }
    /* Solve */
    println!("{}", (a + b) * h / 2);
}

pub fn abc046() {
    /* Input */
    proconio::input! { input:[i32; 3] }
    /* Solve */
    let types: std::collections::HashSet<i32> = input.into_iter().collect();
    println!("{}", types.len());
}

pub fn abc047() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32 }
    /* Solve */
    println!(
        "{}",
        if a + b + c - (std::cmp::max(a, std::cmp::max(b, c)) * 2) == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}

pub fn abc048() {
    /* Input */
    proconio::input! { f:String, m:String, l:String }
    /* Solve */
    println!(
        "{}{}{}",
        f.chars().nth(0).unwrap(),
        m.chars().nth(0).unwrap(),
        l.chars().nth(0).unwrap()
    );
}

pub fn abc049() {
    /* Input */
    proconio::input! { c:char }
    /* Solve */
    println!(
        "{}",
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            "vowel"
        } else {
            "consonant"
        }
    );
}

pub fn abc050() {
    /* Input */
    proconio::input! { a:i32, op:char, b:i32 }
    /* Solve */
    println!("{}", if op == '+' { a + b } else { a - b });
}

pub fn abc051() {
    /* Input */
    proconio::input! { s:String }
    /* Solve */
    println!("{}", s.replace(',', " "))
}

pub fn abc052() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32, d:i32 }
    /* Solve */
    println!("{}", if a * b >= c * d { a * b } else { c * d });
}

pub fn abc053() {
    /* Input */
    proconio::input! { x:i32 }
    /* Solve */
    println!("{}", if x < 1200 { "ABC" } else { "ARC" });
}

pub fn abc054() {
    /* Input */
    proconio::input! { a:i32, b:i32 }
    /* Solve */
    println!(
        "{}",
        if a == b {
            "Draw"
        } else if a == 1 {
            "Alice"
        } else if b == 1 {
            "Bob"
        } else if a > b {
            "Alice"
        } else {
            "Bob"
        }
    );
}

pub fn abc055() {
    /* Input */
    proconio::input! { n:i32 }
    /* Solve */
    println!("{}", n * 800 - (n / 15 * 200));
}

pub fn abc056() {
    /* Input */
    proconio::input! { a:char, b:char }
    /* Solve */
    println!(
        "{}",
        if (a == 'H' && b == 'H') || (a == 'D' && b == 'D') {
            "H"
        } else {
            "D"
        }
    )
}

pub fn abc057() {
    /* Input */
    proconio::input! { a:i32, b:i32 }
    /* Solve */
    println!("{}", if a + b >= 24 { a + b - 24 } else { a + b })
}

pub fn abc058() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32 }
    /* Solve */
    println!("{}", if b - a == c - b { "YES" } else { "NO" });
}

pub fn abc059() {
    /* Input */
    proconio::input! { s1:String, s2:String, s3:String }
    /* Solve */
    println!(
        "{}{}{}",
        s1.to_uppercase().chars().nth(0).unwrap(),
        s2.to_uppercase().chars().nth(0).unwrap(),
        s3.to_uppercase().chars().nth(0).unwrap()
    );
}

pub fn abc060() {
    /* Input */
    proconio::input! { a:String, b:String, c:String }
    /* Solve */
    println!(
        "{}",
        if a.chars().last().unwrap() == b.chars().nth(0).unwrap()
            && b.chars().last().unwrap() == c.chars().nth(0).unwrap()
        {
            "YES"
        } else {
            "NO"
        }
    );
}

pub fn abc061() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32 }
    /* Solve */
    println!("{}", if a <= c && c <= b { "Yes" } else { "No" });
}

pub fn abc062() {
    /* Input */
    proconio::input! { x:usize, y:usize }
    /* Solve */
    let grp: [i32; 12] = [0, 2, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0];
    println!(
        "{}",
        if grp[x - 1] == grp[y - 1] {
            "Yes"
        } else {
            "No"
        }
    );
}

pub fn abc063() {
    /* Input */
    proconio::input! { a:i32, b:i32 }
    /* Solve */
    println!(
        "{}",
        if a + b >= 10 {
            "error".to_string()
        } else {
            (a + b).to_string()
        }
    );
}

pub fn abc064() {
    /* Input */
    proconio::input! { r:i32, g:i32, b:i32 }
    /* Solve */
    println!(
        "{}",
        if (r * 100 + g * 10 + b) % 4 == 0 {
            "YES"
        } else {
            "NO"
        }
    );
}

pub fn abc065() {
    /* Input */
    proconio::input! { x:i32, a:i32, b:i32 }
    /* Solve */
    println!(
        "{}",
        if a >= b {
            "delicious"
        } else if b - a <= x {
            "safe"
        } else {
            "dangerous"
        }
    );
}

pub fn abc066() {
    /* Input */
    proconio::input! { abc:[i32;3] }
    /* Solve */
    let mut array: Vec<i32> = abc;
    array.sort_unstable();
    println!("{}", array[0] + array[1]);
}

pub fn abc067() {
    /* Input */
    proconio::input! { a:i32, b:i32 }
    /* Solve */
    println!(
        "{}",
        if a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0 {
            "Possible"
        } else {
            "Impossible"
        }
    );
}

pub fn abc068() {
    /* Input */
    proconio::input! { n:i32 }
    /* Solve */
    println!("ABC{}", n);
}

pub fn abc069() {
    /* Input */
    proconio::input! { n:i32, m:i32 }
    /* Solve */
    println!("{}", (n - 1) * (m - 1));
}

pub fn abc070() {
    /* Input */
    proconio::input! { n:i32 }
    /* Solve */
    println!("{}", if n % 10 == n / 100 { "Yes" } else { "No" });
}

pub fn abc071() {
    /* Input */
    proconio::input! { x:i32, a:i32, b:i32 }
    /* Solve */
    println!(
        "{}",
        if (x - a).abs() > (x - b).abs() {
            "B"
        } else {
            "A"
        }
    );
}

pub fn abc072() {
    /* Input */
    proconio::input! { x:i32, t:i32 }
    /* Solve */
    println!(
        "{}",
        if x - t <= 0 {
            0.to_string()
        } else {
            (x - t).to_string()
        }
    );
}

pub fn abc073() {
    /* Input */
    proconio::input! { n:i32 }
    /* Solve */
    println!(
        "{}",
        if n % 10 == 9 || n / 10 == 9 {
            "Yes"
        } else {
            "No"
        }
    );
}

pub fn abc074() {
    /* Input */
    proconio::input! { n:i32, a:i32 }
    /* Solve */
    println!("{}", n * n - a);
}

pub fn abc075() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32 }
    /* Solve */
    println!(
        "{}",
        if a == b {
            c
        } else if b == c {
            a
        } else {
            b
        }
    );
}

pub fn abc076() {
    /* Input */
    proconio::input! { r:i32, g:i32 }
    /* Solve */
    println!("{}", g * 2 - r);
}

pub fn abc077() {
    /* Input */
    proconio::input! { r1:String, r2:String }
    /* Solve */
    println!(
        "{}",
        if r1.chars().rev().collect::<String>() == r2 {
            "YES"
        } else {
            "NO"
        }
    );
}

pub fn abc078() {
    /* Input */
    proconio::input! { x:String, y:String }
    /* Solve */
    let x = u32::from_str_radix(x.as_str(), 16).unwrap();
    let y = u32::from_str_radix(y.as_str(), 16).unwrap();
    println!(
        "{}",
        if x > y {
            ">"
        } else if x < y {
            "<"
        } else {
            "="
        }
    );
}

pub fn abc079() {
    /* Input */
    proconio::input! { n:i32 }
    /* Solve */
    println!(
        "{}",
        if (n / 10) % 111 == 0 || (n % 1000) % 111 == 0 {
            "Yes"
        } else {
            "No"
        }
    )
}

pub fn abc080() {
    /* Input */
    proconio::input! { n:i32, a:i32, b:i32 }
    /* Solve */
    println!("{}", if n * a <= b { n * a } else { b })
}

pub fn abc081() {
    /* Input */
    proconio::input! { s:String }
    /* Solve */
    println!("{}", s.chars().filter(|&c| c == '1').count());
}

pub fn abc082() {
    /* Input */
    proconio::input! { a:f32, b:f32 }
    /* Solve */
    println!("{}", ((a + b) / 2.0).ceil());
}

pub fn abc083() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32, d:i32 }
    /* Solve */
    println!(
        "{}",
        if a + b > c + d {
            "Left"
        } else if a + b < c + d {
            "Right"
        } else {
            "Balanced"
        }
    );
}

pub fn abc084() {
    /* Input */
    proconio::input! { m:i32 }
    /* Solve */
    println!("{}", 48 - m);
}

pub fn abc085() {
    /* Input */
    proconio::input! { s:String }
    /* Solve */
    println!("2018{}", &s[4..])
}

pub fn abc086() {
    /* Input */
    proconio::input! { a:i32, b:i32 }
    /* Solve */
    println!("{}", if a * b % 2 == 0 { "Even" } else { "Odd" });
}

pub fn abc087() {
    /* Input */
    proconio::input! { x:i32, a:i32, b:i32 }
    /* Solve */
    println!("{}", (x - a) % b);
}

pub fn abc088() {
    /* Input */
    proconio::input! { n:i32, a:i32 }
    /* Solve */
    println!("{}", if n % 500 <= a { "Yes" } else { "No" });
}

pub fn abc089() {
    /* Input */
    proconio::input! { n:i32 }
    /* Solve */
    println!("{}", n / 3);
}

pub fn abc090() {
    /* Input */
    proconio::input! { s1:String, s2:String, s3:String }
    /* Solve */
    println!(
        "{}{}{}",
        s1.chars().nth(0).unwrap(),
        s2.chars().nth(1).unwrap(),
        s3.chars().nth(2).unwrap()
    );
}

pub fn abc091() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32 }
    /* Solve */
    println!("{}", if c <= a + b { "Yes" } else { "No" });
}

pub fn abc092() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32, d:i32 }
    /* Solve */
    println!("{}", std::cmp::min(a, b) + std::cmp::min(c, d));
}

pub fn abc093() {
    /* Input */
    proconio::input! { s:String }
    /* Solve */
    println!(
        "{}",
        if s.matches("a").count() == 1 && s.matches("b").count() == 1 && s.matches("c").count() == 1
        {
            "Yes"
        } else {
            "No"
        }
    );
}

pub fn abc094() {
    /* Input */
    proconio::input! { a:i32, b:i32, x:i32 }
    /* Solve */
    println!("{}", if b >= x - a && a <= x { "YES" } else { "NO" });
}

pub fn abc095() {
    /* Input */
    proconio::input! { s:String }
    /* Solve */
    println!("{}", s.matches("o").count() * 100 + 700);
}

pub fn abc096() {
    /* Input */
    proconio::input! {}
    /* Solve */
}

pub fn abc097() {
    /* Input */
    proconio::input! {}
    /* Solve */
}

pub fn abc098() {
    /* Input */
    proconio::input! {}
    /* Solve */
}
