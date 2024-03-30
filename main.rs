//alfabe=a,b,c,d,e
fn main(){
    let kelime:String =String::from("eabaeds");
    let mut yedek:String=String::new();
    println!("{}",kelime);
    for i in 0..=kelime.len(){
        yedek=(&kelime[i..=i]).to_string();
        if yedek != "a" && yedek != "b" &&yedek != "c" &&yedek != "d" &&yedek != "e" {
            println!("Bilinmeyen harf!");
            break;

        }
        else {
        if i!=0{
            if yedek=="b" || yedek =="c" ||yedek =="d"{
                if &kelime[i-1..=i-1].to_string()=="b" || &kelime[i-1..=i-1].to_string()=="c" || &kelime[i-1..=i-1].to_string()=="d"{
                    println!("Tanımlanmayan kelime")
                  }

            }
        }
      }
  }
}
   
