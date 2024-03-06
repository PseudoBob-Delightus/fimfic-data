import re


class Author:
    def __init__(self, href, contents):
        self.id = re.search(r'/user/([0-9]+)/.*', href).group(1)
        self.name = contents[0]
