-- Load extentions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Table: public.users

-- DROP TABLE IF EXISTS public.users;

CREATE TABLE IF NOT EXISTS public.users
(
    "user_id" uuid NOT NULL,
    username text COLLATE pg_catalog."default" NOT NULL,
    owned integer NOT NULL DEFAULT 0,
    cash integer NOT NULL DEFAULT 0,
    password text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT userid PRIMARY KEY ("user_id"),
    CONSTRAINT username UNIQUE (username)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.users
    OWNER to postgres;


-- Table: public.transactions

-- DROP TABLE IF EXISTS public.transactions;

CREATE TABLE IF NOT EXISTS public.transactions
(
    transaction_id uuid NOT NULL,
    price money NOT NULL,
    quantity integer NOT NULL,
    buyer_id uuid NOT NULL,
    seller_id uuid NOT NULL,
    "time" timestamp with time zone NOT NULL,
    CONSTRAINT transactions_pkey PRIMARY KEY (transaction_id),
    CONSTRAINT transactions_buyer_id_fkey FOREIGN KEY (buyer_id)
        REFERENCES public.users (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT transactions_seller_id_fkey FOREIGN KEY (seller_id)
        REFERENCES public.users (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT quantity_positive CHECK (quantity > 0) NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.transactions
    OWNER to postgres;


-- Table: public.orders

-- DROP TABLE IF EXISTS public.orders;

CREATE TABLE IF NOT EXISTS public.orders
(
    order_id uuid NOT NULL,
    order_type text COLLATE pg_catalog."default" NOT NULL,
    price money NOT NULL,
    quantity integer NOT NULL,
    user_id uuid NOT NULL,
    status text COLLATE pg_catalog."default" NOT NULL,
    "time" timestamp with time zone NOT NULL,
    CONSTRAINT orders_pkey PRIMARY KEY (order_id),
    CONSTRAINT orders_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES public.users (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT valid_type CHECK (order_type = ANY (ARRAY['BUY'::text, 'SELL'::text])),
    CONSTRAINT valid_status CHECK (status = ANY (ARRAY['PENDING'::text, 'FULFILLED'::text, 'ERROR'::text]))
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.orders
    OWNER to postgres;