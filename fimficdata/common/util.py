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


def next_nth_minute(n: int = 1, time_in: int = None):
    """
    Rounds the input time up to the next nth minute.

    For example, given a time_in of 1641061023 (13:17:03 EST):

    n=1 will return # (13:18:00 EST)

    n=15 will return # (13:30:00 EST)

    n=10 will return # (13:20:00 EST)

    Values of n that do not fit neatly into an hour (such as 7) will result in an inconsistent time hour-to-hour.

    :param n: Rounding value for the input time. Default is 1.
    :param time_in: The input time. Default is the current time.
    :return: Timestamp of the next nth minute.
    """

    if type(time_in) not in [int, type(None)]:
        raise ValueError('Argument time_in must be an integer or None.')

    if type(n) != int:
        raise ValueError('Argument n must be an integer.')
    if n < 1:
        raise ValueError('Argument n must be greater than 0.')


    if time_in is None:
        time_out = int(time.time())
    else:
        time_out = time_in

    # This equation
    return time_out - (time_out % (n * 60)) + ( n * 60 )


def rel_nth_time(rel: int = 0, n: int = 1, time_type: str = 'minute', time_in: int = None):
    """
    Returns an arbitrarily rounded timestamp relative to the input time.

    e.g:
    The closest minute to now;
    The 5th most recent tenth hour before this time yesterday;
    The 67th third second occurring after 2024-01-01 00:00:00.

    If two candidates for 'closest time' are equally distant, the earlier time will be chosen.

    For example, given a time_in of 1641061023 (13:17:03 EST):

    0,1,'minute' will return # (13:17:00 EST)

    1,2,'minute' will return # (13:18:00 EST)

    2,2,'minute' will return # (13:20:00 EST)

    0,2,'hour' will return # (14:00:00 EST)

    Values of n that do not fit neatly into time periods (60, 24, 365, etc) will result in
    an inconsistent time period-to-period.

    :param rel: Relation between input and output; 0 means closest, 1 means next, -2 means second-previous. Default is 1.
    :param n: Rounding value for outputs. 0 means no rounding, 15 means round to nearest 15th time. Default is 1.
    :param time_type: Type of time, e.g. second, minute, hour. Default is 'minute'.
    :param time_in: The input time. Default is the current time.
    :return: Timestamp of the last nth minute.
    """

    #TODO: Write this, if it's ever actually necessary.


def parse_number(number_str: str):
    # Convert from a number string to an integer
    # - String could look like '0', '153', '3,456', '1.1k', '2.3m'

    number_str = number_str.replace(',', '')
    number = float(re.search(r'(-?[0-9.]+)', number_str).group(1))

    # multiply by 1k
    if 'k' in number_str:
        number *= 1000

    # multiply by 1m
    if 'm' in number_str:
        number *= 1000000

    # multiply by 1b
    if 'b' in number_str:
        number *= 1000000000

    # bring back to int
    return int(number)


def format_quote_strings(string: str):
    # Format string so quotes are properly escaped for SQL ingestion
    return string.replace('\"', '\"\"').replace('\'', '\'\'')
