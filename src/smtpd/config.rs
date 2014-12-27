// Copyright (c) 2014 Toby DiPasquale <toby@cbcg.net>
use std::error::FromError;
use std::io;

pub struct Config {
    pub ident_string: String,
    pub listen_address: String,
    pub max_idle_seconds: u32,
    pub max_message_size: u32,
    pub serving_domain: Option<String>,
}

pub fn load_config(path: Option<String>) -> Result<Config, ::errors::SError> {
    match path {
        Some(path) => {
            match io::File::open(&Path::new(path)) {
                Ok(file) => { read_config(io::BufferedReader::new(file)) }
                Err(e) => { Err(FromError::from_error(e)) }
            }
        }
        None => { Ok(empty_config()) }
    }
}

fn empty_config() -> Config {
    Config {
        ident_string: "smtpd".to_string(),
        listen_address: "0.0.0.0:25".to_string(),
        max_idle_seconds: 120u32,
        max_message_size: 16777216u32,
        serving_domain: None,
    }
}

fn read_config(mut f: io::BufferedReader<io::File>) -> Result<Config, ::errors::SError> {
    let mut config = empty_config();
    let mut count = 0u;

    for s in f.lines() {
        count = count + 1;

        let l = try!(s);
        let line = l.trim();
        if line.len() == 0 || line.as_bytes()[0] == b'#' {
            continue;
        }

        let v: Vec<&str> = line.splitn(1, ' ').collect();
        if v.len() != 2 {
            return Err(FromError::from_error(::errors::ParseError::InvalidLineFormat(count)));
        }
        match v[0].trim() {
            "domain" => {
                config.serving_domain = Some(v[1].trim().to_string())
            }
            "ident" => {
                config.ident_string = v[1].trim().to_string()
            }
            "listen" => {
                config.listen_address = v[1].trim().to_string()
            }
            "maxidle" => { 
                config.max_idle_seconds = match v[1].parse::<u32>() {
                    Some(i) => { i }
                    None => {
                        return Err(FromError::from_error(::errors::ParseError::InvalidArgument(count)))
                    }
                }
            }
            "maxmsgsize" => {
                config.max_message_size = match v[1].parse::<u32>() {
                    Some(i) => { i }
                    None => {
                        return Err(FromError::from_error(::errors::ParseError::InvalidArgument(count)))
                    }
                }
            }
            _ => {
                return Err(FromError::from_error(::errors::ParseError::InvalidArgument(count)))
            }
        }
    }
    Ok(config)
}

