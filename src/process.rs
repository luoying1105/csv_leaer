use std::ffi::CString;
use std::fs;
use csv::Reader;
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow;
//  Deserialize, Serialize 意思是支持序列化核反序列化
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}


pub fn process_csv(input:&str,ouput: &str)  -> anyhow::Result<()>   {
    let mut reader = Reader::from_path(Path::new(input)).unwrap();
    let mut ret = Vec::with_capacity(128);
    let header:StringRecord = reader.header()?.clone();
    for result in reader.deserialize() {
        let player: Player = result.unwrap();
        // header.iter() 使用的是gg'i't
        ret.push(player)
    }
    let json = serde_json::to_string_pretty(&ret).unwrap();
    fs::write(ouput, json)?;
    Ok(())

}


// 处理单个字段
// 去除csv 中容易導致亂行的標點符號
fn process_field(field: &str) -> String {
    let re = Regex::new(r#"[,\n\r\t"]"#).unwrap();
    let escaped_field = re.replace_all(field, |caps: &regex::Captures| {
        match caps.get(0).unwrap().as_str() {
            "," => r#"\,"#,
            "\n" => r#""#,
            "\r" => r#""#,
            "\t" => r#""#,
            "\"" => r#"\""#,
            "\'" => r#"\'"#,
            _ => "",
        }
            .to_string()
    });
    escaped_field.into_owned()
}