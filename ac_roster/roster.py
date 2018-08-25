"""AC Raid Roster.

Installation:
    $ pip install requests

Example usage:
    $ (venv) master$ python roster.py roster.txt | expand -t '15,23'
    CHARACTER      ILVL    ALVL
    Apotekarnes    344     19
    Purplebanana   342     19
    Pufs           338     18
    ...

API Documentation and API Key:
    See https://dev.battle.net.
"""
import fileinput
import requests

BASE_URL = 'https://eu.api.battle.net/wow/character'
REALM = 'Draenor'
CHARACTER = 'Blodsfrost'
API_KEY = '...'

print('CHARACTER', 'ILVL', 'ALVL', sep='\t')
for character in fileinput.input():
    url = f'{BASE_URL}/{REALM}/{character}?fields=items&locale=en_GB&apikey={API_KEY}'
    data = requests.get(url).json()
    ilvl = data['items']['averageItemLevel']
    alvl = data['items']['neck']['azeriteItem']['azeriteLevel']
    print(character.rstrip(), ilvl, alvl, sep='\t')
