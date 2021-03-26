# Including Ramda in A Project

Following the instructions from [Ramda's homepage](https://ramdajs.com), to
install Ramda into a project via [NPM](https://www.npmjs.com), you run

```bash
λ npm i ramda
```

If you're building for the frontend and are using a build tool that has
[tree-shaking or dead-code elimination](https://developers.google.com/web/fundamentals/performance/optimizing-javascript/tree-shaking),
then here is how you should import functions from Ramda:

```js
import { compose, lensProp, map, over } from 'ramda'
```

However, if you do not have a build tool that does tree-shaking, you may want to
import directly from the files you use to avoid importing the entire Ramda
library when you only want to use a few functions. The two options with
[`v0.27.1`](https://www.npmjs.com/package/ramda/v/0.27.1) are ESModules- and
CommonJS-based.

```js
// ESModules
import compose from 'ramda/es/compose'
import lensProp from 'ramda/es/lensProp'

// CommonJS
const compose = require('ramda/src/compose')
const lensProp = require('ramda/src/lensProp')
```
