const BOY_SIZE: usize = 20 + 1;
const GIRL_SIZE: usize = 10 + 1;
fn main() {
    let mut arr = [[0; GIRL_SIZE]; BOY_SIZE];
    arr[0][0] = 1;
    for i in 0..BOY_SIZE {
        for j in 0..GIRL_SIZE {
            if j != i && BOY_SIZE - i != GIRL_SIZE - j {
                if i > 0 {
                    arr[i][j] += arr[i - 1][j];
                }
                if j > 0 {
                    arr[i][j] += arr[i][j - 1];
                }
            }
        }
    }
    println!("{}", arr[BOY_SIZE - 2][GIRL_SIZE - 1]);
}
