"""Builds a weekly schedule from a list of timestamps.

This script attempts to re-create a weekly schedule from a list of timestamps. This can be useful if you want to
know historical peeks of events in order to make some educated guess of when to, e.g. restart servers.
"""
from datetime import datetime
import sys

datetimes = [datetime.utcfromtimestamp(int(timestamp)) for timestamp in sys.stdin]

print(*datetimes, sep='\n')

