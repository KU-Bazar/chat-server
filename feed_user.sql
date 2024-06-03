\c kubazar;

-- dropping table if already exists
DROP TABLE IF EXISTS chat_user;

CREATE TABLE chat_user (
id uuid primary key,
username varchar(50) not null unique,
fullname varchar(50) not null,
avatar_url varchar(255)
);

-- seeding some random inputs
INSERT INTO chat_user (id, username, fullname, avatar_url)
VALUES
 ('d9b2d63d-a233-4123-847a-7ac9b47c4f44', 'john_doe', 'John Doe', 'https://images.unsplash.com/photo-1547721064-da6cfb341d50'),
 ('5a7b2b9a-5f69-4d7f-9e2d-8e3e7bbdf71b', 'jane_smith', 'Jane Smith', 'https://images.unsplash.com/photo-1524504388940-b1c1722653e1'),
 ('3e0b67f5-527d-4e2b-b8eb-6e913d0c1b5e', 'alex_jones', 'Alex Jones', 'https://images.unsplash.com/photo-1516117172878-fd2c41f4a759'),
 ('7f9c1c2c-9e4a-4e87-bd7c-d5e4c9c32c3e', 'emily_clark', 'Emily Clark', 'https://images.unsplash.com/photo-1494790108377-be9c29b29330'),
 ('2f8b8c29-8052-42b6-b93c-5d759e2c4e4a', 'michael_brown', 'Michael Brown', 'https://images.unsplash.com/photo-1502767089025-6572583495b6'),
 ('d892f747-69e4-4c76-bd3b-f5c3b1d2e5c2', 'sarah_lee', 'Sarah Lee', 'https://images.unsplash.com/photo-1529626455594-4ff0802cfb7e'),
 ('aad52b6b-d3e1-4c73-9e3c-b3a8c4d2b5c7', 'david_kim', 'David Kim', 'https://images.unsplash.com/photo-1535713875002-d1d0cf377fde'),
 ('4e3b5e7a-93e1-4f8b-9c1c-5b6d7e8a2d4f', 'laura_smith', 'Laura Smith', 'https://images.unsplash.com/photo-1492562080023-ab3db95bfbce');

SELECT * from chat_user;
