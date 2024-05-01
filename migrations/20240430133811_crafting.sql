create schema crafting;

create type crafting.kind as enum ('set', 'enchant', 'consumables', 'research');

create table crafting.orders (
    id serial primary key,              -- Id auto-generado para identificar la comanda
    kind crafting.kind not null,        -- Tipo de crafteo (sets, encantamientos, consumibles, investigacion..)
    owner bigint not null,              -- El que lo ha pedido
    crafter bigint,                     -- El crafter que la coge (cuando la coja, por defecto vacio)
    serialized_order text not null,     -- El contenido del pedido entero en texto
    completed bool default false,       -- Pedido completado o no
    created_at TIMESTAMPTZ not null default (now() at time zone 'utc') -- la hora y fecha de la peticion
);
