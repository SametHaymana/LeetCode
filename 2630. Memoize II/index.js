/**
 * @param {Function} fn
 * @return {Function}
 */
function memoize(fn) {
  cache = {};

  const returnMatchValue = (newArgs, cache) => {
    newArgs = JSON.parse(newArgs);
    Object.keys(cache).forEach((key) => {
      const keyArr = JSON.parse(key);
      if (keyArr.length === newArgs.length) {
        let match = true;
        for (let i = 0; i < keyArr.length; i++) {
          if (keyArr[i] !== newArgs[i]) {
            match = false;
          }
        }
        if (match) {
          return cache[key];
        }
      }
    });
  };

  return function () {
    const key = JSON.stringify(arguments);
    const inCache = returnMatchValue(key, cache);
    console.log(inCache);
    if (inCache) {
      return inCache;
    }
    const val = fn.apply(this, arguments);
    cache[key] = val;

    return val;
  };
}

let callCount = 0;
const memoizedFn = memoize(function (a, b) {
  callCount += 1;
  return a + b;
});

memoizedFn(2, 3); // 5
memoizedFn(2, 3); // 5
memoizedFn({}, {}); // 5
console.log(callCount); // 1
