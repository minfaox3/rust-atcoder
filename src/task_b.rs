pub fn abc053() {
    /* Input */
    proconio::input! { s:String }
    /* Solve */
    println!("{}", s.rfind('Z').unwrap() - s.find('A').unwrap() + 1);
}
