create schema events;

create type events.kind as enum ('trial', 'pvp');
create type events.scope as enum ('public', 'private', 'semi-public');
create type events.role as enum ('tank', 'healer', 'brawler', 'bomber', 'ganker', 'dd', 'reserve', 'absent');
create type events.class as enum ('arcanist', 'necromancer', 'warden', 'dragon-knight', 'templar', 'sorcerer', 'night-blade');

create table events.events (
    message_id bigint primary key not null,
    kind events.kind not null,
    scope events.scope not null,
    title varchar not null,
    description varchar not null,
    datetime TIMESTAMPTZ,
    duration varchar not null,
    leader bigint not null,
    scheduled_event bigint,
    created_at TIMESTAMPTZ not null default (now() at time zone 'utc')
);

create table events.player_roles (
    message_id bigint not null references events.events(message_id) on delete cascade,
    role events.role not null,
    max smallint,
    created_at TIMESTAMPTZ not null default (now() at time zone 'utc')
);

create table events.players (
    message_id bigint not null references events.events(message_id) on delete cascade,
    role events.role not null,
    user_id bigint not null,
    name varchar not null,
    class events.class,
    created_at TIMESTAMPTZ not null default (now() at time zone 'utc')
);

create table events.flex_roles (
    message_id bigint not null references events.events(message_id) on delete cascade,
    user_id bigint not null,
    role events.role not null,
    created_at TIMESTAMPTZ not null default (now() at time zone 'utc')
)
