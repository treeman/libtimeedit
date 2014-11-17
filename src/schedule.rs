use time;
use time::Tm;

use request::request;
use parse;
use info::{ TypeInfo, Entry };
use config::Config;

pub fn schedule(infos: Vec<TypeInfo>, from: Tm, to: Tm, conf: &Config) -> Vec<Entry> {
    assert!(infos.len() > 0);

    let mut objects = String::new();
    for info in infos.iter() {
        if !objects.is_empty() {
            objects.push_str("%2C");
        }
        objects.push_str(info.id.to_string()[]);
    }
    let date_format = "%y%m%d";
    let url = format!("{}/ri.csv?sid=3&p={}-{}&objects={}",
                conf.base,
                time::strftime(date_format, &from).unwrap(),
                time::strftime(date_format, &to).unwrap(),
                objects);
    let txt = request(url[]);

    parse::schedule_res(txt[])
}

//pub fn schedule_from_ids(ids: Vec<String>, from: Tm, to: Tm, conf: &Config) -> Vec<Entry> {
    // Sanity check, cannot mix different ids
//}

