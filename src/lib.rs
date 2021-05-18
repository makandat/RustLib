// コマンドライン引数
pub mod a {
  use std::env;

  // コマンドライン引数の数を得る。
  pub fn args_count() -> i32 {
    let argv: Vec<String> = env::args().collect();
    (argv.len() - 1) as i32
  }

  // コマンドライン引数のリストを得る。
  pub fn args() -> Vec<String> {
    let mut argv: Vec<String> = env::args().collect();
    argv.remove(0);
    argv
  }
}

// 変換
pub mod b {
  // 文字列 &str と String
  // &str から String へ
  pub fn to_String(s: &str) -> String {
      String::from(s)
  }
  // 文字列 &str と数
  pub fn str_to_i32(s:&str) -> i32 {
    s.parse().unwrap()
  }
  // 文字列 String と数
  pub fn string_to_i32(s:String) -> i32 {
    s.parse().unwrap()
  }
  pub fn i32_to_string(n:i32) -> String {
      n.to_string()
  }

  // 数値どうし
  pub fn f64_to_i32(f:f64) -> i32 {
      f as i32
  }
  pub fn i32_to_f64(n:i32) -> f64 {
      n as f64
  }
}

// 文字列 String
pub mod c {
  pub fn length(s:String) -> i32 {
    s.len() as i32
  }
  pub fn toupper(s:String) -> String {
    s.to_uppercase()
  }
  pub fn tolower(s:String) -> String {
    s.to_lowercase()
  }
  pub fn strip(s:String) -> String {
    let s1: &str = &s;
    let s2: &str = s1.trim();
    String::from(s2)
  }
}