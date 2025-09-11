create table public.servers (
  name text primary key not null,
  version text not null,
  difficulty text not null,
  port BigInt not null,
  started boolean
);