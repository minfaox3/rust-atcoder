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
    /* Solve */
    let types:std::collections::HashSet<i32> = input.into_iter().collect();
    println!("{}", types.len());
}

pub fn abc047() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32 }
    /* Solve */
    println!("{}", if a+b+c-(std::cmp::max(a, std::cmp::max(b, c))*2)==0 { "Yes" } else { "No" });
}

pub fn abc048() {
    /* Input */
    proconio::input! { f:String, m:String, l:String }
    /* Solve */
    println!("{}{}{}",f.chars().nth(0).unwrap(),m.chars().nth(0).unwrap(),l.chars().nth(0).unwrap());
}

pub fn abc049() {
    /* Input */
    proconio::input! { c:char }
    /* Solve */
    println!("{}", if c=='a'||c=='e'||c=='i'||c=='o'||c=='u' { "vowel" } else { "consonant" });
}

pub fn abc050() {
    /* Input */
    proconio::input! { a:i32, op:char, b:i32 }
    /* Solve */
    println!("{}", if op=='+' { a+b } else { a-b });
}

pub fn abc051() {
    /* Input */
    proconio::input! { s:String }
    /* Solve */
    println!("{}", s.replace(','," "))
}

pub fn abc052() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32, d:i32 }
    /* Solve */
    println!("{}", if a*b>=c*d { a*b } else { c*d });
}

pub fn abc053() {
    /* Input */
    proconio::input! { x:i32 }
    /* Solve */
    println!("{}", if x<1200 { "ABC" } else { "ARC" });
}

pub fn abc054() {
    /* Input */
    proconio::input! { a:i32, b:i32 }
    /* Solve */
    println!("{}", if a==b { "Draw" } else if a==1 { "Alice" } else if b==1 { "Bob" } else if a>b { "Alice" } else { "Bob" });
}

pub fn abc055() {
    /* Input */
    proconio::input! { n:i32 }
    /* Solve */
    println!("{}", n*800-(n/15*200));
}

pub fn abc056() {
    /* Input */
    proconio::input! { a:char, b:char }
    /* Solve */
    println!("{}", if (a=='H'&&b=='H')||(a=='D'&&b=='D') { "H" } else { "D" })
}

pub fn abc057() {
    /* Input */
    proconio::input! { a:i32, b:i32 }
    /* Solve */
    println!("{}", if a+b>=24 { a+b-24 } else { a+b })
}

pub fn abc058() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32 }
    /* Solve */
    println!("{}", if b-a==c-b { "YES" } else { "NO" });
}

pub fn abc059() {
    /* Input */
    proconio::input! { s1:String, s2:String, s3:String }
    /* Solve */
    println!("{}{}{}", s1.to_uppercase().chars().nth(0).unwrap(),s2.to_uppercase().chars().nth(0).unwrap(),s3.to_uppercase().chars().nth(0).unwrap());
}

pub fn abc060() {
    /* Input */
    proconio::input! { a:String, b:String, c:String }
    /* Solve */
    println!("{}", if a.chars().last().unwrap()==b.chars().nth(0).unwrap() && b.chars().last().unwrap()==c.chars().nth(0).unwrap() { "YES" } else { "NO" });
}

pub fn abc061() {
    /* Input */
    proconio::input! { a:i32, b:i32, c:i32 }
    /* Solve */
    println!("{}", if a<=c && c<=b { "Yes" } else { "No" });
}

pub fn abc062() {
    /* Input */
    proconio::input! { x:usize, y:usize }
    /* Solve */
    let grp:[i32;12] = [0, 2, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0];
    println!("{}", if grp[x-1]==grp[y-1] { "Yes" } else { "No" });
}

pub fn abc063() {
    /* Input */
    proconio::input! { a:i32, b:i32 }
    /* Solve */
    println!("{}", if a+b>=10 { "error".to_string() } else { (a+b).to_string() });
}

pub fn abc064() {
    /* Input */
    proconio::input! { r:i32, g:i32, b:i32 }
    /* Solve */
    println!("{}", if (r*100+g*10+b)%4==0 { "YES" } else { "NO" });
}

pub fn abc065() {
    /* Input */
    proconio::input! { x:i32, a:i32, b:i32 }
    /* Solve */
    println!("{}", if a>=b { "delicious" } else if b-a<=x { "safe" } else { "dangerous" });
}

pub fn abc066() {
    /* Input */
    proconio::input! { abc:[i32;3] }
    /* Solve */
    let mut array:Vec<i32> = abc;
    array.sort_unstable();
    println!("{}", array[0]+array[1]);
}