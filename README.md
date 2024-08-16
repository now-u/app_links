For now this is just playing around with dynamic links alternatives. If this seems viable we might use it for dynamic links going forward.

## Short term

1. Update app to handle app.now-u.com links with support for app links etc
1. Update this thing to redirect to play store on android (i.e. if not handle by native app)
1. Try and host this bad boi
1. Test...

## Long term

The goal would be to add support for link previews

e.g. for twitter we would want to add https://developer.x.com/en/docs/twitter-for-websites/cards/guides/getting-started#started

My thinking for this was we could add in parameters for the preview image etc

The issue with that approach is we will get very long links. Potentially we should also shorten those things...


Potentailly in the future we could auto generate shorted URL in django app for each resource (and update as updated). Probably the api should provide a deep link in the response regardless of whether its stored here?
