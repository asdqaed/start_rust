//3.3함수

fn main(){
    println!("hello, hamster!");
    
    another_function(2,1);
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("y값: {}", y);
    let x = five();
    println!("x값,five()값: {},{}", x,five());
}

fn another_function(x: i32,y: i32) {
    println!("햄스터 {}명", x);
    println!("햄스터 {}살", y);
}

fn five() -> i32 {
    5
}