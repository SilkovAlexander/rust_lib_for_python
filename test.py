import importlib

lib = importlib.import_module('.test_lib', 'lib')
res = lib.sum_as_string(1, 2)
print(f"1 + 2 = {res}")
print("Lib works successfully")
