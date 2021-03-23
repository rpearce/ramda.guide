# Suggested Prerequisite Knowledge

The presentation of concepts in this guide assumes the reader knows JS
fundamentals, so if you're comfortable with the code below, you'll do just fine!

## Arrow Functions

[MDN: Arrow Functions](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Functions#arrow_functions)

```js
const add = (a, b) => a + b

add(4, 5) // 9
```

## Nested Functions

[MDN: Nested Functions and Closures](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Functions#nested_functions_and_closures)

```js
const addExpr = a => b => a + b

addExpr(4)(5) // 9

// or

function addFn(a) {
  return function (b) {
    return a + b
  }
}

addFn(4)(5) // 9
```

## Passing Functions as Arguments (Callbacks)

[MDN: First-Class Function](https://developer.mozilla.org/en-US/docs/Glossary/First-class_Function)

```js
const log = x => console.log(x)

[1, 2, 3].forEach(log)
// 1
// 2
// 3
```

## Map, Filter, & Reduce

MDN: [`map`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Map),
[`filter`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Filter),
and [`reduce`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Reduce)

```js
[1, 2, 3].map(x => x * 2)                      // [2, 4, 6]

[1, 2, 3].filter(x => x % 2 !== 0)             // [1, 3]

[1, 2, 3].reduce((sum, item) => sum + item, 0) // 6
```

## Calling Functions

[MDN: Calling Functions](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Functions#calling_functions)

```js
const add5    = x => x + 5
const times10 = x => x * 10
const div2    = x => x / 2

div2(times10(add5(15))) // 100
```