pub trait Summary{
    fn summarize(&self)->String;
}

struct User{
    name:String,
    age:u32,
}

impl Summary for User{
    fn summarize(&self)->String{
        return format!("The username is {} and thier age is {}",self.name,self.age);
    }
}

fn main(){
    let user=User{
        name:String::from("Hari"),
        age:21,
    };
    println!("{}",user.summarize());
}