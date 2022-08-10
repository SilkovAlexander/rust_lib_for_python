import importlib
import time
import asyncio

lib = importlib.import_module('.test_lib', 'lib')
res = lib.sum_as_string(1, 2)
print(f"1 + 2 = {res}")
assert res == '3'


async def func():
    interval = 2
    start_time = time.time()
    await lib.call_sleep(interval)
    pause = time.time() - start_time
    print(f"{pause=}")
    assert pause >= interval

    fut = lib.return_value()
    print(fut)
    print(asyncio.isfuture(fut))
    await asyncio.wait_for(fut, 10)
    print(fut)
    print(fut.done())
    print(fut.result())

asyncio.run(func())
print("Lib works successfully")
