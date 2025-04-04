
     fn main(){
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.push(5);
           println!(" The elements in the vector v are{:?}",v);

        let second :Option <&i32> =v.get(1);
        match second{
            Some(second)=> println!("The second element is {second}"),
            None=>println!("No second element"),
        }
    
}
