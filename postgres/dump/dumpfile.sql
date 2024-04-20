--
-- PostgreSQL database dump
--

-- Dumped from database version 16.2
-- Dumped by pg_dump version 16.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: achievement; Type: TABLE; Schema: public; Owner: shooteradmin
--

CREATE TABLE public.achievement (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    image text
);


ALTER TABLE public.achievement OWNER TO shooteradmin;

--
-- Name: client; Type: TABLE; Schema: public; Owner: shooteradmin
--

CREATE TABLE public.client (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    username text NOT NULL,
    email text NOT NULL,
    password text NOT NULL,
    salt text NOT NULL,
    role_id integer DEFAULT 1 NOT NULL,
    rank_id integer DEFAULT 1 NOT NULL
);


ALTER TABLE public.client OWNER TO shooteradmin;

--
-- Name: client_achievement; Type: TABLE; Schema: public; Owner: shooteradmin
--

CREATE TABLE public.client_achievement (
    client_id uuid NOT NULL,
    achievement_id uuid NOT NULL
);


ALTER TABLE public.client_achievement OWNER TO shooteradmin;

--
-- Name: friend; Type: TABLE; Schema: public; Owner: shooteradmin
--

CREATE TABLE public.friend (
    client1_id uuid NOT NULL,
    client2_id uuid NOT NULL
);


ALTER TABLE public.friend OWNER TO shooteradmin;

--
-- Name: rank; Type: TABLE; Schema: public; Owner: shooteradmin
--

CREATE TABLE public.rank (
    id integer NOT NULL,
    rank_name text NOT NULL
);


ALTER TABLE public.rank OWNER TO shooteradmin;

--
-- Name: rank_id_seq; Type: SEQUENCE; Schema: public; Owner: shooteradmin
--

CREATE SEQUENCE public.rank_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.rank_id_seq OWNER TO shooteradmin;

--
-- Name: rank_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: shooteradmin
--

ALTER SEQUENCE public.rank_id_seq OWNED BY public.rank.id;


--
-- Name: role; Type: TABLE; Schema: public; Owner: shooteradmin
--

CREATE TABLE public.role (
    role_id integer NOT NULL,
    role_name character varying(50) NOT NULL
);


ALTER TABLE public.role OWNER TO shooteradmin;

--
-- Name: role_role_id_seq; Type: SEQUENCE; Schema: public; Owner: shooteradmin
--

CREATE SEQUENCE public.role_role_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.role_role_id_seq OWNER TO shooteradmin;

--
-- Name: role_role_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: shooteradmin
--

ALTER SEQUENCE public.role_role_id_seq OWNED BY public.role.role_id;


--
-- Name: rank id; Type: DEFAULT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.rank ALTER COLUMN id SET DEFAULT nextval('public.rank_id_seq'::regclass);


--
-- Name: role role_id; Type: DEFAULT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.role ALTER COLUMN role_id SET DEFAULT nextval('public.role_role_id_seq'::regclass);


--
-- Data for Name: achievement; Type: TABLE DATA; Schema: public; Owner: shooteradmin
--

COPY public.achievement (id, name, description, image) FROM stdin;
\.


--
-- Data for Name: client; Type: TABLE DATA; Schema: public; Owner: shooteradmin
--

COPY public.client (id, username, email, password, salt, role_id, rank_id) FROM stdin;
\.


--
-- Data for Name: client_achievement; Type: TABLE DATA; Schema: public; Owner: shooteradmin
--

COPY public.client_achievement (client_id, achievement_id) FROM stdin;
\.


--
-- Data for Name: friend; Type: TABLE DATA; Schema: public; Owner: shooteradmin
--

COPY public.friend (client1_id, client2_id) FROM stdin;
\.


--
-- Data for Name: rank; Type: TABLE DATA; Schema: public; Owner: shooteradmin
--

COPY public.rank (id, rank_name) FROM stdin;
1	Bronze
2	Argent
3	Or
4	Platinum
5	Diamond
\.


--
-- Data for Name: role; Type: TABLE DATA; Schema: public; Owner: shooteradmin
--

COPY public.role (role_id, role_name) FROM stdin;
1	Player
2	DGS
\.


--
-- Name: rank_id_seq; Type: SEQUENCE SET; Schema: public; Owner: shooteradmin
--

SELECT pg_catalog.setval('public.rank_id_seq', 5, true);


--
-- Name: role_role_id_seq; Type: SEQUENCE SET; Schema: public; Owner: shooteradmin
--

SELECT pg_catalog.setval('public.role_role_id_seq', 2, true);


--
-- Name: achievement achievement_pkey; Type: CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.achievement
    ADD CONSTRAINT achievement_pkey PRIMARY KEY (id);


--
-- Name: client_achievement client_achievement_pkey; Type: CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.client_achievement
    ADD CONSTRAINT client_achievement_pkey PRIMARY KEY (client_id, achievement_id);


--
-- Name: client client_email_key; Type: CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.client
    ADD CONSTRAINT client_email_key UNIQUE (email);


--
-- Name: client client_pkey; Type: CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.client
    ADD CONSTRAINT client_pkey PRIMARY KEY (id);


--
-- Name: client client_username_key; Type: CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.client
    ADD CONSTRAINT client_username_key UNIQUE (username);


--
-- Name: friend friend_pkey; Type: CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.friend
    ADD CONSTRAINT friend_pkey PRIMARY KEY (client1_id, client2_id);


--
-- Name: achievement name_unique; Type: CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.achievement
    ADD CONSTRAINT name_unique UNIQUE (name);


--
-- Name: rank rank_pkey; Type: CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.rank
    ADD CONSTRAINT rank_pkey PRIMARY KEY (id);


--
-- Name: rank rank_rank_name_key; Type: CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.rank
    ADD CONSTRAINT rank_rank_name_key UNIQUE (rank_name);


--
-- Name: role role_pkey; Type: CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.role
    ADD CONSTRAINT role_pkey PRIMARY KEY (role_id);


--
-- Name: client_achievement client_achievement_achievement_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.client_achievement
    ADD CONSTRAINT client_achievement_achievement_id_fkey FOREIGN KEY (achievement_id) REFERENCES public.achievement(id);


--
-- Name: client_achievement client_achievement_client_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.client_achievement
    ADD CONSTRAINT client_achievement_client_id_fkey FOREIGN KEY (client_id) REFERENCES public.client(id);


--
-- Name: client client_role_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.client
    ADD CONSTRAINT client_role_id_fkey FOREIGN KEY (role_id) REFERENCES public.role(role_id);


--
-- Name: client fk_rank_id; Type: FK CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.client
    ADD CONSTRAINT fk_rank_id FOREIGN KEY (rank_id) REFERENCES public.rank(id);


--
-- Name: friend friend_client1_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.friend
    ADD CONSTRAINT friend_client1_id_fkey FOREIGN KEY (client1_id) REFERENCES public.client(id);


--
-- Name: friend friend_client2_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: shooteradmin
--

ALTER TABLE ONLY public.friend
    ADD CONSTRAINT friend_client2_id_fkey FOREIGN KEY (client2_id) REFERENCES public.client(id);


--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: pg_database_owner
--

GRANT ALL ON SCHEMA public TO shooteradmin;


--
-- PostgreSQL database dump complete
--

