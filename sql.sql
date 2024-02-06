CREATE TABLE service_info (
    id VARCHAR(255) PRIMARY KEY,
    service_name VARCHAR(255) NOT NULL,
    service_type VARCHAR(255) NOT NULL,
    update_time BIGINT NOT NULL
    -- Uncomment the line below if you want to include Adapters column
    -- adapters JSONB[]
);

CREATE TABLE service_adapter (
    id VARCHAR(255) PRIMARY KEY,
    address VARCHAR(255) NOT NULL,
     protocol_used int NOT NULL,
	 protocol_description VARCHAR(255) NOT NULL,
	 service_info_id VARCHAR(255) NOT NULL
    -- Uncomment the line below if you want to include Adapters column
    -- adapters JSONB[]
);
