use regex::Regex;
use std::fs;
use csv::{Reader,StringRecord};
use std::path::Path;
use anyhow;
 
use crate::OutPutFormat;

//  Deserialize, Serialize 意思是支持序列化核反序列化
// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct Player {
//     name: String,
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit: u8,
// }
//

pub fn process_csv(input:&str,ouput:String,format:OutPutFormat)  -> anyhow::Result<()>   {
    let mut reader = Reader::from_path(Path::new(input)).unwrap();
    let mut ret = Vec::with_capacity(128);
    let headers:StringRecord = reader.headers().unwrap().clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() -> 将两个迭代器合并为一个元组的迭代器 [(header, record), ..]
        // collect::<Value>() -> 将元组的迭代器转换为 JSON Value
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(json_value);
    }

    match format {
        OutPutFormat::Json=>{
            let json = serde_json::to_string_pretty(&ret).unwrap();
            fs::write(ouput, json)?;
        },
        OutPutFormat::Yaml=>{
            let yaml = serde_yaml::to_string(&ret).unwrap();
            fs::write(ouput, yaml)?;
        },
        OutPutFormat::Csv=>{
            let mut writer = serde_csv_core::Writer::new();
            let mut csv = [0; 128];
            let mut nwritten = 0;

            for record in  &ret {
                nwritten += writer.serialize_to_slice(&record, &mut csv[nwritten..])?;
            }

        },

    }
    // let json = serde_json::to_string_pretty(&ret).unwrap();
    // fs::write(ouput, json)?;
    Ok(())

}


// 处理单个字段
// 去除csv 中容易導致亂行的標點符號
fn remove_special_characters(input: &str) -> String {
    // 使用正则表达式删除 '\t'、'\n' 和 '\r' 字符
    let re = Regex::new(r"[\t\n\r]").unwrap();
    re.replace_all(input, "").to_string()
}

fn escape_quotes(input: &str) -> String {
    // 对包含 '","' 的字段进行转义
    input.replace(r#","#, r#"\","#)
}