from functools import reduce

def do_someting(x):
  def do_something_else(y):
    return y + 1
  return do_something_else(x)

if __name__ == "__main__":
  print(do_someting(1))

  example_closure = lambda x : x
  print(example_closure('hello'))
  print(example_closure(5))

  x = [1, 2, 3]
  equal_to_x = lambda z : x == z
  assert equal_to_x([1, 3, 2]) == False

  items = [1, 2, 3, 4, 5]
  plus_one = lambda x : x + 1
  added_one = list(map(plus_one, items))
  added_one_sum = reduce(lambda x, y : x + y, added_one)

  print(added_one, added_one_sum)