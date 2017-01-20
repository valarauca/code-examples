

fn movement_example_1() {
  let x = Type::default();
  let y = x;
  assert_eq!(y,x); //COMPILER ERROR!
}

fn movement_example_2() {
  let x = 5.0f64;
  let y = x;
  assert_eq!(y,x); //Test passed 😎
}

fn movement_example_3() {
  
  #[derive(Copy)]
  struct DataType {
     val: u64,
    data: f64
  }
  
  let x = DataType{ val: 5, data: 5.0 };
  let y = x;
  assert_eq!(y,x); //Test passed 😎
}
