## Set up

```bash
docker-compose up -d
cargo install sqlx-cli
export DATABASE_URL="postgresql://postgres:postgres@127.0.0.1:9091/polylink"
sqlx database create
cargo run
```

## Project info

For now this is just playing around with dynamic links alternatives. If this seems viable we might use it for dynamic links going forward.

### Short term

1. Update app to handle app.now-u.com links with support for app links etc
1. Update this thing to redirect to play store on android (i.e. if not handle by native app)
1. Try and host this bad boi
1. Test...

### Long term

The goal would be to add support for link previews

e.g. for twitter we would want to add https://developer.x.com/en/docs/twitter-for-websites/cards/guides/getting-started#started

My thinking for this was we could add in parameters for the preview image etc

The issue with that approach is we will get very long links. Potentially we should also shorten those things...


Potentailly in the future we could auto generate shorted URL in django app for each resource (and update as updated). Probably the api should provide a deep link in the response regardless of whether its stored here?

## TODO 

For this to be a real deep links replacement, we would need to add in the "app route" and have the app fetch the link when they are opened to get the route into the app.
We could also allow the user to provide the link_path directly (when creating a link) so the app can use this directly to navigate (this could also be used on the website potentially optionally?)
