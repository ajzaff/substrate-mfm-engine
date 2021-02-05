mod lib;
mod model;

fn main() {
  let sites: [lib::Element; 4];
  let bounds = (2, 2);

  let m = model::Model::new(&sites, bounds);

  println!("Hello, world {:?}!", m);
}
