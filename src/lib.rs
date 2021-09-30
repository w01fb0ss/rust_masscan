//! A module for masscan.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::Command;
use std::str;

#[macro_use]
extern crate log;

pub type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default)]
pub struct Masscan {
    pub sudo: bool,
    pub system_path: String,
    pub args: Vec<String>,
    pub ports: String,
    pub ranges: String,
    pub rate: String,
    pub exclude: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Ports>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ports {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
}

impl Masscan {
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use rust_masscan::Masscan;
    ///
    /// let other_args: Vec<String> = vec!["--banners".to_string()];
    ///
    /// let mas = Masscan::default()
    ///         .set_system_path("/usr/local/bin/masscan".to_string())
    ///         .set_ports("22,8080-8100".to_string())
    ///         .set_ranges("xx.xx.xx.xx,yy.yy.yy.yy".to_string())
    ///         .set_rate("10000".to_string())
    ///         .set_other_args(other_args);
    /// let result = mas.run();
    //  println!("{:?}", result);
    /// ```
    pub fn set_system_path(mut self, system_path: String) -> Masscan {
        self.system_path = system_path;
        self
    }
    pub fn set_other_args(mut self, args: Vec<String>) -> Masscan {
        self.args = args;
        self
    }
    pub fn set_ports(mut self, ports: String) -> Masscan {
        self.ports = ports;
        self
    }
    pub fn set_ranges(mut self, ranges: String) -> Masscan {
        self.ranges = ranges;
        self
    }
    pub fn set_rate(mut self, rate: String) -> Masscan {
        self.rate = rate;
        self
    }
    pub fn set_exclude(mut self, exclude: String) -> Masscan {
        self.exclude = exclude;
        self
    }

    pub fn set_sudo(mut self) -> Masscan {
        self.sudo = true;
        self
    }

    pub fn run(&self) -> BoxResult<Vec<Info>> {
        let mut args: Vec<&str> = vec!["-p", self.ports.as_str(), "--range", self.ranges.as_str()];
        let other_args: Vec<&str> = self.args.iter().map(|x| x.as_str()).collect();
        args.extend(other_args.iter().cloned());
        args.push("--rate");
        args.push(self.rate.as_str());
        if !self.exclude.is_empty() {
            args.push("--exclude");
            args.push(self.exclude.as_str());
        }
        args.push("--wait");
        args.push("0");
        args.push("-oJ");
        args.push("-");
        println!("args: {:?}", args);

        let output = if self.sudo {
            match Command::new("sudo")
                .arg(self.system_path.as_str())
                .args(args)
                .output()
            {
                Ok(output) => output,
                Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error>),
            }
        } else {
            match Command::new(self.system_path.as_str()).args(args).output() {
                Ok(output) => output,
                Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error>),
            }
        };

        let result = match str::from_utf8(&output.stdout) {
            Ok(result) => result,
            Err(e) => {
                error!("e1: {:?}", e);
                return Err(Box::new(e) as Box<dyn std::error::Error>);
            }
        };
        let v: Value = match serde_json::from_str(result) {
            Ok(v) => v,
            Err(e) => {
                error!("e2: {:?}", e);
                return Err(Box::new(e) as Box<dyn std::error::Error>);
            }
        };
        let mut ps: Vec<Info> = Vec::new();
        let item_array = match v.as_array() {
            Some(v) => v,
            None => return Ok(ps),
        };
        for item in item_array.iter() {
            let p: Info = serde_json::from_value(item.clone()).unwrap();
            ps.push(p);
        }
        Ok(ps)
    }
}
