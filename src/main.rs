use clap::Parser;
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
    #[arg(short,long)]
    input:String,
    #[arg(short,long,default_value="out.json")]
    output:String,
    #[arg(short,long,default_missing_value = ",")]
    delimiter:String,
    #[arg(long, default_value_t = true)]
    header:bool,


}

fn main() {
    println!("Hello, world!");
    let opts:Opts = Opts::parse();
    println!("{:#?}",opts);

}
