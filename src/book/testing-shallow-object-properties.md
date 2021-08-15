# Testing Shallow Object Properties

In this chapter, we'll make logical decisions in our code (`if`/`else`) by
testing our datasets' properties with a few boolean-returning helper functions.

Outline:

* [`has`](#has)
* [`propEq`](#propeq)
* [`propIs`](#propis)
* [`propSatisfies`](#propsatisfies)

In case you missed it, we'll be using [some ISS and astronaut
data](./data-iss-location.html) as our datasets.

<details><summary>ISS' Current Location</summary>

```json
{
  "message": "success",
  "timestamp": 1617930803,
  "iss_position": {
    "latitude": "27.7270",
    "longitude": "133.2581"
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

## `has`

We're working one morning, eating scones and refactoring some code, when a
fellow developer, who lives in Iceland and started work a few hours before we
did today, pings us with the following message:

> Hjálp!
>
> We shipped some astronaut code this morning that puts a method, `toString`, on
> some `astros` objects, and it joins all the astronauts' names together with
> rocket ships like this:
>
> "Sergey Ryzhikov 🚀 Kate Rubins 🚀 ..."
>
> Pretty cool, right?
>
> We only want to use this method on the `astros` objects where it's
> defined, but we forgot that `toString` is already a defined method defined on
> an `Object` instance, so some `astros` objects are calling that method and
> returning `"[object Object]"` when we want them to do something else!
>
> Can you help us? Takk!

Our Icelandic coworker then sends us the code:

```javascript
const astrosWithToString = {
  "message": "success",
  "number": 7,
  "people": [/* omitted for brevity */],
  toString() {
    return astrosPeopleWithRockets(this)
  }
}

const astrosWithoutToString = {
  "message": "success",
  "number": 7,
  "people": [/* omitted for brevity */],
}

const astrosToString = data => {
  if ('toString' in data) { // THIS IS WHERE THE BUG HAPPENS!
    return data.toString()
  }

  return `There are ${data.number} astronauts`
}

// astrosToString(astrosWithToString) // this works
astrosToString(astrosWithoutToString) // this doesn't!
```

[View this buggy `astrosToString` code in the Ramda
REPL.](https://tinyurl.com/yzqnul84)

_The `astrosPeopleWithRockets` code comes from the prior lesson on ["Reading
Shallow Object Properties"](./reading-shallow-object-properties.html), so check
that out to see how we arrived at the nifty little helper functions you'll find
in the REPL linked above._

Aha! We see where the misunderstanding happened. The `'toString' in data` code
is checking that there is a property defined on the object called `toString` —
whether or not it inherited that property! All object instances have a
`.toString()` method that they inherit, so it'll always have `toString` defined
no matter what. What we want is to check if `toString` was explicitly defined by
us on the `astros` object.

We first confirm our assumption and fix the bug by making this change:

```javascript
// before
if ('toString' in data) {/*...*/}

// after
if (Object.prototype.hasOwnProperty.call(data, 'toString')) {/*...*/}
```

The `hasOwnProperty` exists on all objects, but it could be overwritten like
`toString` was, so we use an [_external_ `hasOwnProperty`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwnProperty)
to do our check.

This check seems rather lengthy, so let's convert that to a function named
`hasProp`:

```javascript
const hasProp = (prop, obj) =>
  Object.prototype.hasOwnProperty.call(obj, prop)
```

Surprise! Ramda already has a helper for this,
[`has`](https://ramdajs.com/docs/#has), so we can replace our `hasProp` with
`has`.

We then write a message back to our Icelandic colleague:

> Hæ!
>
> We found the issue, and it's a matter of testing whether _we_ defined the
> object property or not. Here you go!
>
> Eigðu góðan dag!

```javascript
import { has } from 'ramda'

// ...

const astrosToString = data => {
  if (has('toString', data)) {
    return data.toString()
  }

  return `There are ${data.number} astronauts`
}
```

[View the updated `astrosToString` and `hasProp` functions in the Ramda
REPL.](https://tinyurl.com/ygl3tr4u)

## `propEq`

## `propIs`

## `propSatisfies`
