create type todo_status as enum ('pending', 'in_progress', 'completed');
create type todo_priority as enum ('low', 'medium', 'high');

create table todos (
    id uuid primary key default gen_random_uuid()
    , user_id uuid not null references users(id) on delete cascade
    , title varchar(255) not null
    , description text
    , due_date timestamptz
    , status todo_status not null default 'pending'
    , priority todo_priority not null default 'medium'
    , created_at timestamptz not null default now()
    , updated_at timestamptz not null default now()
);

create index idx_todos_user_id on todos(user_id);
create index idx_todos_status on todos(status);
create index idx_todos_due_date on todos(due_date);
