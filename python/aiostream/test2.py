item = {"x": "foo", "y": "bar", "z": "baz"}

clients = ["x", "y", "z", "w"]


def f(c):
    print(f'Generating secret for {c}')
    return "qux"


item.update({c: f(c) for c in clients if c not in item})
print(item)
