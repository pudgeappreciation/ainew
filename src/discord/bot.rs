use std::sync::atomic::{AtomicBool, Ordering};

use serenity::all::{Context, EventHandler};
use serenity::async_trait;
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;

use sqlx::{Pool, Sqlite};

use crate::global::channels::respond_to_message::RespondToMessageReceiver;
use crate::global::channels::wake_draw_task::WakeDrawTask;
use crate::global::models::base_model::Models;
use crate::global::models::lora::Loras;

use super::commands;
use super::respond_to_message_task;

pub struct Bot {
    pub database: Pool<Sqlite>,
    pub draw_task: WakeDrawTask,
    pub models: Models,
    pub loras: Loras,
    response_receiver: RespondToMessageReceiver,
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "draw" => commands::draw::handle(self, ctx, command).await,
                "loras" => commands::loras::handle(self, ctx, command).await,
                "models" => commands::models::handle(self, ctx, command).await,
                _ => println!("Command not registered"),
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        commands::draw::register(&ctx).await;
        commands::loras::register(&ctx).await;
        commands::models::register(&ctx).await;
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
    ) -> Bot {
        Bot {
            loras,
            models,
            database,
            response_receiver,
            draw_task: wake_draw_task,
            is_loop_running: AtomicBool::new(false),
        }
    }
}
