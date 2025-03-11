#[derive(deride::Without)]
#[without(name = WithoutExampleTwo, derive = Clone, derive = Debug)]
struct WithoutExample {
  field_a: Option<i32>,
  field_b: String,
  #[without]
  field_c: Vec<bool>,
  field_d: i32,
  #[without]
  field_e: f64,
  field_f: (),
}

fn main() {
  let field_a = Some(5);
  let without = WithoutExample {
    field_a,
    field_b: "Hello, world!".into(),
    field_c: vec![false],
    field_d: 500,
    field_e: 1.0,
    field_f: (),
  };

  let without_2: WithoutExampleTwo = without.into();

  assert_eq!(without_2.field_a, field_a);

  dbg!(without_2);
}
