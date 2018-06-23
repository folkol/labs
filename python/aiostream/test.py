import asyncio


class Stream(object):
    def __or__(self, other):
        pass


async def items():
    @staticmethod
    def map(f):
        pass

    for x in [1, 2, 3]:
        await asyncio.sleep(1)
        yield x


def f(x):
    return x


xs = Stream()
ys = xs | Stream.map(f)
