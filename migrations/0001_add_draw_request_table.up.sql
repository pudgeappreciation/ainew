CREATE TABLE `draw_requests` (
    `state` TEXT NOT NULL,
    `prompt` TEXT NOT NULL,
    `negative_prompt` TEXT NOT NULL,
    `steps` INTEGER NOT NULL,
    `user_id` INTEGER NOT NULL,
    `request_id` INTEGER NOT NULL,
    `channel_id` INTEGER NOT NULL,
    `created_at` INTEGER NOT NULL
)
