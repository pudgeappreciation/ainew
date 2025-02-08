CREATE TABLE `draw_requests` (
    `state` TEXT NOT NULL,
    `options` TEXT NOT NULL,
    `user_id` INTEGER NOT NULL,
    `request_id` INTEGER NOT NULL,
    `channel_id` INTEGER NOT NULL,
    `created_at` INTEGER NOT NULL
)
