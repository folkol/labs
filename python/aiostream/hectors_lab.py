import asyncio
from pprint import pprint

from aiostream import stream, pipe


# import chef


async def get_items():
    # await asyncio.sleep(5)
    return ['a', 'b', 'c']


async def get_item(item):
    # await asyncio.sleep(5)
    return {'jake': '123',
            'perry': '345',
            'clients': ['jake', 'perry'],
            'search_query': '*'}


async def search(query):
    # await asyncio.sleep(5)
    return ['carl', 'jake']


async def get_secret(client):
    # await asyncio.sleep 1
    return '23'


async def update_clients(item):
    print(f'Producing {item}')
    await asyncio.sleep(1)
    print(f'Produced {item}')
    return dict(item, clients=await search(item['search_query']))


async def add_keys(item):
    print(f'Adding key {item}')
    await asyncio.sleep(1)
    print(f'Added key {item}')
    return dict(item, **{c: await get_secret(c) for c in item['clients'] if c not in item})


async def main():
    xs = await (
            stream.iterate(await get_items())
            | pipe.map(get_item)
            | pipe.map(update_clients)
            | pipe.map(add_keys)
            | pipe.list()
    )
    pprint(xs, width=120)


loop = asyncio.get_event_loop()
loop.run_until_complete(main())
