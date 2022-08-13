mod lib;
mod problems;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let arr = (0..10).collect::<Vec<_>>();
        let mut arr_scan = arr.iter().scan(1, |state, &x| {*state += x; Some(*state)});
        println!("arr_scan: {:?}", arr_scan);
        println!("arr_scan.next: {:?}", arr_scan.next());
        println!("arr_scan.next: {:?}", arr_scan.next());
        println!("arr_scan.next: {:?}", arr_scan.next());
        println!("arr_scan.next: {:?}", arr_scan.next());
        println!("arr_scan.next: {:?}", arr_scan.next());

        let arr_fold = arr.iter().fold(1, |acc, x| acc+x);
        println!("arr_fold: {:?}", arr_fold);
    }
}