import os
from concurrent.futures import ProcessPoolExecutor
from time import sleep

N = 1234567


def i():
    print(f'Initializing worker in pid={os.getpid()}')


def f(n):
    if os.path.isfile('/tmp/semaphore'):
        print('Waiting for backup to complete.')
        while os.path.isfile('/tmp/semaphore'):
            sleep(0.1)
    print(f'Handling {n} in pid={os.getpid()}')
    return sum(1 for _ in range(N)), n


def main():
    with ProcessPoolExecutor() as executor:
        for result, n in executor.map(f, range(1000)):
            if os.path.isfile('/tmp/semaphore'):
                print('Waiting for backup to complete.')
                while os.path.isfile('/tmp/semaphore'):
                    sleep(0.1)
            print(n, result)


if __name__ == '__main__':
    main()
