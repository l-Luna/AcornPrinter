use std::env::args;
use std::fs;
use std::path::Path;

fn main(){
    let cmds: Vec<String> = args().collect();
    let w: usize = str::parse(&*cmds[1]).unwrap();
    let h: usize = str::parse(&*cmds[2]).unwrap();
    let offset: usize = str::parse(&*cmds[3]).unwrap();
    let mut values = vec![vec![0; w]; h];

    values[offset + 1][offset + 0] = 255;
    values[offset + 3][offset + 1] = 255;
    values[offset + 0][offset + 2] = 255;
    values[offset + 1][offset + 2] = 255;
    values[offset + 4][offset + 2] = 255;
    values[offset + 5][offset + 2] = 255;
    values[offset + 6][offset + 2] = 255;

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