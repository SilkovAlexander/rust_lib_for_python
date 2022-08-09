import importlib
import time
import asyncio

lib = importlib.import_module('.test_lib', 'lib')
res = lib.sum_as_string(1, 2)
print(f"1 + 2 = {res}")
print("Lib works successfully")


async def func():
    loop = asyncio.new_event_loop()
    lib.init()
    start_time = time.time()
    await lib.call_sleep()
    print(time.time() - start_time)
    loop.close()

asyncio.run(func())