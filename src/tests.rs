use crate::download::{download_pack, get_link_pvprp};
use std::{env, fs};
use std::fs::File;

#[test]
fn raw_downloader() {
    download_pack(
        env::current_dir().unwrap(),
        "https://www.dropbox.com/s/bz3gqvzlzad0ntt/%C2%A7ctest.zip?dl=1",
        false,
    )
        .unwrap();
    File::open("./resourcepacks/§ctest.zip").unwrap();
    fs::remove_file("./resourcepacks/§ctest.zip").unwrap();
    download_pack(
        env::current_dir().unwrap(),
        "https://www.dropbox.com/s/bz3gqvzlzad0ntt/%C2%A7ctest.zip?dl=1",
        true,
    )
        .unwrap();
    File::open("./resourcepacks/test.zip").unwrap();
    fs::remove_file("./resourcepacks/test.zip").unwrap();
}

#[test]
fn pvprp_url() {
    let url = get_link_pvprp("https://pvprp.com/pack?p=1759").unwrap();
    assert_eq!("https://pvprp.com/assets/packs/SpacyLmao/1759/zip/cTashi[32x] Thin Swords.zip", url.split("?").collect::<Vec<&str>>()[0])
}