// Copyright 2020 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::borrow::Borrow;
use std::path::Path;
use std::process::Stdio;
use std::str::FromStr;

use async_trait::async_trait;
use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::ArgMatches;
use nix::NixPath;
use serde::Deserialize;
use serde::Serialize;

use crate::cmds::clusters::cluster::ClusterProfile;
use crate::cmds::command::Command;
use crate::cmds::Config;
use crate::cmds::Status;
use crate::cmds::Writer;
use crate::error::CliError;
use crate::error::Result;
use dyn_clone::DynClone;
use sha2::digest::Update;

#[derive(Clone)]
pub struct UpCommand {
    #[allow(dead_code)]
    conf: Config,
    clap: App<'static>,
}

impl UpCommand {
    pub fn create(conf: Config) -> Self {
        let clap = UpCommand::generate();
        UpCommand { conf, clap }
    }
    pub fn generate() -> App<'static> {
        let app = App::new("up")
            .setting(AppSettings::DisableVersionFlag)
            .about("Build a cluster for demo")
            .arg(
                Arg::new("profile")
                    .long("profile")
                    .about("Profile to run queries")
                    .required(false)
                    .possible_values(&["local"])
                    .default_value("local"),
            ).arg(
            Arg::new("dataset")
                .long("dataset")
                .about("Profile to run queries")
                .required(false)
                .possible_values(&["local"])
                .default_value("local"),
        );
        app
    }

    pub(crate) fn exec_match(&self, writer: &mut Writer, args: Option<&ArgMatches>) -> Result<()> {
        match args {
            Some(matches) => {
                let profile = matches.value_of_t("profile");
                match profile {
                    Ok(ClusterProfile::Local) => {
                        return self.local_exec_match(writer, matches);
                    }
                    Ok(ClusterProfile::Cluster) => {
                        todo!()
                    }
                    Err(_) => writer.write_err("currently profile only support cluster or local"),
                }
            }
            None => {
                println!("none ");
            }
        }
        Ok(())
    }

    fn local_exec_match(&self, writer: &mut Writer, args: &ArgMatches) -> Result<()> {
      Ok(())
    }

    /// precheck whether current local profile applicable for local host machine
    fn local_exec_precheck(&self, _args: &ArgMatches) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl Command for UpCommand {
    fn name(&self) -> &str {
        "up"
    }

    fn about(&self) -> &str {
        "Query on databend cluster"
    }

    fn is(&self, s: &str) -> bool {
        s.contains(self.name())
    }

    async fn exec(&self, writer: &mut Writer, args: String) -> Result<()> {
        let words = shellwords::split(args.as_str());
        if words.is_err() {
            writer.write_err("cannot parse words");
            return Ok(());
        }
        match self.clap.clone().try_get_matches_from(words.unwrap()) {
            Ok(matches) => {
                return self.exec_match(writer, Some(matches.borrow()));
            }
            Err(err) => {
                println!("Cannot get subcommand matches: {}", err);
            }
        }

        Ok(())
    }
}
