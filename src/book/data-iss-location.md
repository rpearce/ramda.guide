# Our Data: The ISS' Location

In this section, we're going to _look up_ object properties in all sorts of
ways, and what better data could we ask for than data for something that is
orbiting our world?

The [Awesome JSON
Datasets](https://github.com/jdorfman/Awesome-JSON-Datasets) repository has
[a NASA section](https://github.com/jdorfman/Awesome-JSON-Datasets#nasa) that
links to Open Notify's [ISS Now](http://open-notify.org/Open-Notify-API/ISS-Location-Now/)
and [How Many People Are In Space Right
Now?](http://open-notify.org/Open-Notify-API/People-In-Space/) endpoints.

Here is an example response that shares where on Earth the International Space
Station is right now:

```json
{
  "message": "success",
  "timestamp": 1617930803,
  "iss_position": {
    "longitude": "133.2581",
    "latitude": "27.7270"
  }
}
```

And here is an example of "How Many People Are In Space Right Now?" as of
2021-04-08:

```json
{
  "message": "success",
  "number": 7,
  "people": [
    {
      "craft": "ISS",
      "name": "Sergey Ryzhikov"
    },
    {
      "craft": "ISS",
      "name": "Kate Rubins"
    },
    {
      "craft": "ISS",
      "name": "Sergey Kud-Sverchkov"
    },
    {
      "craft": "ISS",
      "name": "Mike Hopkins"
    },
    {
      "craft": "ISS",
      "name": "Victor Glover"
    },
    {
      "craft": "ISS",
      "name": "Shannon Walker"
    },
    {
      "craft": "ISS",
      "name": "Soichi Noguchi"
    }
  ]
}
```

We'll use these simple datasets to show off a number of very helpful Ramda
functions that deal with looking up object properties.
