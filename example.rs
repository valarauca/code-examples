

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
