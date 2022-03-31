fn main(){
    //let x = 5; 값 못바꿈
    let mut x = 5;
    println!("x값: {}", x);
    x = 6;
    println!("x값: {}", x);
    
    const MAX_POINT: u32 = 100_000;
    println!("MAX_POINT값: {}", MAX_POINT);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("y값: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces값: {}", spaces);

    //let mut spaces1 = "   ";
    //spaces1 = spaces1.len(); 문자형 --> 숫자형 에러
    let spaces = "                  ";
    println!("spaces값: {}", spaces.len());
    
    let x1 = 2.0; // f64
    let y1: f32 = 3.0; // f32    
    println!("x1값: {}", x1);
    println!("y1값: {}", y1);

    //계산
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;    
    println!("sum,difference,product,quotient,remainder:{},{},{},{},{}", sum,difference,
    product,quotient,remainder);

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t,f:{},{}",t,f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c,z,heart_eyed_cat: {},{},{}",c,z,heart_eyed_cat);

    //튜플
    let tup: (i32, f64, u8) = (500,6.4,1);
    //바로 출력 못함
    let (x2,y2,z2) = tup;
    println!("x2,y2,z2:{},{},{}", x2,y2,z2);
    //또는
    println!("tup.0,tup.1,tup.2:{},{},{}", tup.0,tup.1,tup.2);

    //배열
    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];   
 
    println!("a[0],month[0]: {},{}", a[0],months[0]);          
}              