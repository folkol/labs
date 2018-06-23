import asyncio

from aiostream import stream, pipe


async def get_items():
    for x in [1, 2, 3]:
        await asyncio.sleep(1)
        yield x


def do_something(x):
    print(f'Mapping {x}')
    return x


async def main():
    xs = stream.iterate(get_items()) \
         | pipe.map(do_something) \
         | pipe.list()
    print(await xs)


loop = asyncio.get_event_loop()
loop.run_until_complete(main())
loop.close()
