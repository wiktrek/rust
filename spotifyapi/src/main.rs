use reqwest;
use reqwest::header::{ACCEPT,AUTHORIZATION,CONTENT_TYPE}
use serde::{Deserialize,Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls{
    spotify: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Atrist{}