# Blueberry V1 API

An free to use API for Blueberry V1. Hosted at `api.blueberry.garden/`

## Endpoints

### Positions

- GET: `/positions`
  - Returns all positions.
- GET: `/positions/:id`
  - Returns a position by ID.
  - `id` (string): The position's ID.
- GET: `/positions/:user`
  - Returns all positions by user.
  - `user` (string): The user's address.

### BLB (Blueberry Governance Token)

- GET: `/blb/circulating_supply`
  - Returns the circulating supply of BLB.
