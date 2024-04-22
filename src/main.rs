

use clap::Parser;
use std::path::Path;
#[derive(Parser,Debug)]
#[command(name="rcli",version,about,long_about)]
struct Opts {
    #[command(subcommand)]
    cmd:SubCommands,
}

#[derive(Parser,Debug)]
enum SubCommands{
    #[command(name="csv",about="conver csv to json")]
    Csv(CsvOpts),

}

#[derive(Parser,Debug)]
struct CsvOpts{
    // 检查文件是否存在
    #[arg(short, long, value_parser = verify_file)]
    input:String,
    #[arg(short,long,default_value="out.json")]
    output:String,
    #[arg(short,long,default_value = ",")]
    delimiter:String,
    #[arg(long, default_value_t = true)]
    header:bool,


}

fn main() {
    println!("Hello, world!");
    let opts:Opts = Opts::parse();
    println!("{:#?}",opts);

}

// 检查文件是否存在
fn verify_file(input: &str) -> Result<String,String> {
    if  Path::new(input).exists(){
        Ok(input.into())
    }else {
        Err("file does not exist".into())
    }
}