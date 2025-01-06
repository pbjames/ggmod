use std::io::{self, BufRead};

use crate::gamebanana::models::modpage::GBModPage;

use super::{
    gamebanana::builder::{FeedFilter, SearchBuilder, SearchFilter},
    modz::LocalCollection,
};

pub fn search(
    page: usize,
    page_size: Option<usize>,
    name: Option<String>,
    _featured: bool,
    popular: bool,
    recent: bool,
) {
    let entries = SearchBuilder::new()
        .per_page(page_size.unwrap_or(15))
        .with_sort(if recent {
            FeedFilter::Recent
        } else if popular {
            FeedFilter::Popular
        } else {
            FeedFilter::Featured
        })
        .by_search(SearchFilter::Name {
            search: &name.unwrap_or(String::from("")),
            game_id: 11534,
        })
        .build()
        .read_page(page)
        .expect("Couldn't get search results");
    for entry in entries {
        let mut name = entry.name.clone();
        name.truncate(35);
        let mut desc = entry.description.clone();
        desc.truncate(50);
        let views = entry.view_count;
        println!("{name:<35} - {desc:<50} :: {views} views");
    }
}

pub fn list_all(col: LocalCollection) {
    for mod_ in col.mods() {
        println!(
            "[{}] [{}] {}: {}",
            if mod_.staged { "+" } else { " " },
            mod_.character,
            mod_.id,
            mod_.name
        )
    }
}

pub fn download(mut col: LocalCollection, mod_id: usize, do_install: bool) {
    let gbmod = GBModPage::build(mod_id).expect("Couldn't get online mod page");
    let opts = &gbmod.files;
    for (i, f) in opts.iter().enumerate() {
        println!("[{}] {:?}", (i + 1), f.file);
    }
    println!("Choose index:");
    let input = choose_num() - 1;
    col.register_online_mod(gbmod, input)
        .expect("Couldn't download mod");
    if do_install {
        install(col, mod_id)
    }
}

pub fn install(mut col: LocalCollection, mod_id: usize) {
    col.apply_on_mod(mod_id, Box::new(|mod_| mod_.stage()))
        .expect("add ");
}

pub fn uninstall(mut col: LocalCollection, mod_id: usize) {
    col.apply_on_mod(mod_id, Box::new(|mod_| mod_.unstage()))
        .expect("couldnt rempve stuf");
}

/// We use this since the user won't necessarily know what files a mod will include
/// beforehand
fn choose_num() -> usize {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let input = iterator.next().unwrap().unwrap();
    input.trim().parse::<usize>().unwrap()
}
