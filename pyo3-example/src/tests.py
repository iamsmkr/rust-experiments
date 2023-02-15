import pyo3_example

people = pyo3_example.People()

for p in people.getPeople():
    print(p)

for p in people.getPeople():
    print(p.id)
