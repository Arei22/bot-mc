create table public.servers (
  id BIGSERIAL primary key not null,
  name text unique not null,
  version text not null,
  difficulty text not null,
  port BigInt not null,
  started boolean
);