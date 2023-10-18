// P1 below , uncomment 

// fn add_chars_n(s:String, c:char, nr:u8)->String {
//     let mut new_s:String = s;
//     let mut i=nr;
//     while i>0 {
//         new_s.push(c);
//         i-=1;
//     }

//     new_s
// }


// fn main() {
//     let mut s = String::from("");
//     let mut i = 0;
//     while i < 26 {
//         let c = (i as u8 + 'a' as u8) as char;
//         s = add_chars_n(s, c, 26 - i);

//         i += 1;
//     }

//     print!("{}", s);
// }



// P2 below , uncomment

// fn add_chars_n(s: &mut String, c:char, nr:u8) {
//     let mut i=nr;
//     while i>0 {
//         s.push(c);
//         i-=1;
//     }
// }

// fn main() {
//     let mut s = String::from("");
//     let mut i = 0;
//     while i < 26 {
//         let c = (i as u8 + 'a' as u8) as char;
//         add_chars_n(&mut s, c, 26 - i);

//         i += 1;
//     }

//     print!("{}", s);
// }




// P3 below , uncomment

// fn add_space(s:&mut String,nr:usize){
//     let mut i=nr;
//     while i>0 {
//         s.push(' ');
//         i-=1;
//     }
// }

// fn add_str(s:&mut String,s_2:&str){
//     s.push_str(s_2);
// }

// fn add_integer(s:&mut String,nr:usize){
//     let mut l=0 as usize;
//     let mut count=0 as usize;
//     let mut nr_1=nr;
//     let nr_as_string:String = nr.to_string();
//     while nr_1>0{
//         l+=1;
//         nr_1/=10;
//     }

//     while l>0 {
//         s.push_str(&nr_as_string[count..count+1]);
//         l-=1;
//         count+=1;
//         if count%3==0 && l>0 {
//             s.push('_');
//         }
//     }


// }

// fn add_float(s:&mut String,nr:f32){
//     let float_s = nr.to_string();
//     s.push_str(&float_s);
// }


// fn main(){
//     let mut s=String::from("");
//     add_space(&mut s,40);
//     add_str(&mut s,"I");
//     add_space(&mut s,1);
//     add_str(&mut s,"💚\n");
//     add_space(&mut s,40);
//     add_str(&mut s,"RUST.\n");
//     add_space(&mut s,1);
//     add_str(&mut s,"\n");
//     add_space(&mut s,4);
//     add_str(&mut s,"Most");
//     add_space(&mut s,12);
//     add_str(&mut s,"crate");
//     add_space(&mut s,6);
//     add_integer(&mut s,306437968);
//     add_space(&mut s,11);
//     add_str(&mut s,"and");
//     add_space(&mut s,5);
//     add_str(&mut s,"lastest");
//     add_space(&mut s,9);
//     add_str(&mut s,"is\n");
//     add_space(&mut s,9);
//     add_str(&mut s,"downloaded");
//     add_space(&mut s,8);
//     add_str(&mut s,"has");
//     add_space(&mut s,13);
//     add_str(&mut s,"downloads");
//     add_space(&mut s,5);
//     add_str(&mut s,"the");
//     add_space(&mut s,9);
//     add_str(&mut s,"version");
//     add_space(&mut s,4);
//     add_float(&mut s,2.038); 
//     add_str(&mut s,".\n");
//     add_space(&mut s,20);

//     println!("{}",s);
// }
