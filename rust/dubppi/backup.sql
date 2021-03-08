--
-- PostgreSQL database dump
--

-- Dumped from database version 12.4 (Debian 12.4-3)
-- Dumped by pg_dump version 12.4 (Debian 12.4-3)

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
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: y0h4n3s
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO y0h4n3s;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: y0h4n3s
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO y0h4n3s;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: y0h4n3s
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO y0h4n3s;

--
-- Name: dump_collector; Type: TABLE; Schema: public; Owner: y0h4n3s
--

CREATE TABLE public.dump_collector (
    id integer NOT NULL,
    hostname text DEFAULT ''::text,
    full_path text DEFAULT ''::text,
    protocol text DEFAULT ''::text,
    path_only text DEFAULT ''::text,
    full_params text DEFAULT ''::text,
    href text DEFAULT ''::text,
    path_href text DEFAULT ''::text,
    link_from text DEFAULT ''::text,
    ip text DEFAULT ''::text,
    port integer DEFAULT 0,
    endpoint_id text NOT NULL
);


ALTER TABLE public.dump_collector OWNER TO y0h4n3s;

--
-- Name: dump_collector_id_seq; Type: SEQUENCE; Schema: public; Owner: y0h4n3s
--

CREATE SEQUENCE public.dump_collector_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.dump_collector_id_seq OWNER TO y0h4n3s;

--
-- Name: dump_collector_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: y0h4n3s
--

ALTER SEQUENCE public.dump_collector_id_seq OWNED BY public.dump_collector.id;


--
-- Name: dump_collector id; Type: DEFAULT; Schema: public; Owner: y0h4n3s
--

ALTER TABLE ONLY public.dump_collector ALTER COLUMN id SET DEFAULT nextval('public.dump_collector_id_seq'::regclass);


--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: y0h4n3s
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2021-01-03 19:47:32.910454
20201224174916	2021-01-03 19:47:33.007152
\.


--
-- Data for Name: dump_collector; Type: TABLE DATA; Schema: public; Owner: y0h4n3s
--

