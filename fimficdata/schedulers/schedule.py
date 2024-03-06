from fimficdata.common.util import next_nth_minute
from time import time as now, sleep


class Schedule:
    """
    A simple scheduler that will wait for the next Nth minute, plus a Buffer of seconds.

    This scheduler will not pause for N minutes + B seconds after each wait() call.
    Rather, it considers time as divided into fixed intervals of N minutes; and each
    wait() call will pause execution for the remaining time of the current interval
    plus B seconds.
    """
    def __init__(self, interval: int, buffer: int = 0) -> None:
        """
        Create a scheduler with an interval (in minutes) and a buffer (in seconds).

        :param interval: The length of the interval in minutes.
        :param buffer: The number of seconds to wait after the end of the current interval.
        """
        self.interval = interval
        self.buffer = buffer

    def wait(self) -> None:
        """
        Pause execution until the end of the current interval, plus a buffer of seconds.
        """
        cur = int(now())
        delay = abs(cur - next_nth_minute(self.interval, cur)) + self.buffer
        sleep(delay)
