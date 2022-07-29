fn main() {
    
    let curr_point = (-2, 1);
    let nxt_point = (2, 3);

    let (x1, y1) = curr_point;
    let (x2, y2) = nxt_point;

    if x1 < 0 || x1 > 8 || x2 < 0 || x2 > 8 || y1 < 0 || y1 > 8 || y2 < 0 || y2 > 8 {
        println!("Invalid input provided!");
        return
    }
    if (x1 - x2 as i32).abs() == 1 && (y1-y2 as i32).abs() == 2 || (
        x1 - x2 as i32).abs() == 2 && (y1-y2 as i32).abs() == 1{
        println!("YES");
    }
    else{
        println!("NO");
    }

}
