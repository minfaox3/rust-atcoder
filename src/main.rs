/*
RustAtCoder(https://github.com/minfaox3/rust-atcoder)
最終更新日(Last updated):2021/02/01 メンテナー(Maintainer):minfaox3(spdlci30@gmail.com)
更新内容はREADMEを見てください。 Please read README to know what's new.

「説明」
AtCoderの終了済みコンテストの問題をRustで解いたソースコード集です。問題の難易度(A,B,C,...)順に分類されています。
AtCoderのジャッジでACしたものを掲載していますが、理想的な解法でない場合があります。
問題にある制約に従って入力されることを想定しています。
本ソースコードはライセンスフリーCC0ですが、使用している各クレートについては各作成者に著作権などが帰属します。
随時更新します。

"Description"(English translation by Google Translate)
A collection of source code that solves finished AtCoder contest tasks with Rust.
The questions are sorted in order of difficulty (A, B, C, ...).
I have posted the AC version by the AtCoder judge, but it may not be the ideal solution.
It is expected to be entered according to the constraints in question.
This source code is license-free CC0, but the copyright etc. belongs to each creator for each crate used.
I will update it from time to time.
*/

#[allow(dead_code)]
mod task_a;
#[allow(dead_code)]
mod task_b;
#[allow(dead_code)]
mod task_c;
#[allow(dead_code)]
mod task_d;

fn main() {
    /* 使用方法(Usage): task_[difficult_level]::[contest_name](); */
    task_a::abc066();
}