#[derive(Clone, deride::Without)]
#[without(name = WithoutExampleTwo, derive = Clone, derive = Debug)]
#[without(name = WithoutExampleThree, derive = Clone, derive = Debug)]
pub(crate) struct WithoutExample {
  #[without(WithoutExampleThree)]
  field_a: Option<i32>,
  field_b: String,
  #[without(WithoutExampleTwo)]
  field_c: Vec<bool>,
  field_d: i32,
  #[without(WithoutExampleTwo)]
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

  let without_2: WithoutExampleTwo = without.clone().into();

  assert_eq!(without_2.field_a, field_a);

  dbg!(without_2);

  let without_3: WithoutExampleThree = without.into();

  dbg!(without_3);
}
