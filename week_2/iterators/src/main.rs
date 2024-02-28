fn main() {
    // let mut fibonacci = Fibonacci { current: 0, next: 1};

    // for _ in 0..10{
    //     println!("{}",fibonacci.next().unwrap());
    // }

    // Iterator Types:
    // let vec = vec![1,2,3,4,5];

    // for item in vec.iter(){
    //     println!("{}",item);
    // }

    // let mut vec2 = vec![1,2,3,4,5];

    // for item in vec2.iter_mut(){
    //     *item *= 10;
    //     println!("{}",item);
    // }

    // cant use values again
    // for item in vec2.into_iter(){
    //     *item *= 10;
    //     println!("{}",item);
    // }

    // cool thing about iterators
    let numbers = vec![1,2,3,4,5];
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();

    println!("{:?}",even_numbers);

}

struct Fibonacci{
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;

        Some(current)
    }
}
