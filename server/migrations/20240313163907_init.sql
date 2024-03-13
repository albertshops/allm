CREATE TABLE chats (
  id uuid NOT NULL PRIMARY KEY,
  model TEXT NOT NULL
);

CREATE TABLE messages (
  id uuid NOT NULL PRIMARY KEY,
  chat_id uuid NOT NULL,
  timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  role TEXT NOT NULL,
  content TEXT NOT NULL
);
