import pyo3_example
from pyo3_example import Str, Int

people = pyo3_example.People()

for p in people.get_people():
    print(p)

for p in people.get_people():
    print(p.id)


s = Str("pomtery")
pyo3_example.print_prop(s)
# value = pomtery

i = Int(12345)
pyo3_example.print_prop(i)
# value = 12345

pyo3_example.get_props({"name": "Shivam Kapoor", "age": 35, "hobbies": [1, 2, 3]})
# K = name, V = String("Shivam Kapoor")
# K = age, V = Int(35)
# K = hobbies, V = Vec([1, 2, 3])


def greet(name):
    print(f"Hello {name}")

pyo3_example.invoke_passed_func(greet)
# Pometry

pyo3_example.invoke_passed_func3(greet)
# Raphtory
