CREATE TYPE "SocialAccountEnum" as ENUM ('INACTIVE', 'ACTIVE');


CREATE TABLE public.social_account (
	social_name text NOT NULL,
	social_id text NOT NULL,
	status "SocialAccountEnum" NOT NULL DEFAULT 'ACTIVE',
	created_at timestamp DEFAULT now() NOT NULL,
	updated_at timestamp DEFAULT now() NOT NULL,
	published_at timestamp DEFAULT now() NOT NULL,
	username text NULL,
	name text NULL,
	avatar_url text NULL,
	followers_count int NULL,
	followings_count int NULL,
	statuses_count int NULL,
	biography text NULL,
	link text NULL,
	CONSTRAINT social_account_pkey PRIMARY KEY (social_name, social_id)
);
