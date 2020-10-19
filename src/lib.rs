use wasm_bindgen::prelude::*;

// use ndarray::arr2;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  println!("The Rust function say() received {}", s);
  let r = String::from("hello ");
  return r + s;
}


#[wasm_bindgen]
pub fn is_prime(n: i32) -> String {
  let itter = (n as f64).sqrt() as i32;
  if n <= 1 {
      return "false".to_string();
  }
  for a in 2..itter+1 {
      if n % a == 0 {
          return "false".to_string(); // if it is not the last statement you need to use `return`
      }
  }
  "true".to_string() // last value to return
}

// #[wasm_bindgen]
// pub fn multiply_arr() {
//     let a = arr2(&[[1, 2, 3],
//                    [4, 5, 6]]);

//     let b = arr2(&[[6, 3],
//                    [5, 2],
//                    [4, 1]]);

//     println!("{}", a.dot(&b));
// }
