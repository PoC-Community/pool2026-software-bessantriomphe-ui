use std::fs;
use clap::{Parser};
use colored::Colorize; 
use std::process;


#[derive(Parser)]
struct Args {
    pattern: String,
    file: String,
    #[arg(short = 'i', long = "case-insensitive")]
    case_insensitive: bool,
    #[arg(short = 'n', long = "line-numbers")]
    line_numbers: bool,
}


fn search_in_file(args : &Args) ->std::io::Result<()>{
    let content=fs::read_to_string(&args.file)?;
    let search_pattern=if 
    args.case_insensitive{args.pattern.to_lowercase()}else{args.pattern.to_string()};
    for (idx,line) in content.lines().enumerate(){
        let num=idx+1;
        let line_to_check=if args.case_insensitive{line.to_lowercase()}else{line.to_string()};
        if line_to_check.contains(&search_pattern){
            let highlighted_line= highlighted_pattern(line.to_string(), args.pattern.clone(), args.case_insensitive);
            if args.line_numbers{
                println!("{}:{}",num.to_string().cyan(),highlighted_line);
            }
            else{
                println!("{}",highlighted_line);
            }
        };
        
    }
    Ok(())
}


fn highlighted_pattern(line:String,pattern:String,case_insensitive:bool) -> String{
    if case_insensitive{
    let mut content=String::new();
    let pattern_search=pattern.to_lowercase();
    let line_to_check=line.to_lowercase();
    let mut idx=0;
    
    for (pos,result) in line_to_check.match_indices(&pattern_search){
        content.push_str(&line[idx..pos]);   
        content.push_str(&line[pos..pos+result.len()].red().bold().to_string());
        idx=pos+result.len();
        }
        content.push_str(&line[idx..]);
        content
    }
    else{
        line.replace(&pattern,&pattern.red().bold().to_string())
    }
    
}

fn main() {
    let args = Args::parse();

    if let Err(e) = search_in_file(&args) {
        eprintln!("Erreur : {}", e);
        process::exit(1);
    }
}