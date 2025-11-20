struct Dog {
    name: String,
}

impl Dog {
    fn bark(&self) {
        println!("{} says woof", self.name);
    }
}

fn main() {
    let d = Dog { name: "Coco".into() };
    d.bark();
}
