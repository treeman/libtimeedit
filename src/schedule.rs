use time;
use time::Tm;
use std::collections::TreeMap;

use request::request;
use parse;
use typeinfo::{ TypeInfo, DataId, Type };
use event::{ Event };

pub fn schedule(infos: Vec<TypeInfo>, from: Tm, to: Tm, base: &str) -> Vec<Event> {
    if infos.is_empty() {
        return Vec::new();
    }

    let ids = infos.iter().map(|x| x.id).collect();
    schedule_from_ids(ids, from, to, base)
}

pub fn schedule_from_ids(ids: Vec<DataId>, from: Tm, to: Tm, base: &str) -> Vec<Event> {
    if ids.is_empty() {
        return Vec::new();
    }

    // Cannot mix different data types in a single request, so partition them.
    let mut partition: TreeMap<Type, Vec<DataId>> = TreeMap::new();
    for id in ids.iter() {
        let typ = id.typ;
        if !partition.contains_key(&typ) {
            partition.insert(typ, vec![*id]);
        } else {
            partition[typ].push(*id);
        }
    }

    // Merge results of separate requests.
    let mut res = Vec::new();
    for p in partition.values() {
        res.push_all(schedule_from_single_ids(p.clone(), from, to, base)[]);
    }
    res.sort();
    res
}

/// All id types are the same.
///
/// Will make a single request.
fn schedule_from_single_ids(ids: Vec<DataId>, from: Tm, to: Tm, base: &str) -> Vec<Event> {
    if ids.is_empty() {
        return Vec::new();
    }

    let mut objects = String::new();
    for id in ids.iter() {
        if !objects.is_empty() {
            objects.push_str("%2C");
        }
        objects.push_str(id.to_string()[]);
    }
    let date_format = "%y%m%d";
    let url = format!("{}/ri.csv?sid=3&p={}-{}&objects={}",
                base,
                time::strftime(date_format, &from).unwrap(),
                time::strftime(date_format, &to).unwrap(),
                objects);
    let txt = request(url[]);

    parse::schedule_res(txt[])
}

