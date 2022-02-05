from random import choice
from sys import stdout, argv, exit
from time import sleep

def main():
    letters = 'abcdefghijklmnopqrstuvwxyz'
    len = 4

    if (argv.__len__() > 1):
        len = int(argv[1])

    def rand_ls():
        letterls = ''

        for _ in range(0,len):
            letterls = letterls + (choice(letters)) + ' '

        return letterls

    while True:
        stdout.write(f"\r{rand_ls()}\n")

        for _ in range(0,50):
            stdout.write(f"\r{rand_ls()}")
            stdout.flush()
            try:
                sleep(0.02)
            except:
                exit(0)

try:
    main()
except KeyboardInterrupt:
    exit(0)