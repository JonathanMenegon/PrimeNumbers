fn main() {
primes_finder(1000000000);
}

fn primes_finder(upto_number:i32){
    let mut toggle:bool = false;
    let mut primes_list:Vec<i32> = vec![2];
    let mut primes_size:usize= 0;
    for number in 2..upto_number {
        toggle = false;
        for prime in 0..primes_size{
            let curr_number = primes_list[prime];
            if curr_number > (number + (curr_number/10 + 1) - 1)/ (curr_number/10 + 1) {
                break;
            }
            if number % curr_number == 0 {
                toggle = true;
                break;
            }
        }
        if toggle == false{
            primes_list.push(number);
            primes_size = primes_size + 1;
        }
    }
    println!("{primes_size}");
}