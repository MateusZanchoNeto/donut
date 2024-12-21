fn main() {
    let mut c1: f64 = 0.0;
    let mut c2: f64 = 0.0;
    let mut z = [0f64; 1760];
    let mut b = [b' '; 1760];
    print!("\x1b[2J");
    loop {
        b.fill(b' ');
        z.fill(0.0);
        let mut j = 0.0;
        while j < std::f64::consts::TAU {
            let mut i = 0.0;
            while i < std::f64::consts::TAU {
                let c = i.sin();
                let d = j.cos();
                let e = c1.sin();
                let f = j.sin();
                let g = c1.cos();
                let h = d + 2.0;
                let c3 = 1.0 / (c * h * e + f * g + 5.0);
                let l = i.cos();
                let m = c2.cos();
                let n = c2.sin();
                let t = c * h * g - f * e;
                let x = (40.0 + 30.0 * c3 * (l * h * m - t * n)) as i32;
                let y = (12.0 + 15.0 * c3 * (l * h * n + t * m)) as i32;
                let o = x + 80 * y;
                let c4 = (8.0 * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n)) as i32;
                if 22 > y && y > 0 && x > 0 && 80 > x && c3 > z[o as usize] {
                    z[o as usize] = c3;
                    b[o as usize] = b".,-~:;=!*#$@"[c4.clamp(0, 11) as usize];
                }
                i += 0.02;
            }
            j += 0.07;
        }
        print!("\x1b[H");
        for (k, &val) in b.iter().enumerate() {
            if k % 80 == 0 && k != 0 {
                println!();
            }
            print!("{}", val as char);
        }
        c1 += 0.04;
        c2 += 0.02;
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}
