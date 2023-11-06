# Aesa

Entry by **A**, **E**mi, **S**hreyas, and **A**lexander for the Pebble Change the World Hackathon.

## Structure

The server (written in Rust with Axum) lives in `src/`, and all of the frontend code (static Astro site) lives in `public/`. The server exposes a few endpoints at `/api/` and serves `public/dist/` as static files (falling back to `public/dist/404.html` for 404s).

We deploy to [Shuttle](https://www.shuttle.rs/) on the free plan.
