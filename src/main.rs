//extern crate ffmpeg;

use std::env;

fn main() {
    //intro!
    println!("CRust! by superfroggman, Markussim & linusromland");

    //declare variables
    let mut input = "";
    let mut output = "";

    //Get arguments
    let args: Vec<_> = env::args().collect();

    //Check arguments
    for x in 1..args.len(){
        //help
        if args[x]=="-h"||args[x]=="--help" {
            println!("\nCRrust is a cloud remover. Import mp4, gif of the sky and remove the clouds. \n\n-i (--input) input file location \n\n-o (--output) output file location\n");
        
            return;
        }

        //input file
        if (args[x]=="-i" || args[x]=="--input") && args.len()>=x+2 {
            let ha = x+1;
            input = &args[ha];
        }

        
        //out+put file
        if (args[x]=="-o" || args[x]=="--output") && args.len()>=x+2 {
            let ha = x+1;
            output = &args[ha];
        }
    }

    println!("Input file: {}",input);
    println!("Output file: {}",output);
}