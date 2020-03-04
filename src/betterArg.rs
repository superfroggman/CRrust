use std::env;

fn main() {
    println!("CRrust!");
    
    let mut input = "";

    //get arguments
    let args: Vec<_> = env::args().collect();

    //Print all arguments
    for x in 1..args.len(){
        println!("The {} argument is {}",x, args[x]);
    }

    //Check arguments
    for x in 1..args.len(){
        //help
        if(args[x]=="-h"||args[x]=="--help"){
            println!("CRrust is a cloud remover. Import mp4, gif of the sky and remove the clouds. \n\n-i (--input) input file location \n\n-o (--output) output file location");
        
            return;
        }

        if(args[x]=="-i"){
            input = args[x+1]
        }

    }

    if(args[1]==""){
        println!("nice");
    }
}