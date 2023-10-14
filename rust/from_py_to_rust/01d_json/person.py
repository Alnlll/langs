import json
from typing import Any

class Person:
  def __init__(self, name, age, job, verified, parents):
    self.name = name
    self.age = age
    self.job = job
    self.verified = verified
    self.parents = parents
  def __str__(self):
    return ", ".join([f"{k}:{v}" for k,v in self.__dict__.items()])

class PersonEncoder(json.JSONEncoder):
  def default(self, o):
    return o.__dict__

class PersonDecoder(json.JSONDecoder):
  def decode(self, s):
    d = json.JSONDecoder.decode(self,s)
    return Person(**d)

if __name__ == "__main__":
  bob = Person("bob", 12, None, True, ["Alice", "Carl"])
  print(bob)
  bob_json = json.dumps(bob, cls=PersonEncoder)
  print(bob_json, type(bob_json))
  bob = json.loads(bob_json, cls=PersonDecoder)
  print(bob, type(bob))