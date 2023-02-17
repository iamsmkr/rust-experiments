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
