create table contact (
    id serial primary key,
    uuid text not null,
    name text not null,
    address text,
    zipcode text,
    city text,
    country text
);