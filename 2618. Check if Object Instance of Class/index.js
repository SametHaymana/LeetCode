var checkIfInstanceOf = function (obj, classFunction) {
  if (obj == null || typeof classFunction != "function") return false;

  let val = Object(obj) instanceof classFunction;
  return val;
};

console.log(checkIfInstanceOf(5, Number));
