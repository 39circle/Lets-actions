use Lets_actions::my_func;


fn main() {
    let data = my_func(0..10, Box::new(|x| {
        x+x
    }));
    println!("{:?}", data);
}