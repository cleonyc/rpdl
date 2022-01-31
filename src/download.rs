
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


use regex::Regex;
use std::error::Error as StdError;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::{fmt, io};
use urlencoding::decode;

#[derive(Debug)]
pub enum Error {
    AlreadyExists,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AlreadyExists => f.write_str("Resource pack already exists"),
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AlreadyExists => "Resource pack already exists",
        }
    }
}

pub fn smart_download(dot_minecraft_folder: PathBuf, pack_url: &str, rm_color_codes: bool) -> anyhow::Result<()>{
    if pack_url.contains("pvprp.com") && !pack_url.contains("assets/packs") {
        let dl_url = get_link_pvprp(pack_url)?;
        download_pack(dot_minecraft_folder, &dl_url, rm_color_codes)?;
        return Ok(());
    }
    if pack_url.contains("mediafire.com") && !pack_url.contains("https://download") {
        download_pack(dot_minecraft_folder, &get_link_mediafire(&pack_url)?, rm_color_codes)?;
        return Ok(());

    }
    download_pack(dot_minecraft_folder, &pack_url, rm_color_codes)?;

    Ok(())
}

pub fn download_pack(
    mut dot_minecraft_folder: PathBuf,
    pack_url: &str,
    rm_color_codes: bool,
) -> anyhow::Result<()> {
    dot_minecraft_folder.push("resourcepacks");
    let mut file_name = decode(
        &pack_url
            .split("/")
            .last()
            .unwrap()
            .split("?")
            .collect::<Vec<&str>>()[0],
    )
    .unwrap()
    .replace("+", " ")
    .trim_start_matches('!')
    .trim_start()
    .to_string();

    if rm_color_codes {
        let regexp = Regex::new("(?i)§[0-9A-FK-OR]")?;
        file_name = regexp.replace_all(&file_name, "").to_string();
    }
    let mut file_path = dot_minecraft_folder;
    file_path.push(file_name);
    match File::open(file_path.clone()) {
        Ok(_) => return Err(anyhow::Error::from(Error::AlreadyExists)),
        Err(_) => {}
    }
    let mut file = File::create(file_path)?;
    let resp = ureq::get(pack_url).call()?;
    let mut buf_writer = BufWriter::new(&mut file);
    std::io::copy(&mut resp.into_reader(), &mut buf_writer).unwrap();
    Ok(())
}
pub fn get_link_pvprp(url: &str) -> anyhow::Result<String> {
    let resp = ureq::get(&url).call()?;
    let body = resp.into_string().unwrap();
    let series_one = Regex::new(".*zip.*")?;
    let series_two = Regex::new("assets.*[^\"]")?;
    let match_one = series_one.find(&body).unwrap().as_str();
    let match_two = series_two.find(&match_one).unwrap();
    Ok(format!("https://pvprp.com/{}", match_two.as_str()))
}
pub fn get_link_mediafire(url: &str) -> anyhow::Result<String> {
    let regexp = Regex::new("https://download[^\"]*")?;
    let resp = ureq::get(&url).call()?;
    let body = resp.into_string().unwrap();
    let match_one = regexp.find(&body).unwrap().as_str();
    Ok(match_one.to_string())
}