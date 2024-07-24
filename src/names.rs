use std::{array, collections::HashMap, error::Error};

use serde::{Deserialize, Serialize};
const START_YEAR: usize = 1984;
const YEARS: usize = 40;
#[derive(PartialEq, Eq, Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Rating {
    Good,
    Bad,
}
#[derive(Deserialize, Serialize)]
pub struct NameEntry {
    pub name: String,
    pub year_count: [u32; 8],
    pub sex: u8,
    pub total: u32,
    pub comments: String,
    pub rating: Option<Rating>,
}

impl NameEntry {
    pub fn new(name: String, info: Info) -> Self {
        let mut five_years = info
            .year_count
            .array_chunks()
            .map(|a: &[u32; 5]| a.iter().sum::<u32>());
        Self {
            name,
            sex: info.sex,
            total: info.year_count.iter().sum(),
            year_count: array::try_from_fn(|_| five_years.next())
                .expect("could not aggregate year_count"),
            comments: String::new(),
            rating: None,
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct Info {
    pub year_count: [u32; 40],
    pub sex: u8,
}

/*fn load()->Result<HashMap<String, Info>, Box<dyn Error>>{
    let input = std::fs::read_to_string("OGDEXT_VORNAMEN_1.csv")?;
    let mut names = HashMap::new();
    for line in input.lines().skip(1){
        let [year, _, sex, name,count] = line.split(";").next_chunk().or(Err("blub"))?;
        let year: usize = year.parse()?;
        let count: u32 = count.parse()?;
        let sex = sex.parse()?;
        let info = names.entry(name).or_insert_with(||Info{year_count:[0;40], sex});
        info.year_count[year - START_YEAR] += count;
        //entries.push(NameEntry{name, loc: loc.parse()?, year, sex: sex.parse()?, count});
    }
    Ok(names.into_iter().map(|(name,info)|(name.to_string(),info)).collect())
}
fn serialize(names: &HashMap<String, Info>, writer: &mut impl Write) -> Result<(), Box<dyn Error>>{
    writer.write(&START_YEAR.to_le_bytes())?;
    writer.write(&YEARS.to_le_bytes())?;
    writer.write(&names.len().to_le_bytes())?;
    for (name,info) in names{
        writer.write(&[info.sex, name.as_bytes().len() as u8])?;
        writer.write(name.as_bytes())?;
        for count in info.year_count{
            writer.write(&count.to_le_bytes())?;
        }
    }
    Ok(())
}*/

pub fn deserialize(
    reader: &mut impl std::io::Read,
) -> Result<HashMap<String, Info>, Box<dyn Error>> {
    let mut names = HashMap::new();
    let mut u32_buf = [0; 4];
    let mut buf = [0; 256];
    reader.read_exact(&mut u32_buf)?;
    let start_year = u32::from_le_bytes(u32_buf) as usize;
    assert_eq!(START_YEAR, start_year);
    reader.read_exact(&mut u32_buf)?;
    let years = u32::from_le_bytes(u32_buf) as usize;
    assert_eq!(YEARS, years);
    reader.read_exact(&mut u32_buf)?;
    let names_len = u32::from_le_bytes(u32_buf) as usize;
    for _ in 0..names_len {
        reader.read_exact(&mut u32_buf[0..2])?;
        let sex = u32_buf[0];
        let name_len = u32_buf[1] as usize;
        reader.read_exact(&mut buf[0..name_len])?;

        let name = String::from_utf8(buf[0..name_len].to_vec())?;
        reader.read_exact(&mut buf[0..YEARS * 4])?;
        let mut counts = buf[0..YEARS * 4]
            .array_chunks()
            .map(|&x| u32::from_le_bytes(x));
        names.insert(
            name,
            Info {
                sex,
                year_count: array::try_from_fn(|_| counts.next()).unwrap(),
            },
        );
    }

    Ok(names)
}
/*
fn main() -> Result<(),Box<dyn Error>>{
    //let names = load()?;
    //serialize(&names, &mut std::fs::File::create("names.bin")?)?;
    let mut data = std::io::Cursor::new(include_bytes!("../names.bin"));
    //let mut data = std::fs::File::open("names.bin")?;
    let names = deserialize(&mut data)?;
    //assert_eq!(names,names2);
    let mut filtered: Vec<_> = names.iter().filter(|&(name, info)|name.len()<5 && info.sex==1).collect();
    filtered.sort_by_key(|&(name,info)| (info.year_count.iter().skip(35).sum::<u32>(), name));
    let count = filtered.iter().count();
    println!("{count} short names");
    let limit = 50;
    for &(name, info) in filtered.iter().take(limit){
            println!("{} {:?} {}",name, info.year_count, info.year_count.iter().sum::<u32>());
    }
    /*if count > limit{
    println!("...");
    for &(name, info) in filtered.iter().skip(count-50){
            println!("{} {:?} {}",name, info.year_count, info.year_count.iter().sum::<u32>());
    }
    }*/
    let mut query = String::new();
    loop{
        query.clear();
        std::io::stdin().read_line(&mut query)?;
        let Some(occurences) = names.get(query.trim()) else{
            println!("not found");
            continue;
        };
        println!(" {query} {:?} {}",occurences.year_count,occurences.year_count.iter().sum::<u32>());
    }

}*/
