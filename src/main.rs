fn main() {
    println!("rust8_control_flow");
    let food = "cookies";
    if food == "cookies" {
        println!("the food indeed contains  cookies and tea only ");
    } else {
        println!("I like this food even though it not contains enough cookies")
    }
    if food != "bread"{
        println!("it is right meal ");
    } else if food=="fruit" {
        println!("fresh fruits are very testy food");
     } else if food=="cookies" {
        println!("fresh cookies are very testy food");
     } else if food=="fruit"&&food=="cookies" {
        println!("it is fresh and reach meal");
     }
    else {
        println!("I like this food even thoug
            h it not contains fruits neither cookies");
    }
}
