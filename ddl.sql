create table logline (id        serial PRIMARY KEY,
		      server    varchar(50),
		      datetime  varchar(50),
		      loglevel  varchar(50),
		      message   text
		);

CREATE INDEX logline_msg_idx ON logline USING gin(to_tsvector('english', message));
