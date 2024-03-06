import requests
from time import sleep


class WebResource:
    """
    A web resource.
    """

    def __init__(self, uri: str, headers: str = '', cookies: str = '', body: str = '', repeats: int = 0) -> None:
        """
        Creates a web resource.

        :param uri: The URI of the resource.
        :param headers: A header JSON string to send to the resource.
        :param cookies: A cookie JSON string to send to the resource.
        :param body: A request body JSON string to send to the resource.
        :param repeats: The number of times to re-request this resource. Zero (the default) means no limit.
        """

        self.uri = uri
        self.headers = headers
        self.cookies = cookies
        self.body = body
        self.repeats = repeats

    def __request_get(self) -> requests.Response:
        """
        Simply returns the result of a requests.get operation on this resource.

        :return: The resource's response.
        """
        return requests.get(self.uri, headers=self.headers, cookies=self.cookies)

    def get(self, tries: int = 3, delay: float = 1.0, backoff: bool = False, raises: bool = True) -> requests.Response:
        """
        Get the content of the web resource.

        :param tries: Number of times to try to get the resource. Default 3. Rounds up to 1.
        :param delay: Number of seconds to wait before retries. Default 1.0. Rounds up to 0.
        :param backoff: Whether to add time between failed requests. Default false.
        :param raises: Whether to raise the last request exception. Default true.
        :return: The resource's response.
        """

        # Round tries up to 1
        if tries < 1:
            tries = 1

        # Round delay up to 1
        if delay < 0:
            delay = 0

        response = None

        while tries >= 1:
            try:
                response = self.__request_get()
                response.raise_for_status()
                break
            except (requests.exceptions.HTTPError, requests.exceptions.RequestException):
                if tries == 1 & raises:
                    raise
                sleep(delay)
            finally:
                tries -= 1
                if backoff:
                    delay *= 2

        return response

    def test(self) -> bool:
        """
        Test the resource. Makes a request, but only returns true or false depending on request success.

        :return: The resource's response status code.
        """

        try:
            response = self.__request_get()
            response.raise_for_status()
            return True
        except (requests.exceptions.HTTPError, requests.exceptions.RequestException):
            return False
