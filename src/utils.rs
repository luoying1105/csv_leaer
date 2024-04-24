use std::path::Path;
use clap::Parser;




#[derive(Parser,Debug)]
pub struct CsvOpts{
    // 检查文件是否存在
    #[arg(short, long, value_parser = verify_file)]
    pub input:String,
    #[arg(short,long,default_value="out.json")]
    pub output:String,
    #[arg(short,long,default_value = ",")]
    pub(crate) delimiter:String,
    #[arg(long, default_value_t = true)]
    pub(crate) header:bool,


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