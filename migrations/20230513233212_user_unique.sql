ALTER TABLE users ADD CONSTRAINT UQ_Person UNIQUE KEY(ip_address, user_agent);
