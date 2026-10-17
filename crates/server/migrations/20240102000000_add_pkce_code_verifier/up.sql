-- PKCE (RFC 7636): the `code_verifier` behind each login's `code_challenge`.

ALTER TABLE auth_flows ADD COLUMN code_verifier TEXT NOT NULL DEFAULT '';