COPY public.dump_collector (id, hostname, full_path, protocol, path_only, full_params, href, path_href, link_from, ip, port, endpoint_id) FROM stdin;
1	www.xiaomi.com		http	/favicon.ico		"http://www.xiaomi.com/favicon.ico"	/favicon.ico	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
2	app.xiaomi.com		http			"http://app.xiaomi.com"		http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
3	file.market.xiaomi.com		http	/download/ThemeMarket/006fa5b89c49209271b15afbc06410fb1da436900		"http://file.market.xiaomi.com/download/ThemeMarket/006fa5b89c49209271b15afbc06410fb1da436900"	/download/ThemeMarket/006fa5b89c49209271b15afbc06410fb1da436900	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
4	file.market.xiaomi.com		http	/download/ThemeMarket/0e878b4dff45441852650260355d459e13110c413		"http://file.market.xiaomi.com/download/ThemeMarket/0e878b4dff45441852650260355d459e13110c413"	/download/ThemeMarket/0e878b4dff45441852650260355d459e13110c413	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
5	file.market.xiaomi.com		http	/download/ThemeMarket/0b14a51f4b4d5500d4af8a0d6a0881e7c2d417c0d		"http://file.market.xiaomi.com/download/ThemeMarket/0b14a51f4b4d5500d4af8a0d6a0881e7c2d417c0d"	/download/ThemeMarket/0b14a51f4b4d5500d4af8a0d6a0881e7c2d417c0d	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
6	file.market.xiaomi.com		http	/download/ThemeMarket/0b14a51f484d5a00d5af800d600887e7cdd417c0d		"http://file.market.xiaomi.com/download/ThemeMarket/0b14a51f484d5a00d5af800d600887e7cdd417c0d"	/download/ThemeMarket/0b14a51f484d5a00d5af800d600887e7cdd417c0d	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
7	file.market.xiaomi.com		http	/download/ThemeMarket/0fc3c44ea836b41be0826af2d516e8abd42597f69		"http://file.market.xiaomi.com/download/ThemeMarket/0fc3c44ea836b41be0826af2d516e8abd42597f69"	/download/ThemeMarket/0fc3c44ea836b41be0826af2d516e8abd42597f69	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
8	file.market.xiaomi.com		http	/download/ThemeMarket/07e5751f0467c6108c711ca5781ac573dff420213		"http://file.market.xiaomi.com/download/ThemeMarket/07e5751f0467c6108c711ca5781ac573dff420213"	/download/ThemeMarket/07e5751f0467c6108c711ca5781ac573dff420213	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
9	file.market.xiaomi.com		http	/download/ThemeMarket/0536b94b8bdd448380fbb64c60444666533eef3b1		"http://file.market.xiaomi.com/download/ThemeMarket/0536b94b8bdd448380fbb64c60444666533eef3b1"	/download/ThemeMarket/0536b94b8bdd448380fbb64c60444666533eef3b1	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
10	file.market.xiaomi.com		http	/download/ThemeMarket/06b7d46a719325f8fb85f63711970b9757041b103		"http://file.market.xiaomi.com/download/ThemeMarket/06b7d46a719325f8fb85f63711970b9757041b103"	/download/ThemeMarket/06b7d46a719325f8fb85f63711970b9757041b103	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
11	t5.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/00530240b3a884a970f4a8d820bf80ee86227a5fa		"http://t5.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/00530240b3a884a970f4a8d820bf80ee86227a5fa"	/thumbnail/jpeg/w118/ThemeMarket/00530240b3a884a970f4a8d820bf80ee86227a5fa	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
12	t2.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/025b8e44adc9a4efd3531f366bb343dbc0c70f647		"http://t2.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/025b8e44adc9a4efd3531f366bb343dbc0c70f647"	/thumbnail/jpeg/w118/ThemeMarket/025b8e44adc9a4efd3531f366bb343dbc0c70f647	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
13	t5.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/00b845994f9434d42c9d00daf07006581ec43677c		"http://t5.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/00b845994f9434d42c9d00daf07006581ec43677c"	/thumbnail/jpeg/w118/ThemeMarket/00b845994f9434d42c9d00daf07006581ec43677c	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
14	t1.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/00530240b3a884a970f4a8d820bf80ee86227a5fa		"http://t1.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/00530240b3a884a970f4a8d820bf80ee86227a5fa"	/thumbnail/jpeg/w118/ThemeMarket/00530240b3a884a970f4a8d820bf80ee86227a5fa	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
15	t4.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/025b8e44adc9a4efd3531f366bb343dbc0c70f647		"http://t4.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/025b8e44adc9a4efd3531f366bb343dbc0c70f647"	/thumbnail/jpeg/w118/ThemeMarket/025b8e44adc9a4efd3531f366bb343dbc0c70f647	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
16	t4.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/0cd9795abdafb425f2dfa85bf6ff3eef0106d0465		"http://t4.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/0cd9795abdafb425f2dfa85bf6ff3eef0106d0465"	/thumbnail/jpeg/w118/ThemeMarket/0cd9795abdafb425f2dfa85bf6ff3eef0106d0465	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
17	t5.market.xiaomi.com		http	/thumbnail/jpeg/w112/ThemeMarket/0f7e3402fcbc2a8c24185c7e88d65bc1e9d43cd60		"http://t5.market.xiaomi.com/thumbnail/jpeg/w112/ThemeMarket/0f7e3402fcbc2a8c24185c7e88d65bc1e9d43cd60"	/thumbnail/jpeg/w112/ThemeMarket/0f7e3402fcbc2a8c24185c7e88d65bc1e9d43cd60	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
18	t2.market.xiaomi.com		http	/thumbnail/jpeg/w112/ThemeMarket/0f7e3402ffbc258c2918547e83d658c1e0d43cd60		"http://t2.market.xiaomi.com/thumbnail/jpeg/w112/ThemeMarket/0f7e3402ffbc258c2918547e83d658c1e0d43cd60"	/thumbnail/jpeg/w112/ThemeMarket/0f7e3402ffbc258c2918547e83d658c1e0d43cd60	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
19	t5.market.xiaomi.com		http	/thumbnail/jpeg/w112/ThemeMarket/0417e512f4a35cfe94448334cafab4d515e42a674		"http://t5.market.xiaomi.com/thumbnail/jpeg/w112/ThemeMarket/0417e512f4a35cfe94448334cafab4d515e42a674"	/thumbnail/jpeg/w112/ThemeMarket/0417e512f4a35cfe94448334cafab4d515e42a674	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
20	t5.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/000f435a74abc4d872dfca30f85b12c534af0bd62		"http://t5.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/000f435a74abc4d872dfca30f85b12c534af0bd62"	/thumbnail/jpeg/w118/ThemeMarket/000f435a74abc4d872dfca30f85b12c534af0bd62	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
21	t4.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/0bfa84118e897d538d9154cb0e336f250c140f4d7		"http://t4.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/0bfa84118e897d538d9154cb0e336f250c140f4d7"	/thumbnail/jpeg/w118/ThemeMarket/0bfa84118e897d538d9154cb0e336f250c140f4d7	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
22	t5.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/008a245aeec7719aec92677a2f439fcff4c42b366		"http://t5.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/008a245aeec7719aec92677a2f439fcff4c42b366"	/thumbnail/jpeg/w118/ThemeMarket/008a245aeec7719aec92677a2f439fcff4c42b366	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
23	www.xiaomi.com		http			"http://www.xiaomi.com"		http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
24	zhuti.designer.xiaomi.com		https			"https://zhuti.designer.xiaomi.com"		http://zhuti.xiaomi.com/		443	L2RvbS9tdWNoX2RhdGE=
25	www.xiaomi.com		http	/favicon.ico		http://www.xiaomi.com/favicon.ico	/favicon.ico	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
26	zhuti.xiaomi.com		http	/		http://zhuti.xiaomi.com/	/	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
27	app.xiaomi.com		http	/		http://app.xiaomi.com/	/	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
28	zhuti.xiaomi.com		http	/login	?referer=http%3A%2F%2Fzhuti.xiaomi.com	http://zhuti.xiaomi.com/login?referer=http%3A%2F%2Fzhuti.xiaomi.com	/login?referer=http%3A%2F%2Fzhuti.xiaomi.com	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
29	zhuti.xiaomi.com		http	/compound		http://zhuti.xiaomi.com/compound	/compound	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
30	zhuti.xiaomi.com		http	/lockstyle		http://zhuti.xiaomi.com/lockstyle	/lockstyle	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
31	zhuti.xiaomi.com		http	/ringtone		http://zhuti.xiaomi.com/ringtone	/ringtone	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
32	zhuti.xiaomi.com		http	/icon		http://zhuti.xiaomi.com/icon	/icon	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
33	zhuti.xiaomi.com		http	/font		http://zhuti.xiaomi.com/font	/font	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
34	zhuti.xiaomi.com		http	/	#	http://zhuti.xiaomi.com/#	/#	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
35	zhuti.xiaomi.com		http	/subject/d7502bed-0bea-4b7f-a084-79e64309345c		http://zhuti.xiaomi.com/subject/d7502bed-0bea-4b7f-a084-79e64309345c	/subject/d7502bed-0bea-4b7f-a084-79e64309345c	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
36	zhuti.xiaomi.com		http	/subject/d373df9f-dca5-4d71-95eb-76385c8ee235		http://zhuti.xiaomi.com/subject/d373df9f-dca5-4d71-95eb-76385c8ee235	/subject/d373df9f-dca5-4d71-95eb-76385c8ee235	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
37	zhuti.xiaomi.com		http	/subject/a4eeac01-46f0-4499-bfdb-9153a5c7af27		http://zhuti.xiaomi.com/subject/a4eeac01-46f0-4499-bfdb-9153a5c7af27	/subject/a4eeac01-46f0-4499-bfdb-9153a5c7af27	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
38	zhuti.xiaomi.com		http	/subject/34687e1a-f257-41ab-a7c0-ae7ff578b0b4		http://zhuti.xiaomi.com/subject/34687e1a-f257-41ab-a7c0-ae7ff578b0b4	/subject/34687e1a-f257-41ab-a7c0-ae7ff578b0b4	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
39	zhuti.xiaomi.com		http	/detail/7e627405-a9a0-49d2-94ab-9f7538d24811		http://zhuti.xiaomi.com/detail/7e627405-a9a0-49d2-94ab-9f7538d24811	/detail/7e627405-a9a0-49d2-94ab-9f7538d24811	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
40	zhuti.xiaomi.com		http	/detail/46a0e406-1fe7-4286-a8eb-72521bcd5671		http://zhuti.xiaomi.com/detail/46a0e406-1fe7-4286-a8eb-72521bcd5671	/detail/46a0e406-1fe7-4286-a8eb-72521bcd5671	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
41	zhuti.xiaomi.com		http	/detail/1f917337-9cf2-49fa-a3e3-c7a9a8113821		http://zhuti.xiaomi.com/detail/1f917337-9cf2-49fa-a3e3-c7a9a8113821	/detail/1f917337-9cf2-49fa-a3e3-c7a9a8113821	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
42	zhuti.xiaomi.com		http	/detail/1b6d595a-b8a4-440b-8930-b12cc765d6bf		http://zhuti.xiaomi.com/detail/1b6d595a-b8a4-440b-8930-b12cc765d6bf	/detail/1b6d595a-b8a4-440b-8930-b12cc765d6bf	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
43	zhuti.xiaomi.com		http	/detail/fa943822-2e35-41c6-ab55-f535400cf77e		http://zhuti.xiaomi.com/detail/fa943822-2e35-41c6-ab55-f535400cf77e	/detail/fa943822-2e35-41c6-ab55-f535400cf77e	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
44	zhuti.xiaomi.com		http	/detail/134bda4d-3538-47c6-94f5-292f1c8212b2		http://zhuti.xiaomi.com/detail/134bda4d-3538-47c6-94f5-292f1c8212b2	/detail/134bda4d-3538-47c6-94f5-292f1c8212b2	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
45	zhuti.xiaomi.com		http	/detail/dd7afb17-6708-4b4e-952f-21c95e5504a0		http://zhuti.xiaomi.com/detail/dd7afb17-6708-4b4e-952f-21c95e5504a0	/detail/dd7afb17-6708-4b4e-952f-21c95e5504a0	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
46	zhuti.xiaomi.com		http	/detail/8ce56953-6f48-4588-a16d-1010038c2805		http://zhuti.xiaomi.com/detail/8ce56953-6f48-4588-a16d-1010038c2805	/detail/8ce56953-6f48-4588-a16d-1010038c2805	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
47	zhuti.xiaomi.com		http	/detail/eee1aece-b41a-4348-8f0a-ac819262ecd4		http://zhuti.xiaomi.com/detail/eee1aece-b41a-4348-8f0a-ac819262ecd4	/detail/eee1aece-b41a-4348-8f0a-ac819262ecd4	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
48	zhuti.xiaomi.com		http	/detail/856a6858-0f62-49e5-9a6f-3a6596b1e446		http://zhuti.xiaomi.com/detail/856a6858-0f62-49e5-9a6f-3a6596b1e446	/detail/856a6858-0f62-49e5-9a6f-3a6596b1e446	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
49	zhuti.xiaomi.com		http	/detail/e5890e8a-68ab-487a-bf5e-e1ba6585f447		http://zhuti.xiaomi.com/detail/e5890e8a-68ab-487a-bf5e-e1ba6585f447	/detail/e5890e8a-68ab-487a-bf5e-e1ba6585f447	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
50	zhuti.xiaomi.com		http	/detail/7ee8fb20-1ffc-4040-8159-743162b3e17a		http://zhuti.xiaomi.com/detail/7ee8fb20-1ffc-4040-8159-743162b3e17a	/detail/7ee8fb20-1ffc-4040-8159-743162b3e17a	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
51	zhuti.xiaomi.com		http	/detail/196d29ea-933e-4617-98fd-002b7733720e		http://zhuti.xiaomi.com/detail/196d29ea-933e-4617-98fd-002b7733720e	/detail/196d29ea-933e-4617-98fd-002b7733720e	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
52	zhuti.xiaomi.com		http	/detail/c2acbfcb-bf17-45bf-86e3-0ebce27004ac		http://zhuti.xiaomi.com/detail/c2acbfcb-bf17-45bf-86e3-0ebce27004ac	/detail/c2acbfcb-bf17-45bf-86e3-0ebce27004ac	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
53	zhuti.xiaomi.com		http	/detail/5b184b97-8e18-408d-9ce8-0134d3b92d97		http://zhuti.xiaomi.com/detail/5b184b97-8e18-408d-9ce8-0134d3b92d97	/detail/5b184b97-8e18-408d-9ce8-0134d3b92d97	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
54	zhuti.xiaomi.com		http	/detail/fff89de2-d32d-44a8-8dc0-69c9fdc6ff58		http://zhuti.xiaomi.com/detail/fff89de2-d32d-44a8-8dc0-69c9fdc6ff58	/detail/fff89de2-d32d-44a8-8dc0-69c9fdc6ff58	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
55	zhuti.xiaomi.com		http	/detail/fff7c956-8070-4c58-9ec7-c0db8ccd4c92		http://zhuti.xiaomi.com/detail/fff7c956-8070-4c58-9ec7-c0db8ccd4c92	/detail/fff7c956-8070-4c58-9ec7-c0db8ccd4c92	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
56	zhuti.xiaomi.com		http	/detail/fff4ac1d-6370-43df-808d-20186383bdf1		http://zhuti.xiaomi.com/detail/fff4ac1d-6370-43df-808d-20186383bdf1	/detail/fff4ac1d-6370-43df-808d-20186383bdf1	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
57	zhuti.xiaomi.com		http	/detail/fff323eb-5f81-4ab6-8875-640c7f06d6ef		http://zhuti.xiaomi.com/detail/fff323eb-5f81-4ab6-8875-640c7f06d6ef	/detail/fff323eb-5f81-4ab6-8875-640c7f06d6ef	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
58	zhuti.xiaomi.com		http	/detail/ffe6dabc-1805-47f5-bb73-dc6d446f9c75		http://zhuti.xiaomi.com/detail/ffe6dabc-1805-47f5-bb73-dc6d446f9c75	/detail/ffe6dabc-1805-47f5-bb73-dc6d446f9c75	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
59	zhuti.xiaomi.com		http	/detail/ffdf4011-f46a-481e-9c84-af1a8e1717cb		http://zhuti.xiaomi.com/detail/ffdf4011-f46a-481e-9c84-af1a8e1717cb	/detail/ffdf4011-f46a-481e-9c84-af1a8e1717cb	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
60	zhuti.xiaomi.com		http	/detail/ffd19f4e-f05b-472f-9eef-5e9657888472		http://zhuti.xiaomi.com/detail/ffd19f4e-f05b-472f-9eef-5e9657888472	/detail/ffd19f4e-f05b-472f-9eef-5e9657888472	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
61	zhuti.xiaomi.com		http	/detail/ffd0001b-b15a-4ab7-89f8-db9e72a0df2b		http://zhuti.xiaomi.com/detail/ffd0001b-b15a-4ab7-89f8-db9e72a0df2b	/detail/ffd0001b-b15a-4ab7-89f8-db9e72a0df2b	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
62	zhuti.xiaomi.com		http	/detail/ffb9fc96-65d1-4fab-bd84-bb1535aab286		http://zhuti.xiaomi.com/detail/ffb9fc96-65d1-4fab-bd84-bb1535aab286	/detail/ffb9fc96-65d1-4fab-bd84-bb1535aab286	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
63	zhuti.xiaomi.com		http	/detail/ffb579e2-47cc-4414-9256-d8392741e02e		http://zhuti.xiaomi.com/detail/ffb579e2-47cc-4414-9256-d8392741e02e	/detail/ffb579e2-47cc-4414-9256-d8392741e02e	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
64	www.xiaomi.com		http	/		http://www.xiaomi.com/	/	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
65	zhuti.designer.xiaomi.com		https	/		https://zhuti.designer.xiaomi.com/	/	http://zhuti.xiaomi.com/		443	L2RvbS9tdWNoX2RhdGE=
66	zhuti.xiaomi.com		http	/agreement		http://zhuti.xiaomi.com/agreement	/agreement	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
67	file.market.xiaomi.com		http	/download/ThemeMarket/006fa5b89c49209271b15afbc06410fb1da436900		http://file.market.xiaomi.com/download/ThemeMarket/006fa5b89c49209271b15afbc06410fb1da436900	/download/ThemeMarket/006fa5b89c49209271b15afbc06410fb1da436900	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
68	file.market.xiaomi.com		http	/download/ThemeMarket/0e878b4dff45441852650260355d459e13110c413		http://file.market.xiaomi.com/download/ThemeMarket/0e878b4dff45441852650260355d459e13110c413	/download/ThemeMarket/0e878b4dff45441852650260355d459e13110c413	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
69	file.market.xiaomi.com		http	/download/ThemeMarket/0b14a51f4b4d5500d4af8a0d6a0881e7c2d417c0d		http://file.market.xiaomi.com/download/ThemeMarket/0b14a51f4b4d5500d4af8a0d6a0881e7c2d417c0d	/download/ThemeMarket/0b14a51f4b4d5500d4af8a0d6a0881e7c2d417c0d	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
70	file.market.xiaomi.com		http	/download/ThemeMarket/0b14a51f484d5a00d5af800d600887e7cdd417c0d		http://file.market.xiaomi.com/download/ThemeMarket/0b14a51f484d5a00d5af800d600887e7cdd417c0d	/download/ThemeMarket/0b14a51f484d5a00d5af800d600887e7cdd417c0d	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
71	file.market.xiaomi.com		http	/download/ThemeMarket/0fc3c44ea836b41be0826af2d516e8abd42597f69		http://file.market.xiaomi.com/download/ThemeMarket/0fc3c44ea836b41be0826af2d516e8abd42597f69	/download/ThemeMarket/0fc3c44ea836b41be0826af2d516e8abd42597f69	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
72	file.market.xiaomi.com		http	/download/ThemeMarket/07e5751f0467c6108c711ca5781ac573dff420213		http://file.market.xiaomi.com/download/ThemeMarket/07e5751f0467c6108c711ca5781ac573dff420213	/download/ThemeMarket/07e5751f0467c6108c711ca5781ac573dff420213	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
73	file.market.xiaomi.com		http	/download/ThemeMarket/0536b94b8bdd448380fbb64c60444666533eef3b1		http://file.market.xiaomi.com/download/ThemeMarket/0536b94b8bdd448380fbb64c60444666533eef3b1	/download/ThemeMarket/0536b94b8bdd448380fbb64c60444666533eef3b1	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
74	file.market.xiaomi.com		http	/download/ThemeMarket/06b7d46a719325f8fb85f63711970b9757041b103		http://file.market.xiaomi.com/download/ThemeMarket/06b7d46a719325f8fb85f63711970b9757041b103	/download/ThemeMarket/06b7d46a719325f8fb85f63711970b9757041b103	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
75	t5.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/00530240b3a884a970f4a8d820bf80ee86227a5fa		http://t5.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/00530240b3a884a970f4a8d820bf80ee86227a5fa	/thumbnail/jpeg/w118/ThemeMarket/00530240b3a884a970f4a8d820bf80ee86227a5fa	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
76	t2.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/025b8e44adc9a4efd3531f366bb343dbc0c70f647		http://t2.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/025b8e44adc9a4efd3531f366bb343dbc0c70f647	/thumbnail/jpeg/w118/ThemeMarket/025b8e44adc9a4efd3531f366bb343dbc0c70f647	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
77	t5.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/00b845994f9434d42c9d00daf07006581ec43677c		http://t5.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/00b845994f9434d42c9d00daf07006581ec43677c	/thumbnail/jpeg/w118/ThemeMarket/00b845994f9434d42c9d00daf07006581ec43677c	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
78	t5.market.xiaomi.com		http	/thumbnail/jpeg/w118/ThemeMarket/000f435a74abc4d872dfca30f85b12c534af0bd62		http://t5.market.xiaomi.com/thumbnail/jpeg/w118/ThemeMarket/000f435a74abc4d872dfca30f85b12c534af0bd62	/thumbnail/jpeg/w118/ThemeMarket/000f435a74abc4d872dfca30f85b12c534af0bd62	http://zhuti.xiaomi.com/		80	L2RvbS9tdWNoX2RhdGE=
\.


--
-- Name: dump_collector_id_seq; Type: SEQUENCE SET; Schema: public; Owner: y0h4n3s
--

SELECT pg_catalog.setval('public.dump_collector_id_seq', 78, true);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: y0h4n3s
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: dump_collector dump_collector_pkey; Type: CONSTRAINT; Schema: public; Owner: y0h4n3s
--

ALTER TABLE ONLY public.dump_collector
    ADD CONSTRAINT dump_collector_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

