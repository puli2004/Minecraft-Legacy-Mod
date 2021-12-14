#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
mod minecraft;

#[skyline::main(name = "libenderman_voice_effect")]
pub fn main(){
	minecraft::install();
}