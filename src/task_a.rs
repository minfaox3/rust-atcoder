pub fn abc042() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32 }
    /* Solve */
    println!("{}", if a+b+c==17 { "YES" } else { "NO" });
}

pub fn abc043() {
    /* Input */
    proconio::input! { n:i32 }
    /* Solve */
    let mut result = 0;
    for i in 1..n+1 { result+=i; }
    println!("{}", result);
}

pub fn abc044() {
    /* Input */
    proconio::input! { n:i32, k:i32, x:i32, y:i32 }
    /* Solve */
    println!("{}", (std::cmp::min(n,k)*x)+(std::cmp::max(n-k,0)*y));
}

pub fn abc045() {
    /* Input */
    proconio::input! { a:i32, b:i32, h:i32 }
    /* Solve */
    println!("{}", (a+b)*h/2);
}

pub fn abc046() {
    /* Input */
    proconio::input! { input:[i32; 3] }
    /* Output */
    let types:std::collections::HashSet<i32> = input.into_iter().collect();
    println!("{}", types.len());
}

pub fn abc047() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32 }
    /* Output */
    println!("{}", if a+b+c-(std::cmp::max(a, std::cmp::max(b, c))*2)==0 { "Yes" } else { "No" });
}

pub fn abc048() {
    /* Input */
    proconio::input! { f:String, m:String, l:String }
    /* Output */
    println!("{}{}{}",f.chars().nth(0).unwrap(),m.chars().nth(0).unwrap(),l.chars().nth(0).unwrap());
}

pub fn abc049() {
    /* Input */
    proconio::input! { c:char }
    /* Output */
    println!("{}", if c=='a'||c=='e'||c=='i'||c=='o'||c=='u' { "vowel" } else { "consonant" });
}

pub fn abc050() {
    /* Input */
    proconio::input! { a:i32, op:char, b:i32 }
    /* Output */
    println!("{}", if op=='+' { a+b } else { a-b });
}