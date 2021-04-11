# Reading Shallow Object Properties

Outline:

* [`prop`](#prop)
* [`propOr`](#propor)
* [`props`](#props)
* [`pick`](#pick)

Let's dive (but not too deep) into pulling out data at a single level from
objects!

In case you missed it, we'll be using [some ISS and astronaut
data](./data-iss-location.html) as our datasets.

<details><summary>ISS' Current Location</summary>

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

</details>


<details><summary>"How Many People Are In Space Right Now?"</summary>

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

</details>

We will assume we're storing those objects as variables named `iss` and
`astros`.

## `prop`

In vanilla JS, we can get the `timestamp` key off the ISS object like this:

```javascript
iss.timestamp

// or

iss['timestamp']
```

If that's the only functionality we'll ever need, then that's great! We can stop
here.

But what if we want to do more? For example,

1. get the `timestamp`
1. multiply the timestamp by `1000` to convert it to milliseconds
1. convert it to a `Date` string

Our first attempt might be to do this inline:

```javascript
new Date(iss.timestamp * 1000)
// "Sat Apr 10 2021 15:06:50 GMT+0000 (Coordinated Universal Time)"
```

Then we realize that we want to do this for many different ISS location objects,
so we write a function:

```javascript
const issTimeToDate = data =>
  new Date(data.timestamp * 1000)

issTimeToDate(iss)
// "Sat Apr 10 2021 15:06:50 GMT+0000 (Coordinated Universal Time)"
```

It is totally acceptable to stop at this point.

Maybe we should, but we don't.

Squinting at that code a little harder, we notice that there are three
transformations happening:

1. from the ISS object, we get the shallow property, `timestamp`
1. we multiply that value by `1000`
1. we instantiate a new `Date` with the prior result

And we also notice that if `data` is ever `undefined` or `null` (or anything
that isn't an instance of `Object`), we're going to have a problem!

```javascript
issTimeToDate(null)
// Uncaught TypeError: can't access property "timestamp", data is null
```

As you may recall from our ["First Taste of
Composition"](/converting-temperature-units.html#a-taste-of-composition), if we
extract each operation into its own function, there is a way we can "link" these
fuctions together: [`compose`](https://ramdajs.com/docs/#compose)!

```javascript
// Here we create a reusable function that
// receives an object property, then returns
// a function that accepts an object, then
// tries to access that property on the object
const getProp = property => data => {
  if (data instanceof Object) {
    return data[property]
  }
}

const toMilliseconds = n => n * 1000
const toDate         = n => new Date(n)

const issTimeToDate =
  compose(toDate, toMilliseconds, getProp('timestamp'))

issTimeToDate(iss)
// "Sat Apr 10 2021 15:06:50 GMT+0000 (Coordinated Universal Time)"
```

[View this `getProp` with `compose` example in the Ramda
REPL.](https://tinyurl.com/yz93mlt6)

While this doesn't handle all edge cases, at least passing `null` to
`issTimeToDate` will give us an `Invalid Date` message.

That `getProp` function looks like it's fairly generic, but could it handle an
`Array`? Could we leverage it to figure out who the first astronaut is in the
`astros.people` list?

```javascript
compose(getProp(0), getProp('people'))(astros)
// {
//   "craft": "ISS",
//   "name": "Sergey Ryzhikov"
// }

// which can be refactored and reused
// with any group of astronauts

const getFirstAstro =
  compose(getProp(0), getProp('people'))

getFirstAstro(astros)

// and if you really want to get some
// reusable functions

const getPeople     = getProp('people')
const getFirst      = getProp(0)
const getFirstAstro = compose(getFirst, getPeople)

getFirstAstro(astros)
// {
//   "craft": "ISS",
//   "name": "Sergey Ryzhikov"
// }
```

[View this `getFirstAstro` example in the Ramda
REPL.](https://tinyurl.com/ye4ja9b9)

It can handle an `Array`! Why?

```javascript
[] instanceof Object // true
```

An `Array` of `[5, 10, 15]` is an `Object` instance whose keys are `Array`
indices!

```javascript
Array(3) {
  0: 5,
  1: 10,
  2: 15,
  length: 3
}
```

This means `getProp(1)([5, 10, 15]) === 10`. Neat!

As you probably guessed by now, Ramda has a
[`prop`](https://ramdajs.com/docs/#prop) function that does what our `getProp`
function does (and more), and there are a couple of other functions we could
pull in to help us. Let's refactor!

The ISS example:

```javascript
import { compose, multiply, prop } from 'ramda'

const getTimestamp   = prop('timestamp')
const toMilliseconds = multiply(1000)
const toDate         = n => new Date(n)

const issTimeToDate =
  compose(toDate, toMilliseconds, getTimestamp)

issTimeToDate(iss)
// "Sat Apr 10 2021 15:06:50 GMT+0000 (Coordinated Universal Time)"
```

[View this final `issTimeToDate` example in the Ramda
REPL.](https://tinyurl.com/yzqgh9qs)

Finding the first astronaut example:

```javascript
import { compose, head, prop } from 'ramda'

const getPeople     = prop('people')
const getFirst      = prop(0)
const getFirstAstro = compose(getFirst, getPeople)

getFirstAstro(astros)
// {
//   "craft": "ISS",
//   "name": "Sergey Ryzhikov"
// }
```

[View this final `getFirstAstro` example in the Ramda
REPL.](https://tinyurl.com/yzzfuo2y)

## `propOr`

[`propOr`](https://ramdajs.com/docs/#propOr)

## `props`

[`props`](https://ramdajs.com/docs/#props)

## `pick`

[`pick`](https://ramdajs.com/docs/#pick)
