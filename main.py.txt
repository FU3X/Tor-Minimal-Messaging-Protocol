# TOR Minimal Messaging Protocol - TMMP

# To do: Make a minimal p2p messaging protocol, with e2ee, routed through TOR.

#   -Let blank messages be sent.  -Put the variables; username, server_address, and room_key, into a
# config file.

import datetime
import os
import random
import string

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

while running == 1:
  timestamp = datetime.datetime.now()
  timestampstr = str(timestamp)
  program_input = input(timestampstr + ' [' + username + ']: ')

  if ('/') in program_input:

    if program_input == ('/help'):
      print(
        "\n\nList of commands:\n\n/username <username>\nSets your username (20 characters max)\n\n/server <server_address>\nSets the server_address you're connecting to\n\n/mkserver\nHosts a new server and sets it\n\n/key <room_key>\nSets the room_key you're connecting to\n\n/mkkey\nGenerates a random room_key and sets it\n\n/info\nShows your set username, server_address, and room_key\n\n/quit\nExits the program\n\n"
      )
      program_input = ('')

    if program_input.find('/username ') == 0:
      username = program_input[10:30]
      print('\nusername set to:\n' + username + '\n')
      program_input = ('')

    if program_input.find('/server ') == 0:
      server_address = program_input[8:78]
      print('\nserver_address set to:\n' + server_address + '\n')
      program_input = ('')

    if program_input == ('/mkserver'):
      print('Placeholder1')
      program_input = ('')

    if program_input.find('/key ') == 0:
      room_key = program_input[5:517]
      print('\nroom_key set to:\n' + room_key + '\n')
      program_input = ('')

    if program_input == ('/mkkey'):
      print('Placeholder2')
      program_input = ('')

    if program_input == ('/info'):
      print('\n\nInfo:\n\nusername:\n' + username + '\n\nserver_address:\n' +
            server_address + '\n\nroom_key:\n' + room_key + '\n\n')
      program_input = ('')

    if program_input == ('/quit'):
      program_input = ('')
      running = 2
      os._exit(0)

    elif program_input.find('/') == 0:
      print("This command doesn't exist")

  else:
    print('Placeholder3')
