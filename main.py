import os
import random
import string
import re

from datetime import datetime

running = 1

length = 17
lower = string.ascii_lowercase
upper = string.ascii_uppercase
num = string.digits
symbols = string.punctuation
all = lower + upper + num
temp = random.sample(all, length)
id = "".join(temp)

username = ('default_' + id)
program_input = ('')
server_address = ('Not set')
room_key = ('Not set')
non_special_char = re.compile(
  '[^1234567890_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM]')

while running == 1:
  now = datetime.now()
  timestamp = now.strftime("%H:%M:%S")
  program_input = input(timestamp + ' [' + username + ']: ')

  if ('/') in program_input:

    if program_input == ('/help'):
      print(
        "\n\n**List of commands**\n\n/username <username>\nSets your username (20 characters max)\n\n/sethost <server_address:room_key>\nSets the host you're connecting to\n\n/host\nCreates a new host and sets it\n\n/info\nShows your set username, server_address, and room_key\n\n/quit\nExits the program\n\n"
      )
      program_input = ('')

    if program_input.find('/username ') == 0:
      unameprocess = program_input[10:30]

      if (non_special_char.search(unameprocess) == None):
        username = program_input[10:30]
        print('\nusername set to:\n' + username + '\n')
        program_input = ('')

      else:
        print("Special Characters aren't allowed")
        program_input = ('')

    if program_input.find('/sethost ') == 0:
      host = program_input[9:591]

      if host.count(':') == 1:
        server_address, room_key = host.split(':')
        print('\nhost set to:\n' + server_address + ':' + room_key + '\n')
        program_input = ('')

      else:
        print('Not a valid host')
        program_input = ('')

    if program_input == ('/host'):
      print('Placeholder1')
      program_input = ('')

    if program_input == ('/info'):
      print('\n\n**Info**\n\nusername:\n' + username + '\n\nserver_address:\n' +
            server_address + '\n\nroom_key:\n' + room_key + '\n\n')
      program_input = ('')

    if program_input == ('/quit'):
      running = 2
      os._exit(0)

    elif program_input.find('/') == 0:
      print("This command doesn't exist")

  else:
    print('Placeholder3')
