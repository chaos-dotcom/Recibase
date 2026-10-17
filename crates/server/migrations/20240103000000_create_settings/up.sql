-- Small key/value store for runtime settings: currently the adopted owner
-- subject (the first OIDC `sub` to sign in when none is pinned by the env).

CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
