import os
import random
import string
import re
import pickle

from datetime import datetime

LENGTH = 17
LOWER = string.ascii_lowercase
UPPER = string.ascii_uppercase
NUM = string.digits
ALL = LOWER + UPPER + NUM
TEMP = random.sample(ALL, LENGTH)
ID = "".join(TEMP)

PATH = 'tmmp_save.pkl'

if os.path.exists(PATH):
    with open('tmmp_save.pkl', 'rb') as f:
        USERNAME, SERVER_ADDRESS, ROOM_KEY = pickle.load(f)

else:
    USERNAME = ('default_' + ID)
    SERVER_ADDRESS = ('Not set')
    ROOM_KEY = ('Not set')

RUNNING = 1

PROGRAM_INPUT = ('')
NON_SPECIAL_CHAR = re.compile(
    '[^1234567890_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM]')

while RUNNING == 1:
    now = datetime.now()
    timestamp = now.strftime("%H:%M:%S")
    PROGRAM_INPUT = input(timestamp + ' [' + USERNAME + ']: ')

    if ('/') in PROGRAM_INPUT:

        if PROGRAM_INPUT == ('/help'):
            print(
                "\n\n**List of commands**\n\n/username <USERNAME>\nSets your USERNAME (20 characters max)\n\n/sethost <SERVER_ADDRESS:ROOM_KEY>\nSets the host you're connecting to\n\n/host\nCreates a new host and sets it\n\n/save\nSaves your set USERNAME, SERVER_ADDRESS and ROOM_KEY to file\n\n/info\nShows your set USERNAME, SERVER_ADDRESS, and ROOM_KEY\n\n/quit\nExits the program\n\n"
            )
            PROGRAM_INPUT = ('')

        if PROGRAM_INPUT.find('/username ') == 0:
            unameprocess = PROGRAM_INPUT[10:30]

            if (NON_SPECIAL_CHAR.search(unameprocess) is None):
                USERNAME = PROGRAM_INPUT[10:30]
                print('\nusername set to:\n' + USERNAME + '\n')
                PROGRAM_INPUT = ('')

            else:
                print("Special Characters aren't allowed")
                PROGRAM_INPUT = ('')

        if PROGRAM_INPUT.find('/sethost ') == 0:
            host = PROGRAM_INPUT[9:591]

            if host.count(':') == 1:
                SERVER_ADDRESS, ROOM_KEY = host.split(':')
                print('\nhost set to:\n' + SERVER_ADDRESS + ':' + ROOM_KEY + '\n')
                PROGRAM_INPUT = ('')

            else:
                print('Not a valid host')
                PROGRAM_INPUT = ('')

        if PROGRAM_INPUT == ('/host'):
            print('Placeholder1')
            PROGRAM_INPUT = ('')

        if PROGRAM_INPUT == ('/save'):
            with open('tmmp_save.pkl', 'wb') as f:
                pickle.dump([USERNAME, SERVER_ADDRESS, ROOM_KEY], f)
            PROGRAM_INPUT = ('')

        if PROGRAM_INPUT == ('/info'):
            print('\n\n**Info**\n\nUSERNAME:\n' + USERNAME +
                  '\n\nSERVER_ADDRESS:\n' + SERVER_ADDRESS + '\n\nROOM_KEY:\n' +
                  ROOM_KEY + '\n\n')
            PROGRAM_INPUT = ('')

        if PROGRAM_INPUT == ('/quit'):
            RUNNING = 2
            os._exit(0)

        elif PROGRAM_INPUT.find('/') == 0:
            print("This command doesn't exist")

    else:
        print('Placeholder3')
