pub trait Summary{
    fn summarize(&self)->String{ // fn summarize created
        return String::from("Summarize");//we expect output as "Summarize"
    }
}

struct User{
    name:String,
    age:u32,
}

impl Summary for User{
   // fn summarize(&self)->String{ //fn summarize created again, so overwritting happens here
     //   return format!("The username is {} and thier age is {}",self.name,self.age);//this output will be printed
    }
    

fn main(){
    let user=User{
        name:String::from("David"),
        age:32,
    };

println!("{}",user.summarize());
}