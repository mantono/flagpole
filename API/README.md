# API
See also the [Swagger UI rendered version](https://42x.io/flagpole) of the [Open API specification](../open_api.yaml).

Requests that does not alter state (GET & HEAD) will _never_ require any authentication. This makes it also suitable for
front end services, since no secrets or credentials are exposed to any end user.

Requests that may change the state (PUT & DELETE) will require authenticaiton if flagpole was required with an API key.
In such case the authorization header must be sent with the proper API key.
Example: `authorization: ApiKey SECRET_API_KEY`

## GET `/flags/{namespace}`
Get all the enabled feature flags in a namespace.

## HEAD `/flags/{namespace}`
Check if the feature flags have been updated for a namespace. The response will contain the [etag](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag) header.
If the etag header value is the same as in the most recent GET or HEAD request, the namespace's feature flags have not changed.

## PUT `/flags/{namespace}/{flag}`
Enable a feature flag in a namespace.

## DELETE `/flags/{namespace}/{flag}`
Disable a feature flag in a namespace.