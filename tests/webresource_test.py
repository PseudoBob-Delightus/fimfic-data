from fimficdata.requestors.webresource import WebResource
from requests.exceptions import HTTPError
from unittest.mock import Mock
import pytest


@pytest.mark.parametrize(
    'in_uri,out_status',
    [('https://httpstat.us/200', 200),
     ('https://httpstat.us/400', 400),
     ('https://httpstat.us/500', 500)]
)
def test_real_status_code_no_raise(in_uri, out_status):
    resource = WebResource(in_uri)
    response = resource.get(tries=1, raises=False)
    assert response.status_code == out_status


@pytest.mark.parametrize(
    'in_status,out_status',
    [(200, 200),
     (400, 400),
     (500, 500)]
)
def test_fake_status_code_no_raise(mocker, in_status, out_status):
    mock_request = mocker.patch('fimficdata.requestors.webresource.requests.get',
                                return_value=Mock(status_code=in_status))

    fake_uri = 'fake'
    resource = WebResource(fake_uri)
    response = resource.get(tries=1, raises=False)

    mock_request.assert_called_once()
    mock_request.assert_called_with(fake_uri, headers='', cookies='')

    assert response.status_code == out_status

@pytest.mark.parametrize(
    'in_status,in_error,out_status,out_error',
    [(200, False, 200, False),
     (400, True,  400, True),
     (500, True,  500, True)]
)
def test_fake_status_code_yes_raise(mocker, in_status, in_error, out_status, out_error):
    if in_error:
        mock_request = mocker.patch('fimficdata.requestors.webresource.requests.get',
                                    return_value=Mock(status_code=in_status), side_effect=HTTPError())
    else:
        mock_request = mocker.patch('fimficdata.requestors.webresource.requests.get',
                                    return_value=Mock(status_code=in_status))

    fake_uri = 'fake'
    resource = WebResource(fake_uri)
    if out_error:
        with pytest.raises(HTTPError) as e_info:
            response = resource.get(tries=1, raises=True)
    else:
        response = resource.get(tries=1, raises=True)

        mock_request.assert_called_once()
        mock_request.assert_called_with(fake_uri, headers='', cookies='')

        assert response.status_code == out_status


@pytest.mark.parametrize(
    'in_status,in_error,in_tries,out_tries,out_sleep_calls',
    [(200, False, 5, 1, 0),
     (400, True, 1, 1, 1),
     (400, True, 2, 2, 2),
     (400, True, 3, 3, 3),
     (400, True, 4, 4, 4),
     (400, True, 5, 5, 5),
     (500, True, 5, 5, 5)]
)
def test_fake_retries(mocker, in_status, in_error, in_tries, out_tries, out_sleep_calls):
    mock_sleep = mocker.patch('fimficdata.requestors.webresource.sleep')
    if in_error:
        mock_request = mocker.patch('fimficdata.requestors.webresource.requests.get',
                                    return_value=Mock(status_code=in_status), side_effect=HTTPError())
    else:
        mock_request = mocker.patch('fimficdata.requestors.webresource.requests.get',
                                    return_value=Mock(status_code=in_status))

    fake_uri = 'fake'
    resource = WebResource(fake_uri)
    resource.get(tries=in_tries, raises=False)

    assert mock_request.call_count == out_tries
    assert mock_sleep.call_count == out_sleep_calls


@pytest.mark.parametrize(
    'in_status,in_error,in_tries,out_tries,out_sleep_calls,out_sleep_len',
    [(200, False, 5, 1, 0, None),
     (400, True,  1, 1, 1, 1.0),
     (400, True,  2, 2, 2, 2.0),
     (400, True,  3, 3, 3, 4.0),
     (400, True,  4, 4, 4, 8.0),
     (400, True,  5, 5, 5, 16.0),
     (500, True,  5, 5, 5, 16.0)]
)
def test_fake_retries_backoff(mocker, in_status, in_error, in_tries, out_tries, out_sleep_calls, out_sleep_len):
    mock_sleep = mocker.patch('fimficdata.requestors.webresource.sleep')
    if in_error:
        mock_request = mocker.patch('fimficdata.requestors.webresource.requests.get',
                                    return_value=Mock(status_code=in_status), side_effect=HTTPError())
    else:
        mock_request = mocker.patch('fimficdata.requestors.webresource.requests.get',
                                    return_value=Mock(status_code=in_status))

    fake_uri = 'fake'
    resource = WebResource(fake_uri)
    resource.get(tries=in_tries, raises=False, backoff=True)

    assert mock_request.call_count == out_tries
    assert mock_sleep.call_count == out_sleep_calls
    if out_sleep_len:
        assert mock_sleep.call_args.args == (out_sleep_len,)
