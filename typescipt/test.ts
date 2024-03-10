function divideArray(nums: number[], k: number): number[][] {
  nums.sort((a, b) => a - b);
  let arr: number[][] = [];
  for (let i = 0; i < nums.length; i += 3) {
    const tempArr: number[] = [];
    tempArr.push(nums[i]);
    tempArr.push(nums[i + 1]);
    tempArr.push(nums[i + 2]);

    const avarage = tempArr.reduce((a, b) => a + b) / 3;
    if (-(avarage - tempArr[2]) >= k) {
      return [];
    }

    arr.push(tempArr);
  }

  return arr;
}

const nums = [1, 3, 3, 2, 7, 3],
  k = 3;

console.log(divideArray(nums, k));

console.log(nums.sort((a, b) => a - b));
