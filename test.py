import importlib
import time
import asyncio

lib = importlib.import_module('.test_lib', 'lib')
res = lib.sum_as_string(1, 2)
print(f"1 + 2 = {res}")


async def func():
    start_time = time.time()
    await lib.call_sleep(2)
    print(time.time() - start_time)

asyncio.run(func())
print("Lib works successfully")
