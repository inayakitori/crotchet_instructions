use std::f64::consts::TAU;
use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let curvature: f64 = prompt_user("curvature: ");
    let radius = 1. / curvature.abs().sqrt();
    //if the curvature is negative or zero, have a limited line count
    let line_count: u32 = match curvature {
        ..=0.0 => prompt_user("line count:"),
             _ => (0.5 * TAU * radius).ceil() as u32
    };

    //getting stitches per line
    let mut stitches_per_line: Vec<u32> = Vec::new();
    for line_index in 0..line_count {
        let circumference = TAU * &radius * (line_index as f64).csin(&curvature);
        stitches_per_line.push(circumference.round() as u32);
    }

    dbg!(stitches_per_line);

}

fn prompt_user<N: FromStr + Debug>(prompt: &str) -> N {
    let mut response = String::new();
    let mut parsed_response: Option<N> = None;
    while parsed_response.is_none() {
        response = String::new();
        println!("{}", prompt);
        std::io::stdin()
            .read_line(&mut response)
            .expect("failed to read line");
        response.pop();
        dbg!(&response, &parsed_response);
        parsed_response = response.parse::<N>().ok();
    }

    parsed_response.unwrap()
}

trait CSin: Sized {
    fn csin(self, k: &f64) -> Self {
        self
    }
}

impl CSin for f64{
    fn csin(self, k: &f64) -> Self {
        let r = 1./k.abs().sqrt();
        match k.signum() as i8 {
             1 => (self / r).sin(),
             0 => self,
            -1 => (self / r).sinh(),
             _ => panic!("{}.abs isn't unit or zero", k)
        }
    }
}
