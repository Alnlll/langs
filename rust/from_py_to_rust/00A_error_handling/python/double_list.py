def double_list(input_strs: [str]):
  try:
    nums = [int(i) for i in input_strs]
  except ValueError:
      print("Please enter a valid number list")
      return []

  return [2*int(n) for n in nums]

if __name__ == "__main__":
  double1 = double_list(["23", "33", '43'])
  print(f"double1 is {double1}")

  double_list([])
  double_list(["tofu", "23", "34"])