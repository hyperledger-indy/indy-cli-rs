/*
    Copyright © 2023 Province of British Columbia
    https://digital.gov.bc.ca/digital-trust
*/
use crate::command_executor::{
    Command, CommandContext, CommandMetadata, CommandParams, CommandResult,
};

pub mod about_command {
    use super::*;

    command!(CommandMetadata::build("about", "Show about information").finalize());

    fn execute(_ctx: &CommandContext, _params: &CommandParams) -> CommandResult {
        trace!("execute >> _ctx: params: {:?}", _params);

        println_succ!(
            "Hyperledger Indy CLI (https://github.com/hyperledger-indy/indy-cli-rs.git)"
        );
        println!();
        println_succ!(
            "This is CLI tool for Hyperledger Indy (https://www.hyperledger.org/projects),"
        );
        println_succ!("which provides a distributed-ledger-based foundation for");
        println_succ!("self-sovereign identity.");
        println!();
        println_succ!("Version: {}", env!("CARGO_PKG_VERSION"));
        println_succ!("Apache License Version 2.0");
        println_succ!("Copyright © 2023 Province of British Columbia");
        println!();

        let res = Ok(());

        trace!("execute << {:?}", res);
        res
    }
}
