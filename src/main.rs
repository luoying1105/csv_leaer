use csv::Reader;
use clap::Parser;
use std::path::Path;

struct Player {
    #[serde(rename="Name")]
    name             :String   ,
    #[serde(rename="Position")]
    position         :String   ,
    #[serde(rename="DOB")]
    dob:String   ,
    #[serde(rename="Nationality")]
    nationality      :String   ,
    #[serde(rename="Kit Number")]
    kit_number:String
}
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
    match opts.cmd {
        SubCommands::Csv(csv_opts) => {
            let reader = Reader::from_path(Path::new(&csv_opts.input)).unwrap();
            for result in reader.records() {
                let record = result?;
                println!("{:?}", record);
            }
            // let mut writer = csv::Writer::from_path(Path::new(&csv_opts.output)).unwrap();
            // for result in reader.deserialize() {
            //     let player:Player = result.unwrap();
            //     writer.serialize(player).unwrap();
            // }
        }
    }

}

// 检查文件是否存在
// 跟进程生命周期一致 变量用static
fn verify_file(input: &str) -> Result<String,&'static str> {
    if  Path::new(input).exists(){
        Ok(input.into())
    }else {
        Err("file does not exist" )
    }
}