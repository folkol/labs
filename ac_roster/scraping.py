"""AC Raid Roster.

Installation:
    $ pip install bs4

Example usage:
    $ (venv) master$ python scraping.py roster.txt | expand -t '15,23'
    CHARACTER      ILVL    ALVL
    Apotekarnes    343.81  19
    Purplebanana   341.63  19
    ...
"""
import fileinput

import requests
from bs4 import BeautifulSoup as BS

URL = 'https://www.wowprogress.com/guild/eu/Draenor/Ancient+Circle?roster'

roster = {}
html = requests.get(URL).text
table = BS(html, 'html.parser').find(id='char_list_table')
for row in table.find_all('tr'):
    name = row.contents[1].a.text
    ilvl = row.contents[3].text
    alvl = row.contents[4].text
    roster[name] = {
        'ilvl': ilvl,
        'alvl': alvl
    }

print('CHARACTER', 'ILVL', 'ALVL', sep='\t')
for line in fileinput.input():
    name = line.rstrip()
    character = roster.get(name)
    if character:
        print(name, character['ilvl'], character['alvl'], sep='\t')
