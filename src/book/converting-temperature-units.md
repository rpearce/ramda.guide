# Converting Temperature Units

> Life is far too important a thing ever to talk seriously about.
>
> — <cite>Oscar Wilde, Lady Windermere's Fan</cite>

To kick off our Ramda journey, we're going to do something ridiculous: transform
very clear temperature conversion functions that use JS math operators to use
only functions for the operations!

Sometimes, when we forego the obvious and choose to approach problems in
different ways, interesting patterns may emerge that can expand our
understanding.

Let's get introduced to some perfectly fine conversion functions — one of which
we are going to rip apart and make anew.

```javascript
function celsiusToFahrenheit(celsius) {
  return celsius * (9 / 5) + 32
}

function fahrenheitToCelsius(fahrenheit) {
  return 5 / 9 * (fahrenheit - 32)
}

function easyCelsiusToFahrenheit(celsius) {
  return celsius * 2 + 30
}

function easyFahrenheitToCelsius(fahrenheit) {
  return (fahrenheit - 30) / 2
}
```

While the `celsiusToFahrenheit` and `fahrenheitToCelsius` functions are exact
formulas, they're not practical for everyday use. I've lived in the UK and New
Zealand, and since I'm married to a Kiwi, I need to easily convert between
Celsius and Fahrenheit. While not exact, the `easyCelsiusToFahrenheit` and
`easyFahrenheitToCelsius` formulas are easy to do in one's head and are close
enough to the real values.

We are going to single out `celsiusToFahrenheit` for this extended example.

```javascript
function celsiusToFahrenheit(celsius) {
  return celsius * (9 / 5) + 32
}
```

## Enter the Ramda

In it, we:

1. multiply the Celsius value by the result of `9 / 5`
1. add `32` to the result of the prior step(s)

Before we go further, let's first convert it to an arrow function expression,
for doing so will open some interesting doors.

```javascript
const celsiusToFahrenheit = celsius =>
  celsius * (9 / 5) + 32
```

Next, let's get Ramda pulled into the picture.

