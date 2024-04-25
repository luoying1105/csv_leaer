use std::fmt;
use std::fmt::{Formatter};
use std::path::Path;
use std::str::FromStr;
use clap::Parser;


#[derive(Debug,Copy, Clone)]
pub enum OutPutFormat{
    Json,Yaml,Csv,
}
#[derive(Parser,Debug)]
pub struct CsvOpts{
    // 检查文件是否存在
    #[arg(short, long, value_parser = verify_file)]
    pub input:String,
    #[arg(short,long, )]
    pub output:Option<String>,
    #[arg(short,long,default_value = ",")]
    pub  delimiter:String,
    #[arg(long, default_value_t = true)]
    pub  header:bool,
    #[arg(long, short,value_parser=verify_format,default_value = "json")]
    pub format :OutPutFormat,



}


#[derive(Parser,Debug)]
pub enum SubCommands{
    #[command(name="csv",about="conver csv to json")]
    Csv(crate::CsvOpts),

}

#[derive(Parser,Debug)]
#[command(name="rcli",version,about,long_about)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: crate::SubCommands,

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

// 检查是否支持该格式转换
fn verify_format(input: &str) -> Result<OutPutFormat,anyhow::Error> {
    // parse need to realize FromStr trait
    input.parse::<OutPutFormat>()
}

// 实现特征 就是self.into
// rust 下面实现了from 就是实现了into
impl From<OutPutFormat> for &'static str {
    fn from(format: OutPutFormat) -> Self {
        match format {
            OutPutFormat::Json => "json",
            OutPutFormat::Yaml => "yaml",
            OutPutFormat::Csv => "csv",

        }
    }
}
impl FromStr for OutPutFormat {
    type Err =anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutPutFormat::Json),
            "yaml" => Ok(OutPutFormat::Yaml),
            "csv" => Ok(OutPutFormat::Csv),
           
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
    }}
}

impl fmt::Display for OutPutFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       //  实现from 就是实现into
       write!(f, "{}", Into::<&str>::into(*self))
    }
}