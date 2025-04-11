pub trait Summary{
    fn summarize(&self)->String;
}
struct Fix{
    rate:u32,
    quanity:u32,
}
struct Store{
    table_name:String,
    available:bool,
}
struct User{
    name:String,
    age:u32,
}
impl Summary for Fix{
    fn summarize(&self)->String{
        return format!("The rate is {} and the quanity is {}",self.rate, self.quanity);
        }
}
impl Summary for Store{
    fn summarize(&self)->String{
        return format!("The table_name is {} and the availability is {}",self.table_name, self.available);
        }
}
impl Summary for User{
    fn summarize(&self)->String{
        return format!("The name is {} and the age is {}",self.name, self.age);
        }
}

fn main(){
    let user=User{
        name:String::from("David"),
        age:35,
    };

    let store=Store{
        table_name:String::from("Dinner Table"),
        available:true,
    };
    let fix=Fix{
        rate:400,
        quanity:3,
    };
 
 
    notify(&user);//calling notify function in main function
    notify(&fix);    
    notify(&store);
}

pub fn notify(item: &impl Summary){ //we can pass parameter in traits
    println!("{}",item.summarize());
}