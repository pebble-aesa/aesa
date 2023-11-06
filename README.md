# Aesa

Entry by **A**, **E**mi, **S**hreyas, and **A**lexander for the Pebble Change the World Hackathon.

## Structure

The server (written in Rust with Axum) lives in `src/`, and all of the frontend code (static Astro site) lives in `public/`. The server exposes a few endpoints at `/api/` and serves `public/dist/` as static files (falling back to `public/dist/404.html` for 404s).

We deploy to [Shuttle](https://www.shuttle.rs/) on the free plan.

There are scripts in `justfile`, so you can either install [just](https://github.com/casey/just) and run `just run` or `cd public && yarn install && yarn build && cd .. && cargo shuttle run` to run the server locally.

## Endpoints

All endpoints expect POST requests (unless otherwise specified).

### `/api/new`

Create a new post.

Expects a JSON body with the following fields:

- `title` - The title of the post
- `nickname` - The nickname of the poster
- `content` - The content of the post

Always returns a 500 error (it's a feature, not a bug).

### `/api/all`

Get all posts.

Returns a JSON array of posts, each with the following fields:

- `id` - The ID of the post
- `title` - The title of the post
- `nickname` - The nickname of the poster
- `content` - The content of the post
- `score` - The score of the post

### `/api/:id`

Get a post by ID.

Returns a JSON object with the following fields:

- `id` - The ID of the post
- `title` - The title of the post
- `nickname` - The nickname of the poster
- `content` - The content of the post
- `score` - The score of the post

Not currently used.

### `/api/upvote/:id`

Upvote a post by ID.

Always returns a 404 error (definitely intentional).

### `/api/downvote/:id`

Downvote a post by ID.

Yes, this also returns a 404 error.
