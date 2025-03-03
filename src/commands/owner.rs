use crate::global::*;
use crate::ShardManagerContainer;
use log::error;
use serenity::framework::standard::{macros::command, CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::thread;
use std::time::Duration;

#[command]
#[owners_only]
#[aliases("stop")]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let _ = msg.channel_id.say(&ctx.http, "Shutting down!");

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        manager.lock().shutdown_all();
    } else {
        let _ = msg.reply(&ctx, "There was a problem getting the shard manager");

        return Ok(());
    }

    Ok(())
}

#[command]
#[owners_only]
fn gulag(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let list = msg.content.split_whitespace();
    let time = list.last().unwrap().parse::<u64>().unwrap();
    let _user = &msg.mentions[0];

    let gulag_role: u64 = {
        let global = data
            .get::<GlobalInformation>()
            .expect("Expected GlobalInformation in ShareMap");
        match global.get(&GlobalKeys::GulagRole) {
            Some(value) => value.get(0).unwrap().parse::<u64>().unwrap(),
            None => {
                error!("No Gulag Role");
                return Ok(());
            }
        }
    };

    let mut _member = match ctx.http.get_member(msg.guild_id.unwrap().0, _user.id.0) {
        Ok(_member) => _member,
        Err(_) => {
            error!("Failed to parse message information");
            return Err(CommandError(String::from("Couldn't parse user")));
        }
    };
    let _roles = _member.clone().roles;
    for i in _roles.clone() {
        match _member.remove_role(&ctx.http, i) {
            Ok(()) => (),
            Err(_) => {
                error!("Couldn't remove role {}", i);
                return Err(CommandError(String::from("Couldn't parse stuff")));
            }
        }
    }

    if let Err(_) = _member.add_role(&ctx.http, gulag_role) {
        error!("Couldn't give gulag role");
        return Err(CommandError(String::from("Couldn't parse stuff")));
    };

    if let Err(_) = _member.user.read().direct_message(&ctx, |m| {
        m.content(format!("You will be released in {} minute(s)", time))
    }) {
        error!("Couldn't send info dm");
    };

    thread::sleep(Duration::from_secs(time * 60));

    for i in _roles.clone() {
        match _member.add_role(&ctx.http, i) {
            Ok(()) => (),
            Err(_) => {
                error!("Couldn't give role {}", i);
                return Err(CommandError(String::from("Couldn't parse stuff")));
            }
        }
    }

    match _member.remove_role(&ctx.http, gulag_role) {
        Ok(()) => (),
        Err(_) => {
            error!("Could not remove gulag role");
            return Err(CommandError(String::from("Couldn't remove gulag role")));
        }
    }
    Ok(())
}