Ramda has a number of math functions, namely
[`multiply`](https://ramdajs.com/docs/#multiply),
[`divide`](https://ramdajs.com/docs/#divide), and
[`add`](https://ramdajs.com/docs/#add) that we can leverage in place of `*`,
`/`, and `+`. Each function takes two arguments, and each function will wait to
evaluate itself until you provide all the arguments. Check this out:

```javascript
add(1, 2)     // 3
add(1)(2)     // 3
add()(1, 2)   // 3
add()(1)()(2) // 3
```

This is indeed weird, and we'll cover this fully in the "Core Ramda Ideas"
section.

For now, let's import those and use them!

```javascript
import { add, divide, multiply } from 'ramda'

const celsiusToFahrenheit = celsius =>
  add(multiply(celsius, divide(9, 5)), 32)

celsiusToFahrenheit(100) // 212
```

Woah, woah, woah! What's going on here?!

It looks like we're...

1. adding `32` to the result of
1. multiplying the Celsius value by the result of dividing `9` by `5`

That's the same process we did before, but it's merely explained differently!

With addition and multiplication, there's something called the
[commutative law](https://www.mathsisfun.com/definitions/commutative-law.html)
that states we can provide the arguments to an addition and multiplication
operation in any order. Let's leverage this law in order to move our variable,
`celsius`, further toward the edge of our function to judge how it feels.

```javascript
// this is what we're starting with
add(multiply(celsius, divide(9, 5)), 32)

// first, swap `celsius` and `divide(9/5)`
add(multiply(divide(9, 5), celsius), 32)
//               ^------------^

// next, swap the multiplication and `32`
add(32, multiply(divide(9, 5), celsius))
//  ^------^

// the result
const celsiusToFahrenheit = celsius =>
  add(32, multiply(divide(9, 5), celsius))
```

Interesting! Do you see it yet? The forwarding of a result from function to
function? Let's look at this another way:

```javascript
const celsiusToFahrenheit = celsius => {
  const multiplied = multiply(divide(9, 5), celsius)
  const added = add(32, multiplied)

  return added
}
```

We provide `celsius` as the second argument to `multiply`, then we provide the
result of that as the second argument to `add`. We're simply forwarding the
evaluated result of a computation to another function; kind of like passing an
electric guitar's signal through a few effects pedals and then out the
amplifier.

What if we had a cleaner way to link these functions together so we can easily
understand what `celsiusToFahrenheit` is composed of and then provide the data
at the end?

It's time to take this first lesson into overdrive.

## A Taste of Composition

We need a way of passing the result of calling one function to another function
and having that run. It'd be easier if we could abstract an API... let's try
that.

```javascript
// this is essentially what we have
// with our celsiusToFahrenheit
f2(f1(value))

// but we want something like this;
// let's call it `link` because
// we're linking functions together
link(f2, f1)(value)
```

With that desired outcome in mind, let's try to write `link`!

```javascript
const link = (f2, f1) => value =>
  f2(f1(value))
```

Ha! We're still doing the difficult to follow `f2(f1(value))`, but now we can
use this like `link(f2, f1)(value)`.

Circling back to `celsiusToFahrenheit`, let's try to use this `link`
abstraction:

```javascript
// before
const celsiusToFahrenheit = celsius =>
  add(32, multiply(divide(9, 5), celsius))

// after
const celsiusToFahrenheit = celsius =>
  link(add(32), multiply(divide(9, 5)))(celsius)

celsiusToFahrenheit(100) // 212
```

Nice! We can now do a little less inside-out reading. But something doesn't feel
quite right... Why are we accepting the argument `celsius` in our
`celsiusToFahrenheit` function only to turn right back around and call `link()`
with the `celsius` value? Do we need it?

Nope.

```javascript
// before
const celsiusToFahrenheit = celsius =>
  link(add(32), multiply(divide(9, 5)))(celsius)

// after
const celsiusToFahrenheit =
  link(add(32), multiply(divide(9, 5)))

celsiusToFahrenheit(100) // 212
```

You may be wondering why `link` reads right to left. Two short answers are:

1. Mathematics writes `f(x)` and not `(x)f`
1. Evaluation is done from right to left (inside -> outside), so we are
  [right-associative](https://en.wikipedia.org/wiki/Operator_associativity)

However, let me ease your worried mind and make a `linkL` (`L` for "left")
function for us to use:

```javascript
const linkL = (f1, f2) => value =>
  f2(f1(value))
```

And when we compare that to the original function, we realize that we've come
nearly full circle but with a whole new perspective:

```javascript
// where we started
const celsiusToFahrenheit = celsius =>
  celsius * (9 / 5) + 32
//        ^    ^    ^
//  multiply   |    |
//           divide |
//                 add

// where we ended up
const celsiusToFahrenheit =
  linkL(multiply(divide(9, 5)), add(32))
```

Ramda provides a few functions, [`compose`](https://ramdajs.com/docs/#compose)
(or [`o`](https://ramdajs.com/docs/#o)) and
[`pipe`](https://ramdajs.com/docs/#pipe) that do the `link` and `linkL` work for
us!

```javascript
import {
  add,
  compose,
  divide,
  multiply,
  pipe,
} from 'ramda'

// `compose` and `o` are very similar
const celsiusToFahrenheit =
  compose(add(32), multiply(divide(9, 5)))

// `pipe`
const celsiusToFahrenheit =
  pipe(multiply(divide(9, 5)), add(32))
```

We'll cover function composition a bit more in the "Core Ramda Ideas" section.

## Your Turn

Can you convert the remaining temperature conversion functions to use Ramda
functions? Give them a try in [a pre-loaded Ramda
REPL](https://tinyurl.com/ygbkvfj4).

Here they are again, in case that link doesn't work:

```javascript
function fahrenheitToCelsius(fahrenheit) {
  return 5 / 9 * (fahrenheit - 32)
}

function easyCelsiusToFahrenheit(celsius) {
  return celsius * 2 + 30
}

function easyFahrenheitToCelsius(fahrenheit) {
  return (fahrenheit - 30) / 2
}

const result = () => ({
  '212F = 100C': fahrenheitToCelsius(212),
  '25C ≈ 80F': easyCelsiusToFahrenheit(25),
  '60F ≈ 15C': easyFahrenheitToCelsius(60),
})

result()
```

When you're done, compare them against [my
solutions](https://tinyurl.com/yhe4pm2w)!

<details>
  <summary>Expand this to see my solutions if the link doesn't work</summary>

```javascript
//function fahrenheitToCelsius(fahrenheit) {
//  return 5 / 9 * (fahrenheit - 32)
//}

// This is the best I could do before I had to cheat... see below!
//const fahrenheitToCelsius = fahrenheit =>
//  multiply(divide(5, 9), subtract(fahrenheit, 32))

// If you're feeling clever, check this out:
// https://ramdajs.com/docs/#__
const fahrenheitToCelsius =
  compose(multiply(divide(5, 9)), subtract(__, 32))

// ===============================================================

//function easyCelsiusToFahrenheit(celsius) {
//  return celsius * 2 + 30
//}

// Step 1:
//const easyCelsiusToFahrenheit = celsius =>
//  add(30, multiply(2, celsius))

// Step 2:
const easyCelsiusToFahrenheit =
  compose(add(30), multiply(2))

// ===============================================================

//function easyFahrenheitToCelsius(fahrenheit) {
//  return (fahrenheit - 30) / 2
//}

// This is the best I could do before I had to cheat... see below!
//const easyFahrenheitToCelsius = fahrenheit =>
//  divide(subtract(fahrenheit, 30), 2)

// If you're feeling clever, check this out:
// https://ramdajs.com/docs/#__
const easyFahrenheitToCelsius =
  compose(divide(__, 2), subtract(__, 30))

// ===============================================================

const result = () => ({
  '212F = 100C': fahrenheitToCelsius(212),
  '25C ≈ 80F': easyCelsiusToFahrenheit(25),
  '60F ≈ 15C': easyFahrenheitToCelsius(60),
})

result()
```
</details>

## Wrapping Up

This turned out to be far from a gentle introduction!

We started with some addition, division, and multiplication to convert
temperature values, and we ended up walking backwards into the heart of
functional programming.

Way to go!
