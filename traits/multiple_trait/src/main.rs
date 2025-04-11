pub trait Summary{
    fn summarize(&self)->String;
}
pub trait Fix{
    fn fixed(&self)->String;
}
struct User{
    name:String,
    age:u32,
}
impl Summary for User{
    fn summarize(&self)->String{
        return format!("The user name is {} and the age is {}",self.name, self.age);
    }
}
impl Fix for User{
    fn fixed(&self)->String{
        return format!("The user name is {} and the age is {}",self.name, self.age);
    }
}
fn main(){
    let user=User{
        name:String::from("David"),
        age:34,
    };
     notify(&user);
}
 pub fn notify <T: Summary+Fix> (item: &T){
    println!("{}",item.summarize());
    println!("{}",item.fixed());
 }