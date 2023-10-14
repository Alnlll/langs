import json

if "__main__" == __name__:
  # from string
  o_json = '["foo", {"bar": ["bax", null, 1.0, 2]}]'
  o = json.loads(o_json)
  print(o)

  # to string
  s = json.dumps(["foo", {"bar": ["bax", None, 1.0, 2]}])

  # load object
  o = json.loads(s)
  print(o)
