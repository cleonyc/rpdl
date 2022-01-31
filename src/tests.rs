
// RPDL - Resource Pack Downloader
// Copyright (C) 2021  cleonyc

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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