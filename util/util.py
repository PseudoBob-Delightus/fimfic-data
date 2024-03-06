import re
import requests
from bs4 import BeautifulSoup
import time


def get_tenth_minute(time_in: int = None):
    if time_in is None:
        time_out = int(time.time())
    else:
        time_out = time_in
    return time_out - (time_out % 600)


def last_nth_minute(n: int = 1, time_in: int = None):
    """
    Rounds the input time down to the last nth minute.

    For example, given a time_in of 1641061023 (13:17:03 EST):

    n=1 will return 1641061020 (13:17:00 EST)

    n=15 will return 1641060900 (13:15:00 EST)

    n=10 will return 1641060600 (13:10:00 EST)

    Values of n that do not fit neatly into an hour (such as 7) will result in an inconsistent time hour-to-hour.

    :param n: Rounding value for the input time. Default is 1.
    :param time_in: The input time. Default is the current time.
    :return: Timestamp of the last nth minute.
    """
    if time_in is None:
        time_out = int(time.time())
    else:
        time_out = time_in

    # This equation
    return time_out - (time_out % (n * 60))


def parse_number(number_str: str):
    # Convert from a number string to an integer
    # - String could look like '0', '153', '3,456', '1.1k', '2.3m'

    number_str = number_str.replace(',', '')
    number = float(re.search(r'([0-9.]+)', number_str).group(1))

    # multiply by 1k
    if 'k' in number_str:
        number *= 1000

    # multiply by 1m
    if 'm' in number_str:
        number *= 1000000

    # bring back to int
    return int(number)


def format_quote_strings(string: str):
    # Format string so quotes are properly escaped for SQL ingestion
    return string.replace('\"', '\"\"').replace('\'', '\'\'')
