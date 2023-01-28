use reqwest;
use reqwest::header::{ACCEPT,AUTHORIZATION,CONTENT_TYPE}
use serde::{Deserialize,Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls{
    spotify: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Artist{
    name: String,
    external_url: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Album{
    name: String,
    artists: Vec<Artist>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track{
    
}
