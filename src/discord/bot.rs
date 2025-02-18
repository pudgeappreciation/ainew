use std::sync::atomic::{AtomicBool, Ordering};

use serenity::all::{Command, Context, EventHandler};
use serenity::async_trait;
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;

use sqlx::{Pool, Sqlite};

use crate::global::channels::respond_to_message::RespondToMessageReceiver;
use crate::global::channels::wake_draw_task::WakeDrawTask;
use crate::global::generation_options::lora::Loras;
use crate::global::generation_options::model::Models;
use crate::global::generation_options::sampler::Samplers;
use crate::global::generation_options::scheduler::Schedulers;

use super::commands;
use super::respond_to_message_task;

pub struct Bot {
    pub database: Pool<Sqlite>,
    pub draw_task: WakeDrawTask,
    pub models: Models,
    pub loras: Loras,
    pub samplers: Samplers,
    pub schedulers: Schedulers,
    response_receiver: RespondToMessageReceiver,
    is_loop_running: AtomicBool,
}

impl Bot {
    async fn register_commands(&self, ctx: &Context, guild_id: GuildId) {
        let result = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::draw::create(),
                    commands::loras::create(),
                    commands::models::create(),
                    commands::draw_profile::create(),
                ],
            )
            .await;

        match result {
            Ok(_) => println!("Registered commands in server \"{}\"!", guild_id),
            Err(why) => println!(
                "Error registering commands in server \"{}\"! {}",
                guild_id, why,
            ),
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "draw" => commands::draw::handle(self, ctx, command).await,
                "loras" => commands::loras::handle(self, ctx, command).await,
                "models" => commands::models::handle(self, ctx, command).await,
                "profile" => commands::draw_profile::handle(self, ctx, command).await,
                _ => println!("Command not registered"),
            };
        } else if let Interaction::Autocomplete(autocomplete) = interaction {
            match autocomplete.data.name.as_str() {
                "draw" => commands::draw::autocomplete(self, ctx, autocomplete).await,
                "profile" => commands::draw_profile::autocomplete(self, ctx, autocomplete).await,
                _ => println!("Autocomplete not registered"),
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        if let Ok(guild_ids) = dotenvy::var("GUILD_IDS") {
            for guild_id in guild_ids
                .split(",")
                .filter_map(|guild_id| guild_id.parse().ok())
                .map(|guild_id| GuildId::new(guild_id))
            {
                println!("Registering commands in server \"{}\"!", guild_id);
                self.register_commands(&ctx, guild_id).await;
            }
        }

        if dotenvy::var("APP_ENV").unwrap_or("dev".to_string()) == "production" {
            let guild_commands = Command::set_global_commands(
                &ctx.http,
                vec![
                    commands::draw::create(),
                    commands::loras::create(),
                    commands::models::create(),
                    commands::draw_profile::create(),
                ],
            )
            .await;

            println!("I created the following global slash command: {guild_commands:#?}");
        }

        self.draw_task.wake();
    }

    // We use the cache_ready event just in case some cache operation is required in whatever use
    // case you have for this.
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        if !self.is_loop_running.load(Ordering::Relaxed) {
            respond_to_message_task::start(ctx.clone(), self.response_receiver.clone());

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

impl Bot {
    pub fn new(
        database: Pool<Sqlite>,
        wake_draw_task: WakeDrawTask,
        response_receiver: RespondToMessageReceiver,
        models: Models,
        loras: Loras,
        samplers: Samplers,
        schedulers: Schedulers,
    ) -> Bot {
        Bot {
            loras,
            models,
            samplers,
            schedulers,
            database,
            response_receiver,
            draw_task: wake_draw_task,
            is_loop_running: AtomicBool::new(false),
        }
    }
}
