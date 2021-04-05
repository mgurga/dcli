use serenity::{
    async_trait,
    model::gateway::Ready,
    prelude::*,
    http::GuildPagination
};

pub struct ListserverHandler;

#[async_trait]
impl EventHandler for ListserverHandler {
    async fn ready(&self, ctx: Context, ready: Ready){
        println!("{} is connected!", ready.user.name);
        let guildid = ready.guilds[0].id();
        let guildp: GuildPagination = GuildPagination::Before(guildid);
        match ctx.http.get_guilds(&guildp, 100).await {
            Ok(guilds) => {
                println!("got server list");
                for guild in guilds {
                    println!("{}", guild.name)
                }
                return;
            },
            Err(why) => {
                println!("Error getting servers: {}", why);
                return;
            }
        };
    }
}
