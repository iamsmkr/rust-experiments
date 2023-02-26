import pyo3_example
from pyo3_example import Str, Int, Schema, Vertex

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

pyo3_example.get_props(
    {"name": "Shivam Kapoor", "age": 35, "hobbies": [1, 2, 3]})
# K = name, V = String("Shivam Kapoor")
# K = age, V = Int(35)
# K = hobbies, V = Vec([1, 2, 3])


def greet(name):
    print(f"Hello {name}")


pyo3_example.invoke_passed_func(greet)
# Pometry

pyo3_example.invoke_passed_func3(greet)
# Raphtory


class A:  # Subclassing rust struct from python: Attempt 1
    def sayHello(self):
        print("Hello World")


class B(A):
    pass


b = B()
b.sayHello()


# Attempt 2
greeter = pyo3_example.Greeter("London")
greeter.say_hello()


# Attempt 3
class MyGreeter(pyo3_example.Greeter):
    def __new__(cls, name):
        return super(MyGreeter, cls).__new__(cls, name)


greeter = MyGreeter("Shivam")
greeter.say_hello()


class Point(pyo3_example.General):
    def __new__(cls, x, y):
        return super(Point, cls).__new__(cls)

    def __init__(self, x, y) -> None:
        self.x = x
        self.y = y

    def __repr__(self):
        return "".join(["Point(", str(self.x), ",", str(self.y), ")"])


print(Point(5, 4))

pyo3_example.point_serde()
pyo3_example.point_serde2(Point(3, 2))
 
s = Schema(
    Vertex(1, 1, {}),
    Vertex(2, 3, {})
)

pyo3_example.read_lotr(s)

print(s)
