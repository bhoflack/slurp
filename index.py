#!/usr/bin/env python

import os
import psycopg2
import re
import sys
import uuid

c = psycopg2.connect(host='localhost', database='slurp', user='slurp', password='slurp')
cur = c.cursor()

n = 0

LOG_PATTERN = '(\d{4}-\d{2}-\d{2} \d{1,2}:\d{1,2}:\d{2},\d{3}) \| (\w+)\s*\| (.*)'

def put_line(server, l, n):
    m = re.match(LOG_PATTERN, l)
    if m:
	datetime = m.group(1)
	loglevel = m.group(2)
	msg = m.group(3)
	try:
	    cur.execute("""INSERT INTO LOGLINE (server, datetime, loglevel, message) values (%(server)s, %(time)s, %(loglevel)s, %(message)s)""",
		{'server': server,
		 'time': datetime,
		 'loglevel': loglevel,
		 'message': msg
		})
	except:
	    pass
	if n % 100 == 0:
	    sys.stdout.write('.')
	    c.commit()
	n = n + 1

for root, _, files in os.walk('.'):
    for file in files:
	with open(root + '/' + file, 'r') as f:
	    old_line = ""
	    for line in f:

		# append lines that don't start with a data to the previous line

		if re.match('\d{4}-\d{1,2}-\d{1,2}.*', line):
		    put_line(root[2:], old_line, n)
		    old_line = line
		else:
		    old_line = old_line + line

cur.close()
c.close()
