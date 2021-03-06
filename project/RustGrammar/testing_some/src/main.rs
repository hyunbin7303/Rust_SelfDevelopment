struct Name {
    first_name: String,
    middle_name: Option<String>, // middle_name can be empty
    last_name: String,
}
fn get_full_name(fname: &str, lname: &str, mname: Option<&str>) -> String { // middle name can be empty
  match mname {
    Some(n) => format!("{} {} {}", fname, n, lname),
    None => format!("{} {}", fname, lname),
  }
}
  

fn take_fifth(value:Vec<i32>) ->Option<i32> {
  if value.len() <4 {
    None
  }
  else{
    Some(value[4])
  }
}

fn handle_option(my_options: Vec<Option<i32>>){
  for item in my_options {
    match item {
      Some(number) => println!("Got a {}", number),
      None => println!("The vec is too short")
    }
  }
} 


struct StructTest{
  a: String,
  b: Option<String>,
  c: Option<String>,
  d: Option<i32>,
  e: Option<i64>,
  f: Option<f32>
}
struct Person {
  job: Option<Job>,
}
#[derive(Clone, Copy)]
struct Job {
  phone_number: Option<PhoneNumber>,
}
#[derive(Clone, Copy)]
struct PhoneNumber {
  area_code: Option<u8>,
  number: u32,
}

impl Person {

  // Gets the area code of the phone number of the person's job, if it exists.
  fn work_phone_area_code(&self) -> Option<u8> {
      // This would need many nested `match` statements without the `?` operator.
      // It would take a lot more code - try writing it yourself and see which
      // is easier.
      self.job?.phone_number?.area_code
  }
}

 fn main() {
  println!("{}", get_full_name("Galileo", "Galilei", None));
  println!("{}", get_full_name("Leonardo", "Vinci", Some("Da")));

  let new_vec = vec![1,2];
  let big_vec = vec![1,2,3,4,5,6];
  let mut option_vec = Vec::new();
  option_vec.push(take_fifth(new_vec));
  option_vec.push(take_fifth(big_vec));
  handle_option(option_vec);

  let my_num = 20;
  let single_ref = &my_num;
  let double_ref = &&single_ref;
  let triple_ref = &&&double_ref;

  println!("Number displays: {}", triple_ref);


  let p = Person {
    job: Some(Job {
        phone_number: Some(PhoneNumber {
            area_code: Some(61),
            number: 439222222,
        }),
    }),
  };
  p.work_phone_area_code();
  println!("{:?}",Some(p.work_phone_area_code()));
}


// &str is a simple string. 
// let my_var = "Hello" -> you create a &str. which is very fast.println!

// String is a pointer, with data on the heap.println!
