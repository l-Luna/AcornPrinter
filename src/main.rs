use std::fs;
use std::path::Path;

fn main(){
    let w = 512;
    let h = 512;
    let mut values = vec![vec![0; w]; h];

    values[200 + 1][200 + 0] = 255;
    values[200 + 3][200 + 1] = 255;
    values[200 + 0][200 + 2] = 255;
    values[200 + 1][200 + 2] = 255;
    values[200 + 4][200 + 2] = 255;
    values[200 + 5][200 + 2] = 255;
    values[200 + 6][200 + 2] = 255;

    fs::write(Path::new("./out.pgm"), write(w, h, values)).unwrap();
}

fn write(w: usize, h: usize, values: Vec<Vec<u8>>) -> Vec<u8>{
    let mut out = Vec::new();

    out.extend_from_slice(b"P5\n");
    out.extend(format!("{w} {h}\n255\n").into_bytes());

    for x in 0..w {
        for y in 0..h {
            out.push(values[x][y]);
        }
    }

    out
}