-- Create notification table
CREATE TABLE IF NOT EXISTS notification (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    user_id BIGINT NOT NULL,
    name TEXT NOT NULL,
    config JSON NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);

-- Create monitor_notification table for many-to-many relationship
CREATE TABLE IF NOT EXISTS monitor_notification (
    monitor_id BIGINT NOT NULL,
    notification_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (monitor_id, notification_id),
    FOREIGN KEY (monitor_id) REFERENCES monitor(id) ON DELETE CASCADE,
    FOREIGN KEY (notification_id) REFERENCES notification(id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_notification_user_id ON notification(user_id);
CREATE INDEX IF NOT EXISTS idx_monitor_notification_monitor_id ON monitor_notification(monitor_id);
CREATE INDEX IF NOT EXISTS idx_monitor_notification_notification_id ON monitor_notification(notification_id);
