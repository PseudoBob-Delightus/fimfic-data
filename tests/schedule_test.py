from fimficdata.schedulers.schedule import Schedule
import pytest


@pytest.mark.parametrize(
    'in_interval,in_time,out_sleep',
    [(1, 1641061023, 57),
     (1, 1640995199, 1),
     (5, 1641061023, 177),
     (5, 1641061267, 233),
     (5, 1641061199, 1),
     (15, 1641061023, 777),
     (15, 1641060792, 108),
     (15, 1641060899, 1),
     (30, 1641061023, 777),
     (30, 1641061858, 1742),
     (30, 1641061799, 1),
     (60, 1641061023, 2577),
     (120, 1641061023, 6177)])
def test_wait_specific_time(mocker, in_interval, in_time, out_sleep):
    mock_now = mocker.patch('fimficdata.schedulers.schedule.now', return_value=in_time)
    mock_sleep = mocker.patch('fimficdata.schedulers.schedule.sleep')

    sched = Schedule(interval=in_interval)
    sched.wait()

    mock_now.assert_called_once()
    mock_sleep.assert_called_once()

    assert mock_sleep.call_args.args == (out_sleep,)


@pytest.mark.parametrize(
    'in_interval,in_time,out_sleep',
    [(1, 1641061020, 60),
     (5, 1641061200, 300),
     (10, 1641061200, 600),
     (15, 1641061800, 900),
     (30, 1641061800, 1800),
     (60, 1641063600, 3600),
     (120, 1641067200, 7200)])
def test_wait_from_minute_start(mocker, in_interval, in_time, out_sleep):
    mock_now = mocker.patch('fimficdata.schedulers.schedule.now', return_value=in_time)
    mock_sleep = mocker.patch('fimficdata.schedulers.schedule.sleep')

    sched = Schedule(interval=in_interval)
    sched.wait()

    mock_now.assert_called_once()
    mock_sleep.assert_called_once()

    assert mock_sleep.call_args.args == (out_sleep,)


@pytest.mark.parametrize(
    'in_interval,in_time,in_buffer,out_sleep',
    [(1, 1641061020, 1, 61),
     (1, 1641061023, 2, 59),
     (1, 1640995199, 3, 4),
     (5, 1641061200, 4, 304),
     (5, 1641061023, 5, 182),
     (5, 1641061267, 10, 243),
     (5, 1641061199, 15, 16),
     (10, 1641061200, 30, 630),
     (15, 1641061800, 60, 960),
     (15, 1641061023, 120, 897),
     (15, 1641060792, 180, 288),
     (15, 1641060899, 360, 361),
     (30, 1641061800, 720, 2520),
     (30, 1641061023, 1440, 2217),
     (30, 1641061858, 1800, 3542),
     (30, 1641061799, 1200, 1201),
     (60, 1641063600, 600, 4200),
     (60, 1641061023, 300, 2877),
     (120, 1641067200, 120, 7320),
     (120, 1641061023, 60, 6237)])
def test_wait_with_buffer(mocker, in_interval, in_time, in_buffer, out_sleep):
    mock_now = mocker.patch('fimficdata.schedulers.schedule.now', return_value=in_time)
    mock_sleep = mocker.patch('fimficdata.schedulers.schedule.sleep')

    sched = Schedule(interval=in_interval, buffer=in_buffer)
    sched.wait()

    mock_now.assert_called_once()
    mock_sleep.assert_called_once()

    assert mock_sleep.call_args.args == (out_sleep,)
