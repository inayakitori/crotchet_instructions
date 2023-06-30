use std::f64::consts::TAU;
use std::fmt::{Debug, Display, Formatter, write};
use std::ops::Deref;
use std::str::FromStr;
use crate::Stitch::{DECREASE, INCREASE, SINGLE};

fn main() {
    let curvature: f64 = prompt_user("curvature: ");
    let radius = 1. / curvature.abs().sqrt();
    //if the curvature is negative or zero, have a limited line count
    let line_count: usize = match curvature {
        ..=0.0 => prompt_user("line count:"),
        _ => (0.5 * TAU * radius).ceil()
    } as usize;

    //getting stitches per line
    let mut stitches_per_line: Vec<i32> = Vec::new();
    for line_index in 0..line_count {
        let circumference = (line_index as f64).radius_to_circumference(&curvature);
        stitches_per_line.push(circumference.round() as i32);
    }

    //forward difference operation
    let mut delta_stitches : Vec<i32> = Vec::new();
    for line_index in 1..line_count {
        delta_stitches.push((stitches_per_line[line_index] - stitches_per_line[line_index - 1]));
    }

    let stitch_info = LineInstructions::generate(&*delta_stitches, &stitches_per_line[..]);

    println!("{}", stitch_info);
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
        parsed_response = response.parse::<N>().ok();
    }

    parsed_response.unwrap()
}

#[derive(Debug, Clone)]
struct LineInstructions(Vec<LineInstruction>);

impl LineInstructions {
    fn generate(delta_stitches: &[i32], stitches: &[i32]) -> LineInstructions {
        let r = LineInstructions(
            delta_stitches.iter().zip(stitches.iter()).map( |(&delta_stitches, &stitches)| {
                LineInstruction::generate(stitches, delta_stitches)
            }).collect()
        );
        r
    }
}

impl Display for LineInstructions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        for i in 0..self.0.len(){
            let line_instruction = &self.0[i];
            write!(f,"line #{}, {}\n", i+1, line_instruction)?;
        }
        Ok(())
    }
}

impl Deref for LineInstructions{
    type Target = Vec<LineInstruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
struct LineInstruction {
    pub stitches: i32,
    pub increase_stitches: i32,
    pub stitch_info: Vec<Stitch>
}

impl LineInstruction {
    fn generate(stitches: i32, increase_stitches: i32) -> LineInstruction {

        let mut stitch_info : Vec<Stitch> = Vec::with_capacity(stitches as usize);

        let increases_per_stitch: f64 = increase_stitches as f64 / stitches as f64;
        let mut current_increase_count: i32 = 0;
        for stitch in 1..=stitches {
            let approximate_current_increase_count = stitch as f64 * increases_per_stitch;
            let increase_amount = (approximate_current_increase_count - current_increase_count as f64).floor() as i32;
            if approximate_current_increase_count.abs() > i32::abs(current_increase_count) as f64 {
                //should apply correct number of increases
                current_increase_count += increase_amount;
            }
            stitch_info.push(Stitch::from(increase_amount));
        }

        LineInstruction {
            stitches,
            increase_stitches,
            stitch_info,
        }
    }
}

impl Display for LineInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut stitch_text = String::new();
        let mut sc_count = 0;
        for i in 0..self.stitch_info.len() {
            let stitch = &self.stitch_info[i];
            let next_stitch_single = self.stitch_info.get(i+1)
                .map_or(false, |next_stitch| next_stitch == &SINGLE);
            let final_stitch = i == self.stitch_info.len() - 1;

            if stitch == &SINGLE {
                sc_count += 1;
                if !next_stitch_single {// end
                    if sc_count == 1 {
                        stitch_text.push_str(&*format!("{},", stitch));
                    } else {
                        stitch_text.push_str(&*format!("{},", sc_count));
                    }
                    sc_count = 0
                }
            } else {
                stitch_text.push_str(&*format!("{},", stitch));
            }
        }
        stitch_text.pop();
        write!(f, "{} + {} stitches: {}", self.stitches, self.increase_stitches, stitch_text)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Stitch{
    DECREASE(i32),
    SINGLE,
    INCREASE(i32)
}

impl Display for Stitch{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<i32> for Stitch {
    fn from(increase_count: i32) -> Self {

        match increase_count {
            ..=-1 => DECREASE(-increase_count),
            0 => SINGLE,
            1.. => INCREASE(increase_count),
        }
    }
}

impl From<&Stitch> for String {
    fn from(stitch: &Stitch) -> Self {
        match stitch {
            DECREASE(1) => {format!("dec")}
            DECREASE(i) => {format!("dec{}", i)}
            SINGLE => {format!("sc")}
            INCREASE(1) => {format!("inc")}
            INCREASE(i) => {format!("inc{}", i)}
        }
    }
}

trait RadiusToCircumference: Sized {
    fn radius_to_circumference(self, _: &f64) -> Self {
        self
    }
}

impl RadiusToCircumference for f64{
    fn radius_to_circumference(self, k: &f64) -> Self {
        let r = 1./k.abs().sqrt();
        match k{
            0.    => TAU * self,
            0. .. => TAU * (self / r).sin() /k.abs().sqrt(),
            ..=0. => TAU * (self / r).sinh() /k.abs().sqrt(),
             _ => panic!("{}.abs isn't unit or zero", k)
        }
    }
}
