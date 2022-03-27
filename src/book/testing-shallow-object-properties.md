# Testing Shallow Object Properties

> Our destinations are Booleans – we reach them or we don’t – but our journeys
> are spectrums, because there are so many paths we can take to our destination
> that make getting there that much better.
>
> — <cite>A.J. Darkholme</cite>

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

One fine afternoon, our error monitoring service lets us know that our code is
throwing errors when trying to access the latitude property from the ISS'
location API response.

We quickly realize that if there is a problem with the API, we won't get the
data back, so `iss.iss_position.latitude` won't work, for `iss_position` is
`undefined` in the response:

```json
{
  "message": "error",
  "timestamp": 1617930803
}
```

There are many ways to handle this error safely, but we're going to address it
by simply checking if the `message` property is `"success"` or not:

```javascript
if (iss.message === 'success') {
  // carry on...
}
```

Great! Call it a day!

...But our solution nags at us. We are accessing a property's value and equating
it with an expected value. What if we made this a function?

```javascript
const isMessageSuccess = data =>
  (data || {}).message === 'success' // or `data?.message === 'success'`
```

_Note: the Ramda REPL doesn't currently support optional chaining like
`data?.message === 'success'`._

Not bad, but we've simply moved the operations to a single place. What if we
wrote a function that looked up any property on an object and then compared it
with another value?

```javascript
const doesPropEq = (key, val, data) =>
  (data || {})[key] === val // or `data?.[key] === val`
```

Nice! Let's try it out:

```javascript
const isMessageSuccess = data =>
  doesPropEq('message', 'success', data)

isMessageSuccess(iss) // true
```

[View `doesPropEq` and `isMessageSuccess` in the ramda
REPL](https://tinyurl.com/yckj6vnd).

Now that we understand our need for `doesPropEq`, we can swap that out with
ramda's [`propEq`](https://ramdajs.com/docs/#propEq).

```javascript
import { propEq } from 'ramda'

// ...

const isMessageSuccess = data =>
  propEq('message', 'success', data)

isMessageSuccess(iss) // true
```

Stopping the `isMessageSuccess` implementation work at this point is totally
acceptable, but we can take it a little further.

Since all functions in ramda are auto-curried, that means that we can refactor
`isMessageSuccess` like this:

```javascript
// Step 0
const isMessageSuccess = data =>
  propEq('message', 'success', data)

// Step 1
const isMessageSuccess = data =>
  propEq('message', 'success')(data)

// Step 2
const isMessageSuccess =
  propEq('message', 'success')

// Step 3
// isMessageSuccess :: ISSData -> Bool
const isMessageSuccess =
  propEq('message', 'success')
```

1. In Step 1, we demonstrate that `propEq` will accept our last argument as a
   separate function call (the result of calling `propEq` the first time will
   wait until it has all the arguments).
2. In Step 2, we realize that accepting an argument and passing it on again is
   redundant, and so we can remove the need for a function closure and instead
   let the result of calling `propEq` with the first two values be what is bound
   to `isMessageSuccess`.
3. In Step 3. we acknowledge that implicitly forwarding a function argument
   comes at the cost of remembering, "What am I passing in, again?" If you don't
   have a type-checker , you can provide some pseudo-types (these ones are in a
   Haskell style) where the data is defined in order to explain what `ISSData`
   is:

   ```javascript
   // ISSData = { message      :: Message
   //           , timestamp    :: UnixTimestamp
   //           , iss_position :: LatLong
   //           }
   //
   // Message = 'success' | 'error'
   //
   // UnixTimeStamp = Number
   //
   // LatLong = { latitude  :: String
   //           , longitude :: String
   //           }
   ```

   The whole point of Step 3 is simply to identify what the expected input and
   output types are so that someone else (or you in 3 months) can easily
   understand a terse function at a glance.

[Check out this `propEq` usage plus these pseudo-types in the ramda
REPL.](https://tinyurl.com/5n8p6k7f)

## `propIs`

TODO

## `propSatisfies`

TODO
