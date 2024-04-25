use rand::prelude::IteratorRandom;
use rand::{thread_rng, Rng, seq::SliceRandom};

pub fn process_genpassword(length:u8, upper:bool, lower:bool, number:bool, symbol:bool) -> anyhow::Result<String>{
    let mut  password = String::new();
    let mut ch =Vec::new();

    if upper {
        ch.extend(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',   'J', 'K', 'L', 'M', 'N',  'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']);
    }
    if lower {
        ch.extend(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l','m', 'n',   'p', 'q', 'r','s', 't', 'u', 'v', 'w', 'x', 'y', 'z']);
    }
    if  number{
        ch.extend(vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    }

    if  symbol {
        ch.extend(vec!['#', '@', '&',  ]);
    }
    let mut rng = thread_rng();

    // 对字符集合进行乱序处理，以增加密码的随机性
    ch.shuffle(&mut rng);

    // 从乱序处理后的字符集合中随机选择字符，生成密码
    password = (0..length).map(|_| {
        let idx = rng.gen_range(0..ch.len());
        ch[idx] as char
    }).collect();

    Ok(password)



}